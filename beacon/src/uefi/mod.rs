use devpath::msg::Messaging as Msg;
use devpath::{FromBytes, Node};

mod bootinfo;
mod efivar;

const EFI_VARS_PATH: &str = "/sys/firmware/efi/efivars";
const BOOT_VAR_PREFIX: &str = "Boot";
const BOOT_VAR_SUFFIX: &str = "-8be4df61-93ca-11d2-aa0d-00e098032b8c";

pub async fn find_urls() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut urls = Vec::new();

    // Find all Boot#### variables
    let mut entries = tokio::fs::read_dir(EFI_VARS_PATH).await?;
    'outer: while let Some(entry) = entries.next_entry().await? {
        let filename = entry.file_name();
        let filename = filename.to_string_lossy();

        // Make sure it starts with Boot.
        if !filename.starts_with(BOOT_VAR_PREFIX) {
            continue;
        }

        // Make sure it ends with the GUID.
        if !filename.ends_with(BOOT_VAR_SUFFIX) {
            continue;
        }

        // Make sure the middle is hexdigits.
        let middle = filename.trim_end_matches(BOOT_VAR_SUFFIX);
        let middle = middle.trim_start_matches(BOOT_VAR_PREFIX);
        for c in middle.chars() {
            if !c.is_ascii_hexdigit() {
                continue 'outer;
            }
        }

        let data = tokio::fs::read(entry.path()).await?;
        let var = efivar::EfiVar::from_bytes(&data)?;
        let bi = bootinfo::BootInfo::from_bytes(var.data)?;

        'inner: for path in bi.filepaths.iter() {
            let mut mac = false;
            let mut url = None;

            // A Uri should be last. It must have MAC earlier.
            for node in path.iter() {
                match node {
                    _ if url.is_some() => continue 'inner,
                    Node::Messaging(Msg::MacAddress(_)) if !mac => mac = true,
                    Node::Messaging(Msg::Uri(uri)) if mac => url = Some(uri.0.clone()),
                    _ => {}
                }
            }

            if let Some(url) = url {
                urls.push(url.clone());
            }
        }
    }

    Ok(urls)
}
