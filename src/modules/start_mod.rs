use std::{ffi::OsString, str::FromStr};

use crate::{background_tasks::{add_file, rm_file, DEFAULT_MODE}, file_helpers::str_to_vec, main_fs::get_unique_ino, special_files::trigger_file::TriggerFile};

use super::{bathroom, classroom, correct_order, find_the_suid, kill_them_all, many_open};



const WELCOME_MESSAGE: &str = 
"Hello and welcome to the linux challenge.
You may be wondering where the challenge is.
Well you are actually just blind so check again.
";

const WELCOME_MESSAGE_2: &str = 
"Huh that was weird.
You should probably get your eyes checked.
Anyways welcome to the challenge.
This is your main hub directory.
You can navagate using standard linux commands.
Each directory is a different challenge and is contained in that directory.
You are free to do them in any order.
You may have to code a custom program (in Rust if you want) to complete some challenges.
Remember that you may have to think outside the box to solve the challenge.
Things are not always as they seem.

If you want to make your own files, you can make any file with a name that starts with an _ to avoid conflicts.
Additionally, any directory that starts with an _ will allow you to make any files you want inside of it.
You can use this to make areas to use tools like GCC or cargo.
";

pub fn start() {
    add_file("", Box::new(TriggerFile::new(spawn_welcome_2, "Welcome", str_to_vec(WELCOME_MESSAGE), get_unique_ino(), DEFAULT_MODE, 0)));

    // start_mods();
}

fn spawn_welcome_2() {
    add_file("", Box::new(TriggerFile::new(start_mods, "Welcome?", str_to_vec(WELCOME_MESSAGE_2), get_unique_ino(), DEFAULT_MODE, 0)));
}

fn start_mods() {
    rm_file("Welcome");
    many_open::start();
    classroom::start();
    correct_order::start();
    //bathroom::start();
    kill_them_all::start();
}
