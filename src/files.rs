use std::{ffi::{c_int, OsStr}, fmt::Debug};

use fuser::FileAttr;



pub trait File: Send + Debug {
    fn name(&self) -> &OsStr;
    fn attr(&self) -> &FileAttr;
    fn setattr(
        &mut self, 
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        fh: Option<u64>,
        flags: Option<u32>,
    ) -> Result<(), c_int>;
    fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int>;
    fn read(&mut self, offset: i64, size: u32, flags: i32) -> Result<&[u8], c_int>;
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int>;
    fn delete(&mut self) -> Result<(), c_int>;
}