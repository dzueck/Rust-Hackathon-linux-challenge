use std::{ffi::{c_int, OsStr, OsString}, str::FromStr, time::SystemTime};

use fuser::FileAttr;

use crate::{background_tasks::{add_file, DEFAULT_MODE}, errors::{FILE_NOT_FOUND, PERMISSION_DENIED}, file_helpers::{read, str_to_vec, text_file, victory_file}, files::File, main_fs::{get_unique_ino, GID, UID}};

use super::many_open::ManyOpenFile;


const TEACHER_MESSAGE: &str = 
"I have been looking for Sally for the last 10 minutes but I can't find her.
She was here earlier, but once I said we were studying math instead of reading and writing she started hiding.
I guess she really likes reading and writing.
";

const SALLY_MESSAGE: &str = 
"Oh we are going back to reading and writing!
YEAYYY!
Ill come back to class.
";

const KID_1: &str =
"Ok boomer!
";

const KID_2: &str =
"I love Skibidi Toilet!
";

const KID_3: &str =
"STOP HITTING ME BILLY!
";

pub fn start() {
    add_file("Classroom", Box::new(SallyFile::new(victory, "Sally", str_to_vec(SALLY_MESSAGE), get_unique_ino())));
    add_file("Classroom", text_file("Teacher", TEACHER_MESSAGE));

    add_file("Classroom", text_file("Billy", KID_1));
    add_file("Classroom", text_file("Timmy", KID_2));
    add_file("Classroom", text_file("John", KID_3));
}

fn victory() {
    add_file("Classroom", victory_file());
}


#[derive(Debug)]
pub struct SallyFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub data: Vec<u8>,
    pub triggered: bool,
    pub trigger: fn() -> (),
}

impl SallyFile {
    pub fn new(trigger: fn() -> (), name: &str, data: Vec<u8>, ino: u64) -> Self {
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
            triggered: false,
            trigger,
        }
    }
}

impl File for SallyFile {
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

    fn open(&mut self, flags: i32) -> Result<u32, c_int> {
        println!("{:x} {} {}", flags, flags & libc::O_RDONLY, flags & libc::O_RDWR);
        if flags & libc::O_RDWR != 0 {
            Ok(0)
        } else {
            Err(FILE_NOT_FOUND)
        }
    }

}