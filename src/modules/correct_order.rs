use std::{cell::RefCell, ffi::{c_int, OsStr, OsString}, str::FromStr, sync::Mutex, time::SystemTime};

use fuser::FileAttr;
use lazy_static::lazy_static;

const BASE_PATH: &str = "Broken_Sorter";

use crate::{background_tasks::{add_file, DEFAULT_MODE}, errors::PERMISSION_DENIED, file_helpers::{read, victory_file}, files::File, main_fs::{get_unique_ino, GID, UID}};


const ORDER: [usize; 8] = [3, 5, 4, 1, 2, 7, 6, 99999999];

lazy_static! {
    static ref POS: Mutex<usize> = {
        Mutex::new(0)
    };
}

const WRONG_MESSAGE: &str = "No \n";
const RIGHT_MESSAGE: &str = "Yes\n";

pub fn start() {
    for i in 0..8 {
        add_file(BASE_PATH, Box::new(OrderFile::new(i)));
    }
}

fn order_trigger(file_num: usize) -> bool {
    let mut pos = POS.lock().unwrap();
    if ORDER[*pos] == file_num {
        *pos += 1;
        if *pos >= ORDER.len() - 1 {
            add_file(BASE_PATH, victory_file());
        }
        true
    } else {
        *pos = 0;
        false
    }
}

#[derive(Debug)]
pub struct OrderFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub file_num: usize,
}

impl OrderFile {
    pub fn new(file_num: usize) -> Self {
        Self {
            attr: FileAttr { 
                ino: get_unique_ino(), 
                size: 4, 
                blocks: 0, 
                atime: SystemTime::now(), 
                mtime: SystemTime::now(), 
                ctime: SystemTime::now(), 
                crtime: SystemTime::now(), 
                kind: fuser::FileType::RegularFile, 
                perm: DEFAULT_MODE as u16, 
                nlink: 0, 
                uid: *UID, 
                gid: *GID, 
                rdev: 0, 
                blksize: 0, 
                flags: 0,
            },
            name: OsString::from_str(&format!("{file_num}")).unwrap(),
            file_num,
        }
    }
}

impl File for OrderFile {
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
        let data = if order_trigger(self.file_num) {
            RIGHT_MESSAGE.as_bytes()
        } else {
            WRONG_MESSAGE.as_bytes()
        };
        read(data, offset, size)
    }
    
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        Err(PERMISSION_DENIED)
    }
}