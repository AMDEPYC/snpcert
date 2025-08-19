use axum::body::Body;
use axum::response::Response;
use reqwest::{redirect::Policy, Client};
use tracing::{debug, info};
use url::Url;

use std::error::Error;

#[derive(Clone)]
pub struct Proxy {
    client: Client,
    base: Url,
}

impl Proxy {
    pub fn new(
        owner: &str,
        repo: &str,
        redirects: usize,
        domains: Vec<String>,
    ) -> Result<Self, Box<dyn Error>> {
        let policy = Policy::custom(move |attempt| {
            if attempt.previous().len() > redirects {
                return attempt.stop();
            }

            let Some(host) = attempt.url().host_str() else {
                return attempt.stop();
            };

            for domain in &*domains {
                if let Some(prefix) = host.strip_suffix(domain) {
                    if prefix.is_empty() || prefix.ends_with('.') {
                        return attempt.follow();
                    }
                }
            }

            attempt.stop()
        });

        let mut base = Url::parse("https://github.com/")?;
        base.path_segments_mut()
            .map_err(|()| "unable to construct path segments")?
            .push(owner)
            .push(repo)
            .push("releases")
            .push("download")
            .push(""); // This creates the trailing slash

        Ok(Self {
            client: Client::builder().redirect(policy).build()?,
            base,
        })
    }

    pub async fn get(&self, release: &str, file: &str) -> Result<Response, Box<dyn Error>> {
        info!("📥 Requested: /{}/{}", release, file);

        // Construct the URL
        let mut url = self.base.clone();
        url.path_segments_mut()
            .map_err(|()| "unable to construct path segments")?
            .push(release)
            .push(file);
        debug!("🌐 Fetching: {}", url);

        // Send the request (possibly redirecting...)
        let resp = self.client.get(url).send().await?;

        // Construct the response.
        let mut builder = Response::builder().status(resp.status());
        for (key, value) in resp.headers() {
            builder = builder.header(key, value);
        }

        // Convert reqwest body to axum Body.
        let body = Body::from_stream(resp.bytes_stream());
        Ok(builder.body(body)?)
    }
}
