use crate::error::{CalcError, Result};
use crate::ffi;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

pub const RATIONAL_BASE: u32 = 10;
pub const RATIONAL_PRECISION: i32 = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberFormat {
    Float = 0,
    Scientific = 1,
    Engineering = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleType {
    Degrees = 0,
    Radians = 1,
    Gradians = 2,
}

pub struct Rational(pub(crate) ffi::PRAT);
unsafe impl Send for Rational {}
unsafe impl Sync for Rational {}

impl Rational {
    unsafe fn from_raw(ptr: ffi::PRAT) -> Self {
        Rational(ptr)
    }

    pub fn from_i32(val: i32) -> Self {
        unsafe { Self::from_raw(ffi::rat_from_i32(val)) }
    }

    pub fn from_u32(val: u32) -> Self {
        unsafe { Self::from_raw(ffi::rat_from_u32(val)) }
    }

    pub fn to_u64(&self) -> Result<u64> {
        let mut out = 0u64;
        let err = unsafe { ffi::rat_to_u64(self.0, RATIONAL_BASE, RATIONAL_PRECISION, &mut out) };
        if err == 0 {
            Ok(out)
        } else {
            Err(CalcError::from(err))
        }
    }

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

    pub fn checked_add(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_addrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn checked_sub(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_subrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn checked_mul(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_mulrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn checked_div(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_divrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn checked_rem(&self, other: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe { ffi::wrap_modrat(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }
}

impl Drop for Rational {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                ffi::free_rat(self.0);
            }
        }
    }
}

impl Clone for Rational {
    fn clone(&self) -> Self {
        unsafe { Rational::from_raw(ffi::clone_rat(self.0)) }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_formatted_string(RATIONAL_BASE, NumberFormat::Float, RATIONAL_PRECISION)
            .unwrap_or_else(|_| "Error".to_string());
        write!(f, "{}", s)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let mut out = false;
        let err = unsafe { ffi::wrap_rat_equ(self.0, other.0, RATIONAL_PRECISION, &mut out) };
        if err != 0 { false } else { out }
    }
}
impl Eq for Rational {}

impl PartialOrd for Rational {
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

pub struct RationalMath;

impl RationalMath {
    pub fn sin(rat: &Rational, angle_type: AngleType) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_sinrat(rat.0, angle_type as i32, RATIONAL_BASE, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn exp(rat: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_exprat(rat.0, RATIONAL_BASE, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }

    pub fn log(rat: &Rational) -> Result<Rational> {
        let mut out = std::ptr::null_mut();
        let err = unsafe {
            ffi::wrap_lograt(rat.0, RATIONAL_PRECISION, &mut out)
        };
        if err == 0 { Ok(unsafe { Rational::from_raw(out) }) } else { Err(err.into()) }
    }
}