use std::ffi::CString;
use std::io;
use std::mem::MaybeUninit;

use uring_sys2::{io_uring, io_uring_queue_init};

pub fn strerror(code: i32) -> String {
    unsafe {
        CString::from_raw(libc::strerror(code))
            .into_string()
            .unwrap()
    }
}

pub fn io_uring_init(entries: usize) -> io::Result<io_uring> {
    unsafe {
        let ring = MaybeUninit::uninit();
        let ret = io_uring_queue_init(entries as _, ring.as_ptr() as _, 0);
        if ret != 0 {
            Err(io::Error::from_raw_os_error(-ret))
        } else {
            Ok(ring.assume_init())
        }
    }
}
