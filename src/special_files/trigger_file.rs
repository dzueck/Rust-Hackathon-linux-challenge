use std::{ffi::{c_int, OsStr, OsString}, fmt::Debug, time::SystemTime};

use fuser::FileAttr;

use crate::{errors::{NOT_SUPPORTED, PERMISSION_DENIED}, file_helpers::read, files::File, main_fs::{GID, UID}};

#[derive(Debug)]
pub struct TriggerFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub data: Vec<u8>,
    pub triggered: bool,
    pub trigger: fn() -> (),
}

impl TriggerFile {
    pub fn new(trigger: fn() -> (), name: &OsStr, data: Vec<u8>, ino: u64, mode: u32, flags: u32) -> Self {
        Self {
            attr: FileAttr { 
                ino, 
                size: data.len() as u64, 
                blocks: 0, 
                atime: SystemTime::now(), 
                mtime: SystemTime::now(), 
                ctime: SystemTime::now(), 
                crtime: SystemTime::now(), 
                kind: fuser::FileType::RegularFile, 
                perm: mode as u16, 
                nlink: 0, 
                uid: *UID, 
                gid: *GID, 
                rdev: 0, 
                blksize: 0, 
                flags,
            },
            name: name.to_os_string(),
            data,
            triggered: false,
            trigger,
        }
    }
}

impl File for TriggerFile {
    fn name(&self) -> &OsStr {
        &self.name
    }

    fn attr(&self) -> &FileAttr {
        &self.attr
    }

    fn setattr(
        &mut self, 
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        fh: Option<u64>,
        flags: Option<u32>,
    ) -> Result<(), c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn read(&mut self, offset: i64, size: u32, flags: i32) -> Result<&[u8], c_int> {
        let out = read(&self.data, offset, size);
        if out.is_ok() && !self.triggered {
            (self.trigger)();
            self.triggered = true;
        }
        out
    }
    
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        Err(PERMISSION_DENIED)
    }
}