use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use std::thread::sleep;
use libc::c_char;
use std::ffi::CString;

fn main() {
    unsafe {
        libc::fopen(CString::new("stall_3/Toilet").unwrap().as_ptr(), 0 as *const c_char);
    }
    let mut file = File::options().read(true).write(true).open("stall_3/Toilet").unwrap();

    
    file.flush();
    sleep(Duration::from_secs(4));
}