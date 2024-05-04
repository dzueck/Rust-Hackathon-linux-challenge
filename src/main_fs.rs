use std::{collections::HashMap, ffi::{c_int, OsStr, OsString}, os::unix::ffi::OsStrExt, str::FromStr, sync::{atomic::AtomicU64, Mutex}, time::Duration};

use fuser::{FileAttr, Filesystem, KernelConfig, Request};
use lazy_static::lazy_static;

use crate::{dirs::Dir, errors::{NOT_SUPPORTED, PERMISSION_DENIED}, files::File, link::Link, user_files::{NormalDir, UserFile}};

const DEFAULT_CACHE: Duration = Duration::ZERO;

static NEXT_INO: AtomicU64 = AtomicU64::new(2);

lazy_static! {
    static ref DATA: Mutex<FsData> = {
        let mut inos = HashMap::new();
        inos.insert(1, Ino::Dir(Box::new(NormalDir::new(&OsString::from_str("root").unwrap(), false, 1, 0o777, 0, 0, 0))));
        Mutex::new(FsData {
            inos
        })
    };
}

pub struct MainFs {}

#[derive(Debug)]
pub enum Ino {
    File(Box<dyn File>),
    Dir(Box<dyn Dir>),
    Link(Link),
}

impl Ino {
    pub fn unwrap_dir(&self) -> &Box<dyn Dir> {
        self.try_unwrap_dir().unwrap()
    }

    pub fn try_unwrap_dir(&self) -> Option<&Box<dyn Dir>> {
        if let Ino::Dir(dir) = self {
            return Some(dir);
        }
        None
    }

    pub fn unwrap_dir_mut(&mut self) -> &mut Box<dyn Dir> {
        self.try_unwrap_dir_mut().unwrap()
    }

    pub fn try_unwrap_dir_mut(&mut self) -> Option<&mut Box<dyn Dir>> {
        if let Ino::Dir(dir) = self {
            return Some(dir);
        }
        None
    }

    pub fn unwrap_file(&self) -> &Box<dyn File> {
        self.try_unwrap_file().unwrap()
    }

    pub fn try_unwrap_file(&self) -> Option<&Box<dyn File>> {
        if let Ino::File(file) = self {
            return Some(file);
        }
        None
    }

    pub fn unwrap_file_mut(&mut self) -> &mut Box<dyn File> {
        self.try_unwrap_file_mut().unwrap()
    }

    pub fn try_unwrap_file_mut(&mut self) -> Option<&mut Box<dyn File>> {
        if let Ino::File(file) = self {
            return Some(file);
        }
        None
    }

    pub fn attr(&self) -> &FileAttr {
        match self {
            Ino::File(f) => f.attr(),
            Ino::Dir(d) => d.attr(),
            Ino::Link(l) => l.attr(),
        }
    }

    pub fn name(&self) -> &OsStr {
        match self {
            Ino::File(f) => f.name(),
            Ino::Dir(d) => d.name(),
            Ino::Link(l) => l.name(),
        }
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
        match self {
            Ino::File(f) => f.setattr(mode, uid, gid, size, fh, flags),
            Ino::Dir(d) => d.setattr(mode, uid, gid, size, fh, flags),
            Ino::Link(l) => l.setattr(mode, uid, gid, size, fh, flags),
        }
    }

    pub fn rename(&mut self, new_name: &OsStr, is_user_dir: bool) -> Result<(), c_int> {
        match self {
            Ino::File(f) => f.rename(new_name, is_user_dir),
            Ino::Dir(d) => d.rename(new_name, is_user_dir),
            Ino::Link(l) => l.rename(new_name, is_user_dir),
        }
    }

    pub fn delete(&mut self) -> Result<(), c_int> {
        match self {
            Ino::File(f) => f.delete(),
            Ino::Dir(d) => d.delete(),
            Ino::Link(l) => l.delete(),
        }
    }
}

#[derive(Debug)]
struct FsData{
    inos: HashMap<u64, Ino>,
    // fhs: HashMap<u64, Ino>,
}

impl MainFs {
    pub fn new() -> MainFs {
        MainFs {
        }
    }
}

impl Filesystem for MainFs {
    fn init(&mut self, _req: &Request<'_>, _config: &mut KernelConfig) -> Result<(), c_int> {
        println!("Init");
        Ok(())
    }

    fn destroy(&mut self) {
        println!("Destroy");
    }

    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: fuser::ReplyEntry) {
        //println!("Lookup {parent}:{name:?}");
        let data = DATA.lock().unwrap();
        match data.inos.get(&parent).unwrap().unwrap_dir().lookup_child(name, &data.inos) {
            Ok(cino) => reply.entry(&DEFAULT_CACHE, data.inos.get(&cino).unwrap().attr(), 0),
            Err(err) => reply.error(err),
        }
    }

    // fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {
        
    // }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        println!("Getattr: {ino}");
        reply.attr(&DEFAULT_CACHE, DATA.lock().unwrap().inos.get(&ino).unwrap().attr());
        // match DATA.lock().unwrap().inos.get(&ino).unwrap().attr() {
        //     Ok(attr) => reply.attr(&DEFAULT_CACHE, attr),
        //     Err(e) => reply.error(e),
        // }
    }

    fn setattr(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            mode: Option<u32>,
            uid: Option<u32>,
            gid: Option<u32>,
            size: Option<u64>,
            _atime: Option<fuser::TimeOrNow>,
            _mtime: Option<fuser::TimeOrNow>,
            _ctime: Option<std::time::SystemTime>,
            fh: Option<u64>,
            _crtime: Option<std::time::SystemTime>,
            _chgtime: Option<std::time::SystemTime>,
            _bkuptime: Option<std::time::SystemTime>,
            flags: Option<u32>,
            reply: fuser::ReplyAttr,
        ) {
        println!("setattr: {ino}");
        let mut data = DATA.lock().unwrap();
        let target = data.inos.get_mut(&ino).unwrap();
        if let Err(e) = target.setattr(mode, uid, gid, size, fh, flags) {
            reply.error(e);
        } else {
            reply.attr(&DEFAULT_CACHE, target.attr())
        }
    }

    fn mknod(
            &mut self,
            req: &Request<'_>,
            parent: u64,
            name: &std::ffi::OsStr,
            mode: u32,
            umask: u32,
            rdev: u32,
            reply: fuser::ReplyEntry,
        ) {
        println!("mknod: {parent} name: {name:?}");
        let mut data = DATA.lock().unwrap();
        let parent = data.inos.get_mut(&parent).unwrap().unwrap_dir_mut();
        if !parent.is_user_dir() && name.as_bytes()[0] != b'_' {
            reply.error(PERMISSION_DENIED);
            return;
        }

        let new_ino = get_unique_ino();
        if let Err(e) = parent.add_child(new_ino) {
            reply.error(e);
            return;
        }

        let new_user_file = UserFile::new(name, new_ino, 0, mode, req.uid(), req.gid(), 0);
        data.inos.insert(new_ino, Ino::File(Box::new(new_user_file)));

        reply.entry(&DEFAULT_CACHE, data.inos.get(&new_ino).unwrap().attr(), 0);
    }

    fn mkdir(
            &mut self,
            req: &Request<'_>,
            parent: u64,
            name: &std::ffi::OsStr,
            mode: u32,
            umask: u32,
            reply: fuser::ReplyEntry,
        ) {
        println!("mkdir: {parent} name: {name:?}");
        let mut data = DATA.lock().unwrap();
        let parent = data.inos.get_mut(&parent).unwrap().unwrap_dir_mut();
        let is_user_dir = parent.is_user_dir() || name.as_bytes()[0] == b'_';
        if !parent.is_user_dir() && name.as_bytes()[0] != b'_' {
            reply.error(PERMISSION_DENIED);
            return;
        }

        let new_ino = get_unique_ino();
        if let Err(e) = parent.add_child(new_ino) {
            reply.error(e);
            return;
        }

        let new_dir = NormalDir::new(name, is_user_dir, new_ino, mode, req.uid(), req.gid(), 0);
        data.inos.insert(new_ino, Ino::Dir(Box::new(new_dir)));

        reply.entry(&DEFAULT_CACHE, data.inos.get(&new_ino).unwrap().attr(), 0);
    }

    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        let mut data = DATA.lock().unwrap();
        let child = data.inos.get(&parent).unwrap().unwrap_dir().lookup_child(name, &data.inos).unwrap();
        if let Err(e) = data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().remove_child(child) {
            reply.error(e);
            return;
        }

        if let Err(e) = data.inos.get_mut(&child).unwrap().delete() {
            if data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().add_child(child).is_err() {
                println!("Warning: unlink lost a file");
            }
            reply.error(e);
            return;
        }

        data.inos.remove(&child);
        reply.ok();
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &std::ffi::OsStr, reply: fuser::ReplyEmpty) {
        println!("rmdir: {parent} {name:?}");
        let mut data = DATA.lock().unwrap();
        let child_ino = match data.inos.get(&parent).unwrap().unwrap_dir().lookup_child(name, &data.inos) {
            Ok(child_ino) => child_ino,
            Err(e) => {
                reply.error(e);
                return;
            }
        };
        if let Err(e) = data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().remove_child(child_ino) {
            reply.error(e);
        } else {
            reply.ok();
        }
    }

    // fn symlink(
    //         &mut self,
    //         _req: &Request<'_>,
    //         parent: u64,
    //         link_name: &std::ffi::OsStr,
    //         target: &std::path::Path,
    //         reply: fuser::ReplyEntry,
    //     ) {
        
    // }

    fn rename(
            &mut self,
            _req: &Request<'_>,
            parent: u64,
            name: &std::ffi::OsStr,
            newparent: u64,
            newname: &std::ffi::OsStr,
            flags: u32,
            reply: fuser::ReplyEmpty,
        ) {
        println!("Rename: par: {parent} name: {name:?} newparent: {newparent} newname: {newname:?}");
        let mut data = DATA.lock().unwrap();   
        let child_ino = match data.inos.get(&parent).unwrap().unwrap_dir().lookup_child(name, &data.inos) {
            Ok(child_ino) => child_ino,
            Err(e) => {
                reply.error(e);
                return;
            }
        };

        let new_dir_is_user_dir = data.inos.get(&parent).unwrap().unwrap_dir().is_user_dir();

        if let Err(e) = data.inos.get_mut(&child_ino).unwrap().rename(newname, new_dir_is_user_dir) {
            reply.error(e);
            return;
        }
        if let Err(e) = data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().remove_child(child_ino) {
            reply.error(e);
            return;
        }
        if let Err(e) = data.inos.get_mut(&newparent).unwrap().unwrap_dir_mut().add_child(child_ino) {
            reply.error(e);
            if data.inos.get_mut(&parent).unwrap().unwrap_dir_mut().add_child(child_ino).is_err() {
                println!("Warning: Rename dropped a file");
            }
            return;
        }
        
        reply.ok();
    }

    // fn link(
    //         &mut self,
    //         _req: &Request<'_>,
    //         ino: u64,
    //         newparent: u64,
    //         newname: &std::ffi::OsStr,
    //         reply: fuser::ReplyEntry,
    //     ) {
        
    // }

    // fn open(&mut self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
    // }

    fn read(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            fh: u64,
            offset: i64,
            size: u32,
            flags: i32,
            lock_owner: Option<u64>,
            reply: fuser::ReplyData,
        ) {
        println!("Read: {ino} off: {offset} size: {size}");
        let mut data = DATA.lock().unwrap();
        let file = data.inos.get_mut(&ino).unwrap();
        if let Some(file) = file.try_unwrap_file_mut() {
            match file.read(offset, size, flags) {
                Ok(data) => reply.data(data),
                Err(e) => reply.error(e),
            }
        } else {
            reply.error(NOT_SUPPORTED);
        }
    }

    fn write(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            fh: u64,
            offset: i64,
            write_data: &[u8],
            write_flags: u32,
            flags: i32,
            lock_owner: Option<u64>,
            reply: fuser::ReplyWrite,
        ) {
        println!("Write: {ino} off: {offset} len: {}", write_data.len());
        let mut data = DATA.lock().unwrap();
        let file = data.inos.get_mut(&ino).unwrap();
        if let Some(file) = file.try_unwrap_file_mut() {
            match file.write(offset, write_data, write_flags, flags) {
                Ok(amount) => reply.written(amount),
                Err(e) => reply.error(e),
            }
        } else {
            reply.error(NOT_SUPPORTED);
        }
    }

    // fn flush(&mut self, _req: &Request<'_>, ino: u64, fh: u64, lock_owner: u64, reply: fuser::ReplyEmpty) {
        
    // }

    // fn release(
    //         &mut self,
    //         _req: &Request<'_>,
    //         _ino: u64,
    //         _fh: u64,
    //         _flags: i32,
    //         _lock_owner: Option<u64>,
    //         _flush: bool,
    //         reply: fuser::ReplyEmpty,
    //     ) {
        
    // }

    // fn fsync(&mut self, _req: &Request<'_>, ino: u64, fh: u64, datasync: bool, reply: fuser::ReplyEmpty) {
        
    // }

    // fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        
    // }

    fn readdir(
            &mut self,
            _req: &Request<'_>,
            ino: u64,
            fh: u64,
            offset: i64,
            mut reply: fuser::ReplyDirectory,
        ) {
        println!("Read Dir: {ino} off:{offset}");
        let data = DATA.lock().unwrap();
        let dir = data.inos.get(&ino).unwrap().unwrap_dir();
        
        let Ok(mut offset) = offset.try_into() else {
            reply.error(NOT_SUPPORTED);
            return;
        };
        loop {
            let Some(child) = dir.get_child(offset) else {
                println!("Break because not found");
                break;
            };
            let child_ino = data.inos.get(&child).unwrap();
            if reply.add(child, offset as i64 + 1, child_ino.attr().kind, child_ino.name()) {
                println!("Full");
                break;
            }
            println!("Added");
            offset += 1;
        }
        reply.ok();
    }

    // fn releasedir(
    //         &mut self,
    //         _req: &Request<'_>,
    //         _ino: u64,
    //         _fh: u64,
    //         _flags: i32,
    //         reply: fuser::ReplyEmpty,
    //     ) {
        
    // }

    // fn fsyncdir(
    //         &mut self,
    //         _req: &Request<'_>,
    //         ino: u64,
    //         fh: u64,
    //         datasync: bool,
    //         reply: fuser::ReplyEmpty,
    //     ) {
        
    // }



}

fn get_unique_ino() -> u64 {
    NEXT_INO.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}