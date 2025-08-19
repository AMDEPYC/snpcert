use alloc::string::String;

pub struct Invalid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ByteOrder {
    #[default]
    Little,
    Big,
}

pub trait Length {
    fn len(&self) -> usize;
}

impl Length for &[u8] {
    fn len(&self) -> usize {
        (*self).len()
    }
}

pub trait Parser<T>: Length {
    type Arg;

    fn parse(&mut self, arg: Self::Arg) -> Result<T, Invalid>;

    fn finish(&mut self, arg: Self::Arg) -> Result<T, Invalid> {
        let output = self.parse(arg)?;

        if self.len() != 0 {
            return Err(Invalid);
        }

        Ok(output)
    }
}

impl Parser<Vec<u8>> for &[u8] {
    type Arg = usize;

    fn parse(&mut self, len: usize) -> Result<Vec<u8>, Invalid> {
        let (head, tail) = self.split_at_checked(len).ok_or(Invalid)?;
        *self = tail;
        Ok(head.to_vec())
    }
}

impl<const N: usize> Parser<[u8; N]> for &[u8] {
    type Arg = ();

    fn parse(&mut self, _arg: ()) -> Result<[u8; N], Invalid> {
        let (head, tail) = self.split_at_checked(N).ok_or(Invalid)?;
        *self = tail;

        let array: &[u8; N] = head.try_into().map_err(|_| Invalid)?;
        Ok(*array)
    }
}

impl Parser<u8> for &[u8] {
    type Arg = ();

    fn parse(&mut self, _arg: ()) -> Result<u8, Invalid> {
        let bytes: [u8; 1] = self.parse(())?;
        Ok(bytes[0])
    }
}

impl Parser<u16> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: ByteOrder) -> Result<u16, Invalid> {
        Ok(match arg {
            ByteOrder::Little => u16::from_le_bytes(self.parse(())?),
            ByteOrder::Big => u16::from_be_bytes(self.parse(())?),
        })
    }
}

impl Parser<u32> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: ByteOrder) -> Result<u32, Invalid> {
        Ok(match arg {
            ByteOrder::Little => u32::from_le_bytes(self.parse(())?),
            ByteOrder::Big => u32::from_be_bytes(self.parse(())?),
        })
    }
}

impl Parser<u64> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: ByteOrder) -> Result<u64, Invalid> {
        Ok(match arg {
            ByteOrder::Little => u64::from_le_bytes(self.parse(())?),
            ByteOrder::Big => u64::from_be_bytes(self.parse(())?),
        })
    }
}

impl Parser<u128> for &[u8] {
    type Arg = ByteOrder;

    fn parse(&mut self, arg: ByteOrder) -> Result<u128, Invalid> {
        Ok(match arg {
            ByteOrder::Little => u128::from_le_bytes(self.parse(())?),
            ByteOrder::Big => u128::from_be_bytes(self.parse(())?),
        })
    }
}

pub enum Format {
    Utf16(Option<usize>),
    Utf8(Option<usize>),
}

// UTF-16 string parser that converts to UTF-8 String
impl Parser<String> for &[u8] {
    type Arg = Format;

    fn parse(&mut self, format: Format) -> Result<String, Invalid> {
        match format {
            Format::Utf16(Some(len)) if !len.is_multiple_of(2) => Err(Invalid),

            // Use the given length.
            Format::Utf16(Some(len)) => {
                let mut chars = alloc::vec::Vec::with_capacity(len);

                for _ in 0..len / 2 {
                    chars.push(self.parse(ByteOrder::Little)?);
                }

                String::from_utf16(&chars).map_err(|_| Invalid)
            }

            // Find the null terminator.
            Format::Utf16(None) => {
                let mut chars = alloc::vec::Vec::new();

                loop {
                    let c = self.parse(ByteOrder::Little)?;
                    if c == 0 {
                        break;
                    }

                    chars.push(c);
                }

                String::from_utf16(&chars).map_err(|_| Invalid)
            }

            // Use the given length.
            Format::Utf8(Some(len)) => {
                let mut bytes: Vec<u8> = self.parse(len)?;

                // Remove the null terminator if present.
                // The spec implies URI is not null-terminated,
                // but some implementations terminate it.
                if len > 0 && bytes[len - 1] == 0 {
                    bytes.truncate(len - 1);
                }

                String::from_utf8(bytes).map_err(|_| Invalid)
            }

            // Find the null terminator.
            Format::Utf8(None) => {
                let len = self.iter().position(|&b| b == 0).ok_or(Invalid)?;
                let string = String::from_utf8(self.parse(len)?).map_err(|_| Invalid)?;
                let _: u8 = self.parse(())?; // Discard the null-terminator.
                Ok(string)
            }
        }
    }
}

impl Parser<bool> for &[u8] {
    type Arg = ();

    fn parse(&mut self, _arg: ()) -> Result<bool, Invalid> {
        let byte: u8 = self.parse(())?;
        Ok(byte != 0)
    }
}
