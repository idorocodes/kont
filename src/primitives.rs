use crate::errors::KontError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawPubKey([u8; 32]);

impl RawPubKey {
    fn new(bytes: &[u8; 32]) -> Self {
        Self(*bytes)
    }

    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    fn eq(&self, other: &RawPubKey) -> bool {
        self == other
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawAccount<'a> {
    pub key: &'a RawPubKey,
    pub owner: &'a RawPubKey,
    pub data: &'a [u8],
}
impl<'a> RawAccount<'a> {
    fn new(key: &'a RawPubKey, owner: &'a RawPubKey, data: &'a [u8]) -> Self {
        Self { key, owner, data }
    }
    fn key(&self) -> &RawPubKey {
        self.key
    }
    fn owner(&self) -> &RawPubKey {
        self.owner
    }
    fn borrow_data(&self) -> Result<&[u8], KontError> {
        Ok(self.data)
    }
}
