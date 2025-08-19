use super::types::{FoundArgs, ItemNewArgs, ItemNewStream};

use tokio_stream::StreamExt;

#[derive(Debug, Clone)]
pub struct Service {
    pub interface: i32,
    pub protocol: i32,
    pub name: String,
    pub service_type: String,
    pub domain: String,
    pub flags: u32,
}

impl From<FoundArgs<'_>> for Service {
    fn from(args: FoundArgs<'_>) -> Self {
        Self {
            interface: *args.interface(),
            protocol: *args.protocol(),
            name: args.name().to_string(),
            service_type: args.service_type().to_string(),
            domain: args.domain().to_string(),
            flags: *args.flags(),
        }
    }
}

impl From<ItemNewArgs<'_>> for Service {
    fn from(args: ItemNewArgs<'_>) -> Self {
        Self {
            interface: *args.interface(),
            protocol: *args.protocol(),
            name: args.name().to_string(),
            service_type: args.service_type().to_string(),
            domain: args.domain().to_string(),
            flags: *args.flags(),
        }
    }
}

pub struct Browsing(pub(super) ItemNewStream);

impl Browsing {
    pub async fn next(&mut self) -> Option<Service> {
        let signal = self.0.next().await?;
        let args = signal.args().ok()?;
        Some(args.into())
    }
}
