 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KontError {
    /// The provided byte slice is shorter than the minimum size required
    /// for the target view (e.g. less than 165 bytes for a legacy
    /// `TokenAccountView`).
    InvalidBufferLength,

    /// The data pointer/offset does not satisfy the alignment requirements
    /// needed to safely read a field at its expected byte offset.
    MisalignedPointer,

    /// The account's owner field does not match the expected authority
    /// or program ID for the operation being performed.
    InvalidAccountOwner,

    /// The account's token balance is lower than the amount required
    /// to complete the requested operation (e.g. a transfer or burn).
    InsufficientFunds,

    /// The signature provided for this instruction does not match an
    /// authorized signer for the account or authority in question.
    UnauthorizedSignature,

    /// A Token-2022 extension type was encountered that Kont does not
    /// recognize or does not currently support parsing.
    UnknownExtensionType,

    /// While scanning a Token-2022 account's TLV (type-length-value)
    /// extension data, a declared length would read past the end of
    /// the buffer.
    TlvParsingOverflow,

    /// An arithmetic operation (e.g. addition, subtraction, or
    /// multiplication on token amounts) overflowed or underflowed
    /// during a checked computation.
    CheckedMathFailure,
}