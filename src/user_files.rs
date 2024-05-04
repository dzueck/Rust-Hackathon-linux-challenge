use std::{collections::HashMap, ffi::{c_int, OsStr, OsString}, iter::repeat, os::unix::ffi::OsStrExt, time::SystemTime};

use fuser::FileAttr;

use crate::{dirs::Dir, errors::{DIR_NOT_EMPTY, FILE_NOT_FOUND, NOT_SUPPORTED, PERMISSION_DENIED}, files::File, main_fs::Ino};

#[derive(Debug)]
pub struct UserFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub data: Vec<u8>,   
}

impl UserFile {
    pub fn new(name: &OsStr, ino: u64, size: u64, mode: u32, uid: u32, gid: u32, flags: u32) -> Self {
        UserFile {
            attr: FileAttr { 
                ino, 
                size, 
                blocks: 0, 
                atime: SystemTime::now(), 
                mtime: SystemTime::now(), 
                ctime: SystemTime::now(), 
                crtime: SystemTime::now(), 
                kind: fuser::FileType::RegularFile, 
                perm: mode as u16, 
                nlink: 0, 
                uid, 
                gid, 
                rdev: 0, 
                blksize: 0, 
                flags,
            },
            name: name.to_os_string(),
            data: vec![0; size as usize]
        }
    }
}

impl File for UserFile {
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
        if let Some(mode) = mode {
            self.attr.perm = mode as u16;
        }
        if let Some(uid) = uid {
            self.attr.uid = uid;
        }
        if let Some(gid) = gid {
            self.attr.gid = gid;
        }
        if let Some(flags) = flags {
            self.attr.flags = flags;
        }
        if let Some(size) = size {
            self.data.resize(size as usize, 0);
            self.attr.size = size;
        }
        Ok(())
    }
    
    fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int> {
        if !in_user_dir && new_name.as_bytes()[0] != b'_' {
            return Err(PERMISSION_DENIED);
        }

        self.name = new_name.to_os_string();
        Ok(())
    }
    
    fn read(&mut self, offset: i64, size: u32, flags: i32) -> Result<&[u8], c_int> {
        if offset.is_negative() ||  offset as usize >= self.data.len() {
            return Ok(&[]);
        }

        let offset = offset as usize;
        let end = offset + size as usize;

        if end >= self.data.len() {
            return Ok(&self.data[offset..]);
        } else {
            return Ok(&self.data[offset..end]);
        }
    }
    
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int> {
        if offset.is_negative() {
            return Err(NOT_SUPPORTED);
        }
        let offset = offset as usize;
        let end = offset + data.len();
        if self.data.len() < end {
            let needed_data = end - self.data.len();
            self.data.extend(repeat(0).take(needed_data));
        }
        self.data[offset..end].copy_from_slice(data);
        self.attr.size = self.data.len() as u64;
        Ok(data.len() as u32)
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct NormalDir {
    pub children: Vec<u64>,
    pub name: OsString,
    pub user_dir: bool,
    pub attr: FileAttr,
}

impl NormalDir {
    pub fn new(name: &OsStr, is_user_dir: bool, ino: u64, mode: u32, uid: u32, gid: u32, flags: u32) -> Self {
        let children = Vec::new();

        NormalDir {
            attr: FileAttr { 
                ino, 
                size: children.len() as u64, 
                blocks: 0, 
                atime: SystemTime::now(), 
                mtime: SystemTime::now(), 
                ctime: SystemTime::now(), 
                crtime: SystemTime::now(), 
                kind: fuser::FileType::Directory, 
                perm: mode as u16, 
                nlink: 0, 
                uid, 
                gid, 
                rdev: 0, 
                blksize: 0, 
                flags,
            },
            name: name.to_os_string(),
            children,
            user_dir: is_user_dir,
        }
    }
}

impl Dir for NormalDir {
    fn lookup_child<'a>(&self, child_name: &OsStr, inos: &'a HashMap<u64, Ino>) -> Result<u64, c_int> {
        for child in &self.children {
            let child = inos.get(child).unwrap();
            if child.name() == child_name {
                return Ok(child.attr().ino);
            }
        }
        Err(FILE_NOT_FOUND)
    }

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
        if size.is_some() {
            return Err(NOT_SUPPORTED);
        }

        if let Some(mode) = mode {
            self.attr.perm = mode as u16;
        }
        if let Some(uid) = uid {
            self.attr.uid = uid;
        }
        if let Some(gid) = gid {
            self.attr.gid = gid;
        }
        if let Some(flags) = flags {
            self.attr.flags = flags;
        }
        Ok(())
    }
    
    fn is_user_dir(&self) -> bool {
        self.user_dir
    }
    
    fn add_child(&mut self, ino: u64) -> Result<(), c_int> {
        self.children.push(ino);
        self.attr.size = self.children.len() as u64;
        Ok(())
    }
    
    fn remove_child(&mut self, ino: u64) -> Result<(), c_int> {
        self.children.retain(|x| *x != ino);
        return Ok(())
    }
    
    fn rename(&mut self, new_name: &OsStr, in_user_dir: bool) -> Result<(), c_int> {
        if !in_user_dir && new_name.as_bytes()[0] != b'_' {
            return Err(PERMISSION_DENIED);
        }

        if in_user_dir || new_name.as_bytes()[0] == b'_' {
            self.user_dir = true;
        }

        self.name = new_name.to_os_string();
        Ok(())
    }
    
    fn get_child(&self, index: usize) -> Option<u64> {
        println!("Index: {index}, len:{}, {:?}", self.children.len(), self.children.get(index).copied());
        self.children.get(index).copied()
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        if !self.children.is_empty() {
            return Err(DIR_NOT_EMPTY);
        }
        Ok(())
    }
}
