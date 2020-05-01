extern crate winapi;
extern crate user32;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::process::{Command, Stdio};

fn main() {
    let mut child_id = Command::new("notepad")
        .stderr(Stdio::null())
        .spawn()
        .expect("notepad command failed to start");

    let window_title = "Unbenannt - Editor";
    let wide: Vec<u16> = OsStr::new(window_title).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        user32::FindWindowW(null_mut(), wide.as_ptr())
    };
    if ret == null_mut() {
        println!("Failed: {:?}", Error::last_os_error());
    } else {
        println!("Found the window '{}': {:?}", window_title, ret);

        let is_closed = unsafe {
            user32::SendMessageW(ret, 0x0010, 0, 0)
        };

        if is_closed == 0 {
            println!("Window was closed: {}", window_title);
        }
    }

    let _ = child_id.wait();    
}
