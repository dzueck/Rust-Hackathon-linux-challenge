use std::{collections::{HashMap, HashSet}, ffi::{c_int, OsStr, OsString}, fmt::Debug};

use fuser::FileAttr;
use libc::ENOENT;

use crate::{errors::{FILE_NOT_FOUND, NOT_SUPPORTED}, main_fs::Ino};


pub trait Dir: Send + Debug {
    fn lookup_child(&self, child_name: &OsStr, inos: &HashMap<u64, Ino>) -> Result<u64, c_int>;
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
    fn is_user_dir(&self) -> bool;
    fn add_child(&mut self, ino: u64) -> Result<(), c_int>;
    fn remove_child(&mut self, ino: u64) -> Result<(), c_int>;
    fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int>;
    fn get_child(&self, index: usize) -> Option<u64>;
    fn delete(&mut self) -> Result<(), c_int>;
}
