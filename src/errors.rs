#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KontError {
    InvalidBufferLength,
    MisalignedPointer,
    InvalidAccountOwner,
    InsufficientFunds,
    UnauthorizedSignature,
    UnknownExtensionType,
    TlvParsingOverflow,
    CheckedMathFailure,
}
