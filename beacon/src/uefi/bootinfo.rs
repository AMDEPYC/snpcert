use devpath::{Error, FromBytes, Paths};

/// Boot entry information from UEFI Boot#### variables
#[derive(Debug)]
#[allow(dead_code)]
pub struct BootInfo {
    /// Boot entry attributes
    pub attributes: u32,

    /// Human-readable description
    pub description: String,

    /// Device paths for boot files
    pub filepaths: Paths,

    /// Optional data
    pub data: Vec<u8>,
}

impl<'a> FromBytes<'a> for BootInfo {
    type Error = Error;

    fn from_bytes(mut bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse the attributes.
        let (head, tail) = bytes.split_at_checked(4).ok_or(Error::Invalid)?;
        let head: &[u8; 4] = head.try_into().map_err(|_| Error::Invalid)?;
        let attributes = u32::from_le_bytes(*head);
        bytes = tail;

        // Parse the file paths length.
        let (len_bytes, tail) = bytes.split_at_checked(2).ok_or(Error::Invalid)?;
        let len: &[u8; 2] = len_bytes.try_into().map_err(|_| Error::Invalid)?;
        let len = u16::from_le_bytes(*len);
        bytes = tail;

        // Find the description null terminator.
        let null = bytes
            .chunks_exact(2)
            .position(|chunk| chunk == [0, 0])
            .ok_or(Error::Invalid)?;

        // Get the whole string, including the terminator.
        let (head, tail) = bytes.split_at_checked(null * 2 + 2).ok_or(Error::Invalid)?;
        bytes = tail;

        // Convert the bytes to a Vec<u16>
        let shorts: Vec<u16> = head
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        // Validate the description string.
        let descr = String::from_utf16(&shorts[..shorts.len() - 1]).map_err(|_| Error::Invalid)?;

        // Split the filepaths from the extra data.
        let (head, tail) = bytes.split_at_checked(len as usize).ok_or(Error::Invalid)?;

        Ok(Self {
            attributes,
            description: descr,
            filepaths: Paths::from_bytes(head)?,
            data: tail.to_vec(),
        })
    }
}
