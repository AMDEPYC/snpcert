pub mod avahi;
pub mod uefi;

use std::net::IpAddr;
use std::time::Duration;

use clap::{Parser, Subcommand};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use zbus::Connection;

use crate::avahi::Avahi;

#[derive(Parser)]
#[command(name = std::env!("CARGO_PKG_NAME"))]
#[command(about = std::env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Alerts dispatch that a workload is booting.
    Boot,

    /// Asks dispatch to create a GitHub issue with test results.
    Report {
        /// The title of the GitHub issue
        #[arg(short, long)]
        title: String,

        /// Text file to use as the GitHub issue body
        ///
        /// If not specified, the body will be read from stdin.
        #[arg(short, long, value_name = "FILE")]
        body: Option<std::path::PathBuf>,

        /// Labels for the GitHub issue (can be specified multiple times)
        #[arg(short, long, action = clap::ArgAction::Append)]
        label: Vec<String>,

        /// Assignees for the GitHub issue (can be specified multiple times)
        #[arg(short, long, action = clap::ArgAction::Append)]
        assignee: Vec<String>,
    },
}

#[derive(Debug, Clone)]
enum Action {
    Boot,
    Report(Report),
}

impl TryFrom<Cli> for Action {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        match value.command {
            Commands::Boot => Ok(Action::Boot),
            Commands::Report {
                title,
                body,
                label,
                assignee,
            } => Ok(Action::Report(Report {
                title,
                body: body
                    .map(std::fs::read_to_string)
                    .unwrap_or_else(|| std::io::read_to_string(std::io::stdin().lock()))?,
                labels: label,
                assignees: assignee,
            })),
        }
    }
}

impl Action {
    async fn perform(&self, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Send the appropriate request based on action
        let response = match self {
            Action::Boot => Client::new().post(url).send().await?,
            Action::Report(report) => Client::new().put(url).json(report).send().await?,
        };

        // Handle the response
        match response.status() {
            // This is the normal error when the service worked,
            // but no task was found for our IP address. This either
            // means that there is no job or that we need to contact
            // the server on a different address. Skip.
            StatusCode::EXPECTATION_FAILED => Ok(false),
            StatusCode::OK => Ok(true),
            status => {
                eprintln!("warning: {status}");
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    title: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    body: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    labels: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    assignees: Vec<String>,
}

const RESOLVER_TIMEOUT: Duration = Duration::from_secs(5);
const BROWSER_TIMEOUT: Duration = Duration::from_secs(10);

// Avahi D-Bus proxy interfaces
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action: Action = Cli::parse().try_into()?;

    for url in uefi::find_urls().await? {
        match action.perform(&url).await {
            Ok(true) => return Ok(()),
            Ok(false) => continue,
            Err(e) => eprintln!("error: {}: {}", url, e),
        }
    }

    let connection = Connection::system().await?;
    let avahi = Avahi::new(&connection).await?;

    let mut browsing = avahi.browse(-1, -1, "_dispatch._tcp", "local", 0).await?;
    while let Ok(Some(item)) = timeout(BROWSER_TIMEOUT, browsing.next()).await {
        let mut resolving = avahi.resolve(item).await?;

        let action = action.clone();
        tokio::spawn(async move {
            while let Ok(Some(resolved)) = timeout(RESOLVER_TIMEOUT, resolving.next()).await {
                // Ignore link-local addresses. While these are theoretically
                // possible, they are unlikely. Supporting them means we have
                // to resolve an interface since naked link-local addresses
                // aren't permitted without an interface qualifier.
                match resolved.address.ip() {
                    addr if addr.is_loopback() => continue,
                    IpAddr::V4(ipv4) if ipv4.is_link_local() => continue,
                    IpAddr::V6(ipv6) if ipv6.is_unicast_link_local() => continue,
                    _ => {}
                }

                // Construct the URL
                let url = match resolved.txt.get("path") {
                    Some(path) => format!("http://{}{}", resolved.address, path),
                    None => continue,
                };

                match action.perform(&url).await {
                    Ok(true) => std::process::exit(0),
                    Ok(false) => continue,
                    Err(e) => eprintln!("error: {}: {}", url, e),
                }
            }
        });
    }

    Err("no dispatch services found".into())
}
