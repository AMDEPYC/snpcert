use super::browsing::Service;
use super::types::{FoundArgs, FoundStream};

use std::collections::HashMap;
use std::net::{AddrParseError, IpAddr, SocketAddr};

use tokio_stream::StreamExt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Resolved {
    pub service: Service,
    pub host: String,
    pub address: SocketAddr,
    pub txt: HashMap<String, String>,
}

impl TryFrom<FoundArgs<'_>> for Resolved {
    type Error = AddrParseError;

    fn try_from(args: FoundArgs<'_>) -> Result<Self, Self::Error> {
        let mut txt = HashMap::new();

        // Parse TXT records
        for entry in args.txt() {
            if let Ok(s) = core::str::from_utf8(entry) {
                if let Some((k, v)) = s.split_once('=') {
                    txt.insert(k.to_string(), v.to_string());
                }
            }
        }

        // Parse the socket
        let address: IpAddr = args.address().parse()?;
        let address: SocketAddr = (address, *args.port()).into();

        Ok(Resolved {
            host: args.host().to_string(),
            service: args.into(),
            address,
            txt,
        })
    }
}

pub struct Resolving(pub(super) FoundStream);

impl Resolving {
    pub async fn next(&mut self) -> Option<Resolved> {
        let signal = self.0.next().await?;
        let args = signal.args().ok()?;
        args.try_into().ok()
    }
}
