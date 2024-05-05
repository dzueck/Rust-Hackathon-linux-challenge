use std::{ffi::{c_int, OsStr, OsString}, process::Command, str::FromStr, sync::Mutex, time::SystemTime};

use fuser::FileAttr;
use lazy_static::lazy_static;

use crate::{background_tasks::{add_file, DEFAULT_MODE}, errors::PERMISSION_DENIED, file_helpers::{read, text_file, victory_file}, files::File, main_fs::{get_unique_ino, GID, UID}, special_files::trigger_file::TriggerFile, MOUNT_POINT};

const BASE_PATH: &str = "Arena";
const FILE_NAME: &str = "Ogre";
const OGRE_MESSAGE: &str = 
"He is scary!
";

const WARRIOR_MESSAGE: &str = 
"Ah adventurer!
Help me kill these Ogres.
";

lazy_static! {
    static ref OGRES_LEFT: Mutex<usize> = {
        Mutex::new(NUM_OGRES)
    };
}

const NUM_OGRES: usize = 20;

pub fn start() {
    let file_path = format!("{MOUNT_POINT}/{BASE_PATH}/{FILE_NAME}");
    Command::new("chmod").arg("+s").arg("arg").output().expect("Failed to run command");

    add_file(BASE_PATH, text_file("Warrior", WARRIOR_MESSAGE));

    for _ in 0..NUM_OGRES {
        add_file(BASE_PATH, Box::new(OgreFile::new(killed_ogre, FILE_NAME, OGRE_MESSAGE.as_bytes().iter().map(|x| *x).collect(), get_unique_ino(), DEFAULT_MODE, 0)));
    }
}

fn killed_ogre() {
    *OGRES_LEFT.lock().unwrap() -= 1;

    if *OGRES_LEFT.lock().unwrap() <= 0 {
        add_file(BASE_PATH, victory_file());
    }

}

#[derive(Debug)]
pub struct OgreFile {
    pub attr: FileAttr,
    pub name: OsString,
    pub data: Vec<u8>,
    pub triggered: bool,
    pub trigger: fn() -> (),
}

impl OgreFile {
    pub fn new(trigger: fn() -> (), name: &str, data: Vec<u8>, ino: u64, mode: u32, flags: u32) -> Self {
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
            name: OsString::from_str(name).unwrap(),
            data,
            triggered: false,
            trigger,
        }
    }
}

impl File for OgreFile {
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
        out
    }
    
    fn write(&mut self, offset: i64, data: &[u8], write_flags: u32, flags: i32) -> Result<u32, c_int> {
        Err(PERMISSION_DENIED)
    }
    
    fn delete(&mut self) -> Result<(), c_int> {
        println!("Delete");
        (self.trigger)();
        Ok(())
    }
}