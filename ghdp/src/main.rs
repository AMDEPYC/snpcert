#![forbid(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

mod proxy;
mod tests;

use axum::{extract::Path, http::StatusCode, routing::get, Router};
use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use proxy::Proxy;

/// GitHub Download Proxy
///
/// Provides stable, redirect-free URLs to GitHub release asset downloads.
#[derive(Parser)]
#[command(name = "ghdp")]
#[command(about = "A GitHub Download Proxy Server")]
struct Args {
    /// GitHub repository owner
    #[arg(short = 'o', long)]
    owner: String,

    /// GitHub repository name
    #[arg(short = 'r', long)]
    repo: String,

    /// Address and port to bind to
    #[arg(short = 'b', long, default_value = "127.0.0.1:8080")]
    bind: String,

    /// Allowed domain(s) for redirects
    #[arg(short = 'd', long = "domain", action = clap::ArgAction::Append, default_values = ["githubusercontent.com"])]
    domains: Vec<String>,

    /// Maximum number of redirects to follow
    #[arg(short = 'n', long, default_value = "2")]
    redirects: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging.
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("ghdp=info".parse()?))
        .init();

    // Parse arguments.
    let args = Args::parse();
    info!("Proxying for {}/{}", args.owner, args.repo);

    // Set up the proxy.
    let proxy = Arc::new(Proxy::new(
        &args.owner,
        &args.repo,
        args.redirects,
        args.domains,
    )?);

    // Set up the router.
    let app = Router::new().route("/{release}/{file}", {
        let proxy = proxy.clone();
        get(
            move |Path((release, file)): Path<(String, String)>| async move {
                proxy.get(&release, &file).await.map_err(|error| {
                    info!("❌ Proxy error: {}", error);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                })
            },
        )
    });

    let listener = TcpListener::bind(&args.bind).await?;

    info!("Server listening on {}", args.bind);
    axum::serve(listener, app).await?;
    Ok(())
}
