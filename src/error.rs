/// Errors that can be returned by ratpak.
///
/// These correspond to the error codes produced by the underlying C/C++ math library,
/// and are mapped from raw `u32` codes via [`From<u32>`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    /// A division by zero was attempted.
    DivideByZero,
    /// The input is outside the domain of the operation (e.g. `sqrt` of a negative number).
    Domain,
    /// The result is mathematically indeterminate (e.g. `0 / 0`).
    Indefinite,
    /// The result overflows to positive infinity.
    PosInfinity,
    /// The result overflows to negative infinity.
    NegInfinity,
    /// An argument falls outside the valid range for the operation.
    InvalidRange,
    /// Ratpak could not allocate sufficient memory to complete the operation.
    OutOfMemory,
    /// A numeric overflow occurred during the computation.
    Overflow,
    /// The operation produced no result (e.g. an empty input expression).
    NoResult,
    /// An unrecognised error code was returned by ratpak.
    Unknown(u32),
}

impl From<u32> for CalcError {
    /// Converts a raw ratpak error code into a [`CalcError`] variant.
    ///
    /// Any code that does not match a known constant is wrapped in [`CalcError::Unknown`].
    fn from(code: u32) -> Self {
        match code {
            0x80000000 => CalcError::DivideByZero,
            0x80000001 => CalcError::Domain,
            0x80000002 => CalcError::Indefinite,
            0x80000003 => CalcError::PosInfinity,
            0x80000004 => CalcError::NegInfinity,
            0x80000006 => CalcError::InvalidRange,
            0x80000007 => CalcError::OutOfMemory,
            0x80000008 => CalcError::Overflow,
            0x80000009 => CalcError::NoResult,
            _ => CalcError::Unknown(code),
        }
    }
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CalcError {}

/// Convenience alias for `Result<T, CalcError>`.
pub type Result<T> = std::result::Result<T, CalcError>;