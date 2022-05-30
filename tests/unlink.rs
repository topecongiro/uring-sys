use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

use libc::AT_FDCWD;
use uring_sys2::*;

use common::strerror;

mod common;

#[test]
fn unlink() {
    unsafe {
        let mut ring = MaybeUninit::uninit();
        assert_eq!(io_uring_queue_init(1, ring.as_mut_ptr(), 0), 0);
        let mut ring = ring.assume_init();

        let f = tempfile::NamedTempFile::new().unwrap();
        assert!(f.path().exists());

        let ret = test_unlink(&mut ring, f.path());
        if ret < 0 {
            if ret == -libc::EBADF || ret == -libc::EINVAL {
                eprintln!("unlink not supported, skipping\n");
                return;
            } else {
                panic!("rename: {}\n", strerror(-ret));
            }
        }
        assert!(!f.path().exists());

        assert_eq!(
            test_unlink(&mut ring, Path::new("/3/2/3/1/z/y")),
            -libc::ENOENT
        );
    }
}

unsafe fn test_unlink(ring: &mut io_uring, path: &Path) -> libc::__s32 {
    let mut sqe = io_uring_get_sqe(ring);
    assert!(!sqe.is_null());
    io_uring_prep_unlinkat(sqe, 0, path.as_os_str().as_bytes().as_ptr() as _, 0);
    assert!(io_uring_submit(ring) >= 0);

    let mut cqe = ptr::null_mut();
    let ret_wait = io_uring_wait_cqe(ring, &mut cqe);
    assert!(ret_wait >= 0, "wait completion {}", ret_wait);
    let ret = (*cqe).res;
    io_uring_cqe_seen(ring, cqe);
    return ret;
}
