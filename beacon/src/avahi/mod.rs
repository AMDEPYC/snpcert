pub mod browsing;
pub mod resolving;

mod types;

use zbus::Connection;

#[derive(Clone)]
pub struct Avahi<'a>(&'a Connection, types::ServerProxy<'a>);

impl<'a> Avahi<'a> {
    pub async fn new(connection: &'a Connection) -> Result<Self, zbus::Error> {
        Ok(Self(connection, types::ServerProxy::new(connection).await?))
    }

    pub async fn browse(
        &self,
        interface: i32,
        protocol: i32,
        service_type: &'static str,
        domain: &'static str,
        flags: u32,
    ) -> Result<browsing::Browsing, zbus::Error> {
        let path = self
            .1
            .service_browser_new(interface, protocol, service_type, domain, flags)
            .await?;

        let proxy = types::ServiceBrowserProxy::builder(self.0)
            .path(path)?
            .build()
            .await?;

        Ok(browsing::Browsing(proxy.receive_item_new().await?))
    }

    pub async fn resolve(
        &self,
        service: browsing::Service,
    ) -> Result<resolving::Resolving, zbus::Error> {
        let path = self
            .1
            .service_resolver_new(
                service.interface,
                service.protocol,
                &service.name,
                &service.service_type,
                &service.domain,
                service.protocol,
                0,
            )
            .await?;

        let proxy = types::ServiceResolverProxy::builder(self.0)
            .path(path)?
            .build()
            .await?;

        Ok(resolving::Resolving(proxy.receive_found().await?))
    }
}
