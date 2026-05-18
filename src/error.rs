#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    DivideByZero,
    Domain,
    Indefinite,
    PosInfinity,
    NegInfinity,
    InvalidRange,
    OutOfMemory,
    Overflow,
    NoResult,
    Unknown(u32),
}

impl From<u32> for CalcError {
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

pub type Result<T> = std::result::Result<T, CalcError>;