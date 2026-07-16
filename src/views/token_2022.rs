 use crate::{errors::KontError, views::token_v1::TokenAccountView};

pub struct Token2022AccountView<'a> {
    data: &'a [u8],
}

impl<'a> Token2022AccountView<'a> {
    pub fn try_from_slice(data: &'a [u8]) -> Result<Self, KontError> {
        // Must be at least 165 bytes (base SPL token structure)
        if data.len() < 165 {
            return Err(KontError::InvalidBufferLength);
        }
        Ok(Self { data })
    }

    pub fn base(&self) -> Result<TokenAccountView<'a>, KontError> {
        // TokenAccountView parses exactly the first 165 bytes
        TokenAccountView::try_from_slice(&self.data[0..165])
    }

    pub fn get_extension(&self, extension_type: u16) -> Result<Option<&'a [u8]>, KontError> {
        // If the account is exactly 165 bytes, there are no extensions
        if self.data.len() <= 165 {
            return Ok(None);
        }

        // Byte 165 is the AccountType byte. 
        // Index 166 is where the TLV data block actually begins.
        let mut offset = 166;

        while offset + 4 <= self.data.len() {
            // 1. Read the 2-byte Type (T)
            let mut type_bytes = [0u8; 2];
            type_bytes.copy_from_slice(&self.data[offset..offset + 2]);
            let t = u16::from_le_bytes(type_bytes);

            // 2. Read the 2-byte Length (L)
            let mut len_bytes = [0u8; 2];
            len_bytes.copy_from_slice(&self.data[offset + 2..offset + 4]);
            let l = u16::from_le_bytes(len_bytes) as usize;

            // Advance offset past Type and Length fields
            offset += 4;

            // Safety Check: Check if the value payload overflows the slice length
            if offset + l > self.data.len() {
                return Err(KontError::TlvParsingOverflow);
            }

            // 3. Match the extension type
            if t == extension_type {
                return Ok(Some(&self.data[offset..offset + l]));
            }

            // Move the offset past the Value payload to parse the next extension
            offset += l;
        }

        // Extension not found on this account
        Ok(None)
    }
}