use std::os::raw::{c_int, c_uint};

#[repr(C)]
pub struct _rat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _number {
    _private: [u8; 0],
}

pub type PRAT = *mut _rat;
pub type PNUMBER = *mut _number;

extern "C" {
    pub fn init_ratpack(radix: c_uint, precision: c_int);
    pub fn free_rat(rat: PRAT);
    pub fn free_num(num: PNUMBER);
    pub fn clone_rat(rat: PRAT) -> PRAT;
    pub fn free_string(s: *mut u16);
    pub fn rat_from_i32(val: i32) -> PRAT;
    pub fn rat_from_u32(val: u32) -> PRAT;
    pub fn rat_to_u64(rat: PRAT, radix: c_uint, precision: c_int, out: *mut u64) -> u32;
    pub fn rat_to_string(rat: PRAT, radix: c_uint, format: c_int, precision: c_int, out_str: *mut *mut u16) -> u32;
    pub fn wrap_addrat(a: PRAT, b: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_subrat(a: PRAT, b: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_mulrat(a: PRAT, b: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_divrat(a: PRAT, b: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_modrat(a: PRAT, b: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_rat_equ(a: PRAT, b: PRAT, precision: c_int, out: *mut bool) -> u32;
    pub fn wrap_rat_lt(a: PRAT, b: PRAT, precision: c_int, out: *mut bool) -> u32;
    pub fn wrap_sinrat(x: PRAT, angle_type: c_int, radix: c_uint, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_exprat(x: PRAT, radix: c_uint, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_lograt(x: PRAT, precision: c_int, out: *mut PRAT) -> u32;
    pub fn wrap_cosrat(x: PRAT, angle_type: c_int, radix: c_uint, precision: c_int, out: *mut PRAT) -> u32;
}