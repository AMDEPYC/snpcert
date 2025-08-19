use std::collections::BTreeSet;
use std::process::Command;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Asset {
    pub name: String,
    pub size: u64,

    #[serde(rename = "browser_download_url")]
    pub url: String,

    pub content_type: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    assignees: Option<Vec<String>>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct GitHub {
    /// GitHub token for API access
    #[arg(long, env = "GITHUB_TOKEN")]
    pub token: Option<String>,

    /// GitHub repository owner
    #[arg(short = 'o', long)]
    pub owner: String,

    /// GitHub repository name  
    #[arg(short = 'r', long)]
    pub repo: String,

    /// Release tag to download assets from
    #[arg(short = 't', long)]
    pub tag: String,

    /// Filter asset names
    #[arg(trailing_var_arg = true)]
    pub filter: Vec<String>,
}

impl GitHub {
    const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    pub const EFI_TYPE: &str = "application/vnd.microsoft.portable-executable";

    /// Authenticate with GitHub by guiding user to create a Personal Access Token
    pub fn login(&mut self) -> Result<()> {
        // Try to get token from GitHub CLI
        if self.token.is_none() {
            self.token = Command::new("gh")
                .arg("auth")
                .arg("token")
                .output()
                .ok()
                .and_then(|output| {
                    if !output.status.success() {
                        return None;
                    }

                    String::from_utf8(output.stdout)
                        .ok()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                });
        }

        // If we already have a token, nothing to do
        if self.token.is_some() {
            return Ok(());
        }

        println!("No GitHub token found. Please authenticate with GitHub.");
        println!();
        println!("Option 1 - Use GitHub CLI (recommended):");
        println!("  gh auth login");
        println!("  # Re-run this command");
        println!();
        println!("Option 2 - Create a Personal Access Token manually:");
        println!("  1. Visit: https://github.com/settings/tokens/new");
        println!("  2. Select scopes (permissions needed):");
        println!("     • Repository access: Select your target repository");
        println!("     • Repository permissions:");
        println!("       - Contents: Read (for downloading release assets)");
        println!("       - Issues: Write (for creating issues)");
        println!("  3. Click 'Generate token'");
        println!();
        println!("  export GITHUB_TOKEN=<YOUR_TOKEN>");
        println!("  # Re-run this command");
        println!();

        anyhow::bail!("GitHub authentication required. Please follow the instructions above.");
    }

    fn client(&self) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        headers.insert("User-Agent", Self::USER_AGENT.parse().unwrap());

        if let Some(token) = &self.token {
            let auth_value = format!("Bearer {token}");
            headers.insert("Authorization", auth_value.parse().unwrap());
        }

        Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client")
    }

    pub async fn assets(&self) -> Result<BTreeSet<Asset>> {
        let client = self.client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/tags/{}",
            self.owner, self.repo, self.tag
        );

        let response = client.get(&url).send().await?;
        let release: Release = response.json().await?;

        let assets = release
            .assets
            .into_iter()
            .filter(|asset| asset.content_type == Self::EFI_TYPE)
            .filter(|asset| {
                self.filter.is_empty() || self.filter.iter().any(|f| asset.name.contains(f))
            })
            .collect::<BTreeSet<_>>();

        Ok(assets)
    }

    pub async fn report(&self, report: Report) -> Result<()> {
        let client = self.client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.owner, self.repo
        );

        client.post(&url).json(&report).send().await?;
        Ok(())
    }
}
