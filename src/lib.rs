pub mod error;
pub mod ffi;
pub mod rational;

pub use error::{CalcError, Result};
pub use rational::{Rational, RationalMath, AngleType, NumberFormat, RATIONAL_BASE, RATIONAL_PRECISION};

pub fn initialize_engine(radix: u32, precision: i32) {
    unsafe {
        ffi::init_ratpack(radix, precision);
    }
}