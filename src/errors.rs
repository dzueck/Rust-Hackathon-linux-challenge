use std::ffi::c_int;

use libc::{EACCES, ENOENT, ENOTEMPTY, ENOTSUP};


pub const FILE_NOT_FOUND: c_int = ENOENT;
pub const NOT_SUPPORTED: c_int = ENOTSUP;
pub const PERMISSION_DENIED: c_int = EACCES;
pub const DIR_NOT_EMPTY: c_int = ENOTEMPTY;