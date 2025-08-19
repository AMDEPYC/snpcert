use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use futures_util::{stream, StreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::body::{Bytes, Frame, Incoming};
use hyper::{Method, StatusCode as Code};
use hyper::{Request, Response};
use reqwest::Client;
use std::convert::Infallible;
use tokio::sync::Mutex;

use crate::github::{Asset, GitHub, Report};
use crate::tui::Status;

// This contains the bytes of the poweroff.efi module.
const POWEROFF_EFI: &[u8] = include_bytes!(env!("POWEROFF_BIN_PATH"));
const EMPTY: &[u8] = &[];

// The content type for EFI binaries.
const CT_EFI: &str = "application/efi";

/// Main HTTP service that handles all requests
pub struct Service {
    remote: IpAddr,
    status: Arc<Mutex<Status>>,
    github: Arc<GitHub>,
    client: Client,
    path: Arc<String>,
}

impl Service {
    pub const fn new(
        remote: IpAddr,
        status: Arc<Mutex<Status>>,
        github: Arc<GitHub>,
        client: Client,
        path: Arc<String>,
    ) -> Self {
        Self {
            remote,
            status,
            github,
            client,
            path,
        }
    }
}

impl hyper::service::Service<Request<Incoming>> for Service {
    type Response = Response<BoxBody<Bytes, Infallible>>;
    type Error = anyhow::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let status = self.status.clone();
        let client = self.client.clone();
        let github = self.github.clone();
        let path = self.path.clone();
        let remote = self.remote;

        Box::pin(async move {
            if req.uri().path() != *path {
                return Ok(EMPTY.reply(Code::NOT_FOUND, None, None));
            }

            let response = match *req.method() {
                // The POST request is used to signal the start of a job.
                Method::POST => {
                    if status.lock().await.update().booting(remote) {
                        return Ok(EMPTY.reply(None, None, None));
                    }

                    return Ok(EMPTY.reply(Code::EXPECTATION_FAILED, None, None));
                }

                // The PUT request is used to report completion of a job.
                Method::PUT => {
                    // Collect the request body
                    let bytes = req.into_body().collect().await?.to_bytes();
                    let report: Report = match serde_json::from_slice(&bytes) {
                        Err(..) => return Ok(EMPTY.reply(Code::BAD_REQUEST, None, None)),
                        Ok(report) => report,
                    };

                    // Display that the job has been reported.
                    if !status.lock().await.update().report(remote) {
                        return Ok(EMPTY.reply(Code::EXPECTATION_FAILED, None, None));
                    }

                    // Create a GitHub issue for the report.
                    let reported = tokio::time::Instant::now();
                    if github.report(report).await.is_err() {
                        return Ok(EMPTY.reply(Code::INTERNAL_SERVER_ERROR, None, None));
                    }

                    // Mark the job as finished.
                    tokio::spawn(async move {
                        tokio::time::sleep_until(reported + Duration::from_secs(5)).await;
                        status.lock().await.update().finish(remote);
                    });

                    return Ok(EMPTY.reply(None, None, None));
                }

                // The HEAD request is used to get information about the assigned asset.
                Method::HEAD => {
                    match status.clone().assign(remote).await {
                        // No asset assigned, return poweroff EFI binary.
                        None => return Ok(POWEROFF_EFI.reply(None, CT_EFI, EMPTY)),

                        // Send the request (possibly redirecting...)
                        Some(asset) => client.head(asset.url).send().await?,
                    }
                }

                // The GET request is used to fetch the assigned asset.
                Method::GET => {
                    match status.clone().assign(remote).await {
                        // No asset assigned, return poweroff EFI binary.
                        None => return Ok(POWEROFF_EFI.reply(None, CT_EFI, None)),

                        // Send the request (possibly redirecting...)
                        Some(asset) => {
                            let response = client.get(asset.url).send().await?;
                            status.lock().await.update().downloading(remote);
                            response
                        }
                    }
                }

                // Bad method.
                _ => {
                    return Ok(Response::builder()
                        .status(Code::METHOD_NOT_ALLOWED)
                        .header("allow", "GET, POST, HEAD, PUT")
                        .body(EMPTY.embody())?)
                }
            };

            let ct_efi = CT_EFI.parse().unwrap();

            // Construct the response.
            let mut builder = Response::builder().status(response.status());
            for (key, mut value) in response.headers() {
                // GitHub always returns `application/octet-stream` for EFI
                // binaries, so we override it here.
                if key == "content-type" && value == "application/octet-stream" {
                    value = &ct_efi;
                }

                builder = builder.header(key, value);
            }

            // Stream the response body directly, mapping errors to Infallible
            Ok(builder.body(BoxBody::new(StreamBody::new(Box::pin(
                response.bytes_stream().map(|result| {
                    result.map_or_else(
                        |_| Ok(Frame::data(Bytes::new())),
                        |bytes| Ok(Frame::data(bytes)),
                    )
                }),
            ))))?)
        })
    }
}

trait Embody {
    fn embody(self) -> BoxBody<Bytes, Infallible>;
}

impl Embody for &'static [u8] {
    fn embody(self) -> BoxBody<Bytes, Infallible> {
        BoxBody::new(StreamBody::new(Box::pin(stream::once(async move {
            Ok(Frame::data(Bytes::from(self)))
        }))))
    }
}

trait Reply {
    fn reply(
        self,
        code: impl Into<Option<Code>>,
        ct: impl Into<Option<&'static str>>,
        body: impl Into<Option<&'static [u8]>>,
    ) -> Response<BoxBody<Bytes, Infallible>>;
}

impl Reply for &'static [u8] {
    fn reply(
        self,
        code: impl Into<Option<Code>>,
        ct: impl Into<Option<&'static str>>,
        body: impl Into<Option<&'static [u8]>>,
    ) -> Response<BoxBody<Bytes, Infallible>> {
        let mut builder = Response::builder()
            .status(code.into().unwrap_or(Code::OK))
            .header("content-length", self.len());

        if let Some(ct) = ct.into() {
            builder = builder.header("content-type", ct);
        }

        builder.body(body.into().unwrap_or(self).embody()).unwrap()
    }
}

trait Assign {
    async fn assign(self, ip: IpAddr) -> Option<Asset>;
}

impl Assign for Arc<Mutex<Status>> {
    async fn assign(self, ip: IpAddr) -> Option<Asset> {
        self.lock().await.update().assign(ip)
    }
}
