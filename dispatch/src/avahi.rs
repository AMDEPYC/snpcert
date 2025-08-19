use anyhow::{Context, Result};
use zbus::zvariant::OwnedObjectPath;
use zbus::Connection;

#[zbus::proxy(
    interface = "org.freedesktop.Avahi.Server",
    default_service = "org.freedesktop.Avahi",
    default_path = "/"
)]
trait AvahiServer {
    /// Create a new entry group
    fn entry_group_new(&self) -> zbus::Result<OwnedObjectPath>;
}

#[zbus::proxy(
    interface = "org.freedesktop.Avahi.EntryGroup",
    default_service = "org.freedesktop.Avahi"
)]
trait AvahiEntryGroup {
    /// Add a service to the entry group
    #[allow(clippy::too_many_arguments)]
    fn add_service(
        &self,
        interface: i32,
        protocol: i32,
        flags: u32,
        name: &str,
        service_type: &str,
        domain: &str,
        host: &str,
        port: u16,
        txt: Vec<Vec<u8>>,
    ) -> zbus::Result<()>;

    /// Commit the entry group
    fn commit(&self) -> zbus::Result<()>;
}

/// Avahi service registration wrapper
pub struct AvahiService {
    entry_group: AvahiEntryGroupProxy<'static>,
}

impl AvahiService {
    /// Create a new Avahi service
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .context("Failed to connect to system bus")?;

        let server = AvahiServerProxy::new(&connection)
            .await
            .context("Failed to create Avahi server proxy")?;

        let entry_group_path = server
            .entry_group_new()
            .await
            .context("Failed to create entry group")?;

        let entry_group = AvahiEntryGroupProxy::builder(&connection)
            .path(entry_group_path)
            .context("Invalid entry group path")?
            .build()
            .await
            .context("Failed to create entry group proxy")?;

        Ok(Self { entry_group })
    }

    /// Register an HTTP service
    pub async fn register(&self, name: &str, port: u16, txt: &[(&str, &str)]) -> Result<()> {
        let service_type = format!("_{name}._tcp");

        // Convert TXT records to the format expected by Avahi
        let txt: Vec<Vec<u8>> = txt
            .iter()
            .map(|(key, val)| format!("{key}={val}").into_bytes())
            .collect();

        // Register the service
        // interface: -1 (AVAHI_IF_UNSPEC)
        // protocol: 0 (AVAHI_PROTO_INET - IPv4)
        // flags: 0 (no special flags)
        // domain: "" (use default)
        // host: "" (use local hostname)
        self.entry_group
            .add_service(-1, 0, 0, name, &service_type, "", "", port, txt)
            .await
            .context("Failed to add service")?;

        self.entry_group
            .commit()
            .await
            .context("Failed to commit entry group")?;

        Ok(())
    }
}
