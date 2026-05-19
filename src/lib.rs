//! Safe Rust bindings for ratpak.
//!
//! This crate wraps a C/C++ arbitrary-precision math library and exposes:
//!
//! - [`Rational`] — an arbitrary-precision rational number backed by ratpak.
//! - [`RationalMath`] — transcendental / scientific functions (`sin`, `exp`, `log`, …).
//! - [`CalcError`] / [`Result`] — a typed error enum and convenience alias.
//! - [`initialize_engine`] — one-time ratpak initialisation that must be called before
//!   any arithmetic is performed.
//!
//! ```rust
//! use calc_engine::{initialize_engine, Rational, RationalMath, AngleType, RATIONAL_BASE, RATIONAL_PRECISION};
//!
//! initialize_engine(RATIONAL_BASE, RATIONAL_PRECISION);
//!
//! let a = Rational::from_i32(3);
//! let b = Rational::from_i32(4);
//! let c = a.checked_add(&b)?;
//! println!("{}", c); // "7"
//! ```

pub mod error;
pub mod ffi;
pub mod rational;

pub use error::{CalcError, Result};
pub use rational::{Rational, RationalMath, AngleType, NumberFormat, RATIONAL_BASE, RATIONAL_PRECISION};

/// Initialises ratpak.
///
/// This **must** be called once before any [`Rational`] values are created or any
/// arithmetic operations are performed. Calling again to change it is safe and fast.
///
/// ## Arguments
///
/// * `radix` — the default numeric base for internal representations (e.g. `10`).
/// * `precision` — the number of significant digits to maintain during calculations.
///
/// ## Example
///
/// ```rust
/// use calc_engine::{initialize_engine, RATIONAL_BASE, RATIONAL_PRECISION};
///
/// initialize_engine(RATIONAL_BASE, RATIONAL_PRECISION);
/// ```
pub fn initialize_engine(radix: u32, precision: i32) {
    unsafe {
        ffi::init_ratpack(radix, precision);
    }
}