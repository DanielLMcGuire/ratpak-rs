use crate::error::{CalcError, Result};
use crate::ffi;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

/// The default numeric base used for string formatting and internal representation.
pub const RATIONAL_BASE: u32 = 10;

/// The default number of significant digits maintained during calculations.
pub const RATIONAL_PRECISION: i32 = 32;

/// Controls how a [`Rational`] is formatted as a string.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberFormat {
    /// Standard decimal notation: `3.14159`.
    Float = 0,
    /// Scientific notation: `3.14159e+0`.
    Scientific = 1,
    /// Engineering notation (exponents are multiples of 3): `3.14159e+0`.
    Engineering = 2,
}

/// The angular unit used for trigonometric operations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleType {
    /// Angles measured in degrees (0 – 360).
    Degrees = 0,
    /// Angles measured in radians (0 – 2π).
    Radians = 1,
    /// Angles measured in gradians (0 – 400).
    Gradians = 2,
}

/// An arbitrary-precision rational number managed by ratpak.
///
/// `Rational` is a thin, owning wrapper around an opaque `PRAT` pointer.
/// All arithmetic is delegated to ratpak, which handles memory
/// allocation; the pointer is freed via [`ffi::free_rat`] when the value is dropped.
///
/// ## Thread safety
///
/// `Rational` is `Send + Sync` because ratpak treats each `PRAT`
/// allocation as an independent value with no shared mutable state.
///
/// ## Panicking operators
///
/// The standard operator traits (`+`, `-`, `*`, `/`, `%`) are implemented for
/// convenience and will **panic** on error. Prefer the `checked_*` methods in
/// production code where errors must be handled gracefully.
pub struct Rational(pub(crate) ffi::PRAT);
unsafe impl Send for Rational {}
unsafe impl Sync for Rational {}

impl Rational {
    /// Wraps a raw `PRAT` pointer without performing any checks.
    ///
    /// ## Safety
    ///
    /// `ptr` must be a valid, non-null `PRAT` pointer obtained from ratpak.
    /// Ownership is transferred to the returned `Rational`; the pointer must
    /// not be freed by the caller.
    unsafe fn from_raw(ptr: ffi::PRAT) -> Self {
        Rational(ptr)
    }

    /// Creates a `Rational` from a signed 32-bit integer.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let r = Rational::from_i32(-42);
    /// ```
    pub fn from_i32(val: i32) -> Self {
        unsafe { Self::from_raw(ffi::rat_from_i32(val)) }
    }

    /// Creates a `Rational` from an unsigned 32-bit integer.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let r = Rational::from_u32(100);
    /// ```
    pub fn from_u32(val: u32) -> Self {
        unsafe { Self::from_raw(ffi::rat_from_u32(val)) }
    }

    /// Attempts to convert this `Rational` to a `u64`.
    ///
    /// Returns [`CalcError::Overflow`] if the value cannot be represented as a `u64`,
    /// or another [`CalcError`] variant if ratpak reports a different failure.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let r = Rational::from_u32(42);
    /// assert_eq!(r.to_u64()?, 42u64);
    /// ```
    pub fn to_u64(&self) -> Result<u64> {
        let mut out = 0u64;
        let err = unsafe { ffi::rat_to_u64(self.0, RATIONAL_BASE, RATIONAL_PRECISION, &mut out) };
        if err == 0 {
            Ok(out)
        } else {
            Err(CalcError::from(err))
        }
    }

    /// Formats this `Rational` as a string under the given `radix`, `format`, and
    /// `precision`.
    ///
    /// Ratpak allocates the output buffer; this method copies it into a Rust
    /// [`String`] and then frees the buffer.
    ///
    /// ## Arguments
    ///
    /// * `radix` — the numeric base for the output (e.g. `10` for decimal, `16` for hex).
    /// * `format` — one of [`NumberFormat::Float`], [`NumberFormat::Scientific`], or
    ///   [`NumberFormat::Engineering`].
    /// * `precision` — the number of significant digits in the output.
    ///
    /// ## Errors
    ///
    /// Returns a [`CalcError`] if ratpak fails to format the value.
    pub fn to_formatted_string(&self, radix: u32, format: NumberFormat, precision: i32) -> Result<String> {
        let mut out_str: *mut u16 = std::ptr::null_mut();
        let err = unsafe {
            ffi::rat_to_string(self.0, radix, format as i32, precision, &mut out_str)
        };

        if err != 0 {
            return Err(CalcError::from(err));
        }

        unsafe {
            let mut len = 0;
            while *out_str.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(out_str, len);
            let result = String::from_utf16_lossy(slice);

            ffi::free_string(out_str);
            Ok(result)
        }
    }

    /// Adds `self` and `other`, returning a new `Rational` or a [`CalcError`].
    pub fn checked_add(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_addrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Subtracts `other` from `self`, returning a new `Rational` or a [`CalcError`].
    pub fn checked_sub(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_subrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Multiplies `self` by `other`, returning a new `Rational` or a [`CalcError`].
    pub fn checked_mul(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_mulrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Divides `self` by `other`, returning a new `Rational` or a [`CalcError`].
    ///
    /// Returns [`CalcError::DivideByZero`] if `other` is zero.
    pub fn checked_div(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_divrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Computes `self % other`, returning a new `Rational` or a [`CalcError`].
    ///
    /// Returns [`CalcError::DivideByZero`] if `other` is zero.
    pub fn checked_rem(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_modrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }
}

impl Drop for Rational {
    /// Frees the underlying `PRAT` allocation via ratpak's deallocator.
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                ffi::free_rat(self.0);
            }
        }
    }
}

impl Clone for Rational {
    /// Returns a deep copy of this `Rational` by asking ratpak to duplicate
    /// the underlying `PRAT` structure.
    fn clone(&self) -> Self {
        unsafe { Rational::from_raw(ffi::clone_rat(self.0)) }
    }
}

impl fmt::Display for Rational {
    /// Formats the value as a decimal float string using [`RATIONAL_BASE`] and
    /// [`RATIONAL_PRECISION`]. Falls back to `"Error"` if formatting fails.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_formatted_string(RATIONAL_BASE, NumberFormat::Float, RATIONAL_PRECISION)
            .unwrap_or_else(|_| "Error".to_string());
        write!(f, "{}", s)
    }
}

impl PartialEq for Rational {
    /// Returns `true` if `self` and `other` are numerically equal.
    ///
    /// Returns `false` if ratpak reports an error during comparison.
    fn eq(&self, other: &Self) -> bool {
        let mut out = false;
        let err = unsafe { ffi::wrap_rat_equ(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err != 0 { false } else { out }
    }
}
impl Eq for Rational {}

impl PartialOrd for Rational {
    /// Compares `self` and `other`, returning the appropriate [`std::cmp::Ordering`].
    ///
    /// Returns `None` if ratpak reports an error during the less-than comparison.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(std::cmp::Ordering::Equal)
        } else {
            let mut out = false;
            let err = unsafe { ffi::wrap_rat_lt(self.0, other.0, RATIONAL_PRECISION, &mut out) };
            if err != 0 {
                None
            } else if out {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

/// Implements a panicking binary operator trait by delegating to the corresponding
/// `checked_*` method on [`Rational`].
///
/// Four combinations of owned / borrowed operands are generated for each trait so
/// that the operator can be used ergonomically in expressions.
macro_rules! impl_op {
    ($trait:ident, $method:ident, $checked_method:ident) => {
        impl $trait for &Rational {
            type Output = Rational;
            fn $method(self, rhs: Self) -> Rational {
                self.$checked_method(rhs).expect(concat!(stringify!($trait), " operation resulted in CalcError"))
            }
        }
        impl $trait<Rational> for &Rational {
            type Output = Rational;
            fn $method(self, rhs: Rational) -> Rational {
                self.$checked_method(&rhs).expect(concat!(stringify!($trait), " operation resulted in CalcError"))
            }
        }
        impl $trait<&Rational> for Rational {
            type Output = Rational;
            fn $method(self, rhs: &Rational) -> Rational {
                self.$checked_method(rhs).expect(concat!(stringify!($trait), " operation resulted in CalcError"))
            }
        }
        impl $trait<Rational> for Rational {
            type Output = Rational;
            fn $method(self, rhs: Rational) -> Rational {
                self.$checked_method(&rhs).expect(concat!(stringify!($trait), " operation resulted in CalcError"))
            }
        }
    };
}

impl_op!(Add, add, checked_add);
impl_op!(Sub, sub, checked_sub);
impl_op!(Mul, mul, checked_mul);
impl_op!(Div, div, checked_div);
impl_op!(Rem, rem, checked_rem);

/// A collection of mathematical functions that operate on [`Rational`] values.
///
/// Each function delegates to ratpak and returns a new `Rational`
/// on success, or a [`CalcError`] on failure.
pub struct RationalMath;

impl RationalMath {
    /// Computes the sine of `rat` using the specified angular unit.
    ///
    /// ## Errors
    ///
    /// Returns [`CalcError::Domain`] or another variant if ratpak cannot
    /// evaluate the function for the given input.
    pub fn sin(rat: &Rational, angle_type: AngleType) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_sinrat(rat.0, angle_type as i32, RATIONAL_BASE, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Computes *e* raised to the power of `rat` (i.e. `eˣ`).
    ///
    /// ## Errors
    ///
    /// Returns a [`CalcError`] if the result would overflow or ratpak fails.
    pub fn exp(rat: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_exprat(rat.0, RATIONAL_BASE, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    /// Computes the natural logarithm (base *e*) of `rat`.
    ///
    /// ## Errors
    ///
    /// Returns [`CalcError::Domain`] if `rat` is zero or negative, or another
    /// [`CalcError`] variant if ratpak fails for any other reason.
    pub fn log(rat: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_lograt(rat.0, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }
}