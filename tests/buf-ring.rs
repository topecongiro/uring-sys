use std::alloc;
use uring_sys2::*;

use crate::common::{io_uring_init, strerror};

mod common;

enum TestResult {
    Ok,
    NotSupported,
}

#[test]
fn test_buf_ring() {
    let bgids = [1, 127, -1];

    for bgid in bgids {
        match unsafe { test_reg_unreg(bgid) } {
            TestResult::Ok => (),
            TestResult::NotSupported => {
                eprintln!("buffer ring not supported, skipping");
                return;
            }
        }
    }
}

unsafe fn test_reg_unreg(bgid: i32) -> TestResult {
    let mut ring = io_uring_init(1).unwrap();

    let ptr = alloc::alloc(alloc::Layout::from_size_align_unchecked(4096, 4096));
    assert!(!ptr.is_null());

    let mut reg = io_uring_buf_reg {
        ring_addr: ptr as usize as u64,
        ring_entries: 32,
        bgid: bgid as u16,
        pad: 0,
        resv: [0; 3],
    };
    let ret = io_uring_register_buf_ring(&mut ring, &mut reg, 0);
    if ret != 0 {
        if ret == -libc::EINVAL {
            return TestResult::NotSupported;
        }
        panic!("buffer ring register failed: {}", strerror(-ret));
    }

    let ret = io_uring_unregister_buf_ring(&mut ring, bgid);
    if ret != 0 {
        panic!("buffer ring register failed: {}", -ret);
    }

    io_uring_queue_exit(&mut ring);

    return TestResult::Ok;
}
