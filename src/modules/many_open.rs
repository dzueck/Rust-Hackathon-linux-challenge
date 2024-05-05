use std::{ffi::{c_int, OsStr, OsString}, str::FromStr, time::SystemTime};

use fuser::FileAttr;

use crate::{background_tasks::{add_file, DEFAULT_MODE}, errors::PERMISSION_DENIED, file_helpers::{read, str_to_vec, victory_file}, files::File, main_fs::{get_unique_ino, GID, UID}};

pub fn start() {
    add_file("The_Door", Box::new(ManyOpenFile::new(victory, "Heavy_Door", get_unique_ino())));
}

pub fn victory() {
    add_file("The_Door", victory_file())
}

const MESSAGE_1: &str = 
"You think you can get past me that easy.
You would need 10 men to try and open me to even have a chance.
";

const MESSAGE_2: &str = 
"Wow you actually got 10 men.
The door is now open and you have beaten this module.
";

#[derive(Debug)]
pub struct ManyOpenFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub data: Vec<u8>,
    pub num_opens: u32,
    pub trigger: fn() -> (),
    pub data2: Vec<u8>,
    pub triggered: bool,
}

impl ManyOpenFile {
    pub fn new(trigger: fn() -> (), name: &str, ino: u64) -> Self {
        let data = str_to_vec(MESSAGE_1);
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
                perm: DEFAULT_MODE as u16, 
                nlink: 0, 
                uid: *UID, 
                gid: *GID, 
                rdev: 0, 
                blksize: 0, 
                flags: 0,
            },
            name: OsString::from_str(name).unwrap(),
            data,
            data2: str_to_vec(MESSAGE_2),
            trigger,
            num_opens: 0,
            triggered: false,
        }
    }
}

impl File for ManyOpenFile {
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
        let data = if self.num_opens >= 10 {
            if !self.triggered {
                (self.trigger)();
                self.triggered = true;
            }
            println!("Data 2");
            &self.data2
        } else {
            println!("Data 1");

            &self.data
        };

        let out = read(data, offset, size);
        out
    }
    
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        Err(PERMISSION_DENIED)
    }

    fn open(&mut self) -> Result<(), c_int> {
        self.num_opens += 1;
        println!("Open: {}", self.num_opens);
        Ok(())
    }

    fn release(&mut self) -> Result<(), c_int> {
        self.num_opens -= 1;
        println!("Close: {}", self.num_opens);
        Ok(())
    }
}