use crate::errors::KontError;

/// A wrapper around a 32-byte array representing a public key.
/// Derives common traits for cloning, copying, debugging, and structural equality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawPubKey(pub [u8; 32]);

impl RawPubKey {
    /// Creates a new `RawPubKey` by copying a 32-byte array.
    fn new(bytes: &[u8; 32]) -> Self {
        Self(*bytes)
    }

    /// Returns a reference to the underlying 32-byte array.
    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Checks for equality between two `RawPubKey` instances.
    /// NOTE: Because `PartialEq` is derived above, `self == other` works out-of-the-box.
    /// If you call this specific method, it will safely use that derived equality.
    fn eq(&self, other: &RawPubKey) -> bool {
        self == other
    }
}

/// Represents a raw, zero-copy view of an account's state.
/// Holds references to the account's key, owner, and underlying data payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawAccount<'a> {
    /// The public key identifier of this account.
    pub key: &'a RawPubKey,
    /// The public key of the program/entity that owns this account.
    pub owner: &'a RawPubKey,
    /// The raw byte slice containing the account's state data.
    pub data: &'a [u8],
}

impl<'a> RawAccount<'a> {
    /// Constructs a new `RawAccount` using borrowed references.
    fn new(key: &'a RawPubKey, owner: &'a RawPubKey, data: &'a [u8]) -> Self {
        Self { key, owner, data }
    }

    /// Returns a reference to the account's public key.
    fn key(&self) -> &RawPubKey {
        self.key
    }

    /// Returns a reference to the account owner's public key.
    fn owner(&self) -> &RawPubKey {
        self.owner
    }

    /// Borrows the account's data slice. 
    /// Returns a `Result` to conform to error-handling interfaces, 
    /// though it currently always returns `Ok`.
    fn borrow_data(&self) -> Result<&[u8], KontError> {
        Ok(self.data)
    }
}