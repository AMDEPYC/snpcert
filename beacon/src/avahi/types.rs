#[zbus::proxy(
    interface = "org.freedesktop.Avahi.Server",
    default_service = "org.freedesktop.Avahi",
    default_path = "/"
)]
pub trait Server {
    /// Create a new service browser
    #[allow(clippy::too_many_arguments)]
    fn service_browser_new(
        &self,
        interface: i32,
        protocol: i32,
        service_type: &str,
        domain: &str,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Create a new service resolver  
    #[allow(clippy::too_many_arguments)]
    fn service_resolver_new(
        &self,
        interface: i32,
        protocol: i32,
        name: &str,
        service_type: &str,
        domain: &str,
        aprotocol: i32,
        flags: u32,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

/// Service browser proxy for receiving service discovery notifications
#[zbus::proxy(
    interface = "org.freedesktop.Avahi.ServiceBrowser",
    default_service = "org.freedesktop.Avahi"
)]
pub trait ServiceBrowser {
    /// Signal emitted when a new service is discovered
    #[zbus(signal)]
    fn item_new(
        &self,
        interface: i32,
        protocol: i32,
        name: &str,
        service_type: &str,
        domain: &str,
        flags: u32,
    ) -> zbus::Result<()>;
}

/// Service resolver proxy for resolving service details
#[zbus::proxy(
    interface = "org.freedesktop.Avahi.ServiceResolver",
    default_service = "org.freedesktop.Avahi"
)]
pub trait ServiceResolver {
    /// Signal emitted when service details are successfully resolved
    #[zbus(signal)]
    fn found(
        &self,
        interface: i32,
        protocol: i32,
        name: &str,
        service_type: &str,
        domain: &str,
        host: &str,
        aprotocol: i32,
        address: &str,
        port: u16,
        txt: Vec<Vec<u8>>,
        flags: u32,
    ) -> zbus::Result<()>;

    /// Signal emitted when service resolution fails
    #[zbus(signal)]
    fn failure(&self, error: &str) -> zbus::Result<()>;
}
