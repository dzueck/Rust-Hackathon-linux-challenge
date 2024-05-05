use fuser::{mount2, MountOption};

pub const MOUNT_POINT: &str = "/home/dylan/c";
fn main() {
    println!("Starting logger");
    // set_logger(&SIMPLE_LOGGER).expect("Faied to setup logger");
    println!("set logger");

    let fs = main_fs::MainFs::new();
    println!("mounting");
    
    // warn!("Warning");

    mount2(fs, MOUNT_POINT, &[
        MountOption::AllowOther,
        MountOption::AutoUnmount,
        MountOption::Exec,
        MountOption::NoAtime,
    ]).expect("Failed to mount fs");
}

mod main_fs;
mod files;
mod dirs;
mod errors;
mod link;
mod user_files;
mod special_files;
mod file_helpers;
mod modules;
mod background_tasks;