use std::ffi::{c_int, OsStr, OsString};

use fuser::FileAttr;

use crate::errors::NOT_SUPPORTED;

#[derive(Debug)]
pub struct Link {
    pub ino: u64,
    pub name: OsString,
    pub attr: FileAttr,
}


impl Link {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn attr(&self) -> &FileAttr {
        &self.attr
    }

    pub fn setattr(
        &mut self, 
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        fh: Option<u64>,
        flags: Option<u32>,
    ) -> Result<(), c_int> {
        return Err(NOT_SUPPORTED);
        // if size.is_some() || fh.is_some() {
        //     return Err(NOT_SUPPORTED);
        // }

        // if let Some(mode) = mode {
        //     self.attr.perm = mode as u16;
        // }
        // if let Some(uid) = uid {
        //     self.attr.uid = uid;
        // }
        // if let Some(gid) = gid {
        //     self.attr.gid = gid;
        // }
        // if let Some(flags) = flags {
        //     self.attr.flags = flags;
        // }
        // Ok(())
    }

    pub fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int> {
        self.name = new_name.to_os_string();
        return Ok(())
    }

    pub fn delete(&mut self) -> Result<(), c_int> {
        return Err(NOT_SUPPORTED);
    }

}