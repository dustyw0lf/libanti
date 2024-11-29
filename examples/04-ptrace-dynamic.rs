use std::ffi::{c_int, c_long, c_uint, c_void};

use libloading::Symbol;

use linux_antidbg::utils::get_lib;

fn main() {
    let not_ptrace = get_not_ptrace().unwrap();

    if unsafe { not_ptrace(0 as *const c_uint, 0, 0 as *mut c_void, 0 as *mut c_void) } == -1 {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}

type NotPtraceFn = unsafe extern "C" fn(
    request: *const c_uint,
    pid: c_int,
    addr: *mut c_void,
    data: *mut c_void,
) -> c_long;

pub fn get_not_ptrace() -> Result<NotPtraceFn, Box<dyn std::error::Error>> {
    let lib = get_lib("libc.so.6")?;
    unsafe {
        let not_ptrace: Symbol<NotPtraceFn> = lib.get(b"ptrace\0")?;
        Ok(*not_ptrace.into_raw())
    }
}
