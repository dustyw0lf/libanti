use core::arch::asm;
use std::ffi::{c_int, c_long, c_uint, c_void};

use libloading::Symbol;

use crate::utils::get_lib;

type PtraceFn = unsafe extern "C" fn(
    request: *const c_uint,
    pid: c_int,
    addr: *mut c_void,
    data: *mut c_void,
) -> c_long;

pub fn get_ptrace() -> Result<PtraceFn, Box<dyn std::error::Error>> {
    let lib = get_lib("libc.so.6")?;
    unsafe {
        let not_ptrace: Symbol<PtraceFn> = lib.get(b"ptrace\0")?;
        Ok(*not_ptrace.into_raw())
    }
}

unsafe fn syscall_ptrace(request: usize, pid: usize, addr: usize, data: usize) -> isize {
    // Based on
    // https://github.com/jasonwhite/syscalls/blob/main/src/syscall/x86_64.rs
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") 101 as usize => ret, // ptrace syscall number
        in("rdi") request,                    // arg 1
        in("rsi") pid,                        // arg 2
        in("rdx") addr,                       // arg 3
        in("r10") data,                       // arg 4
        out("rcx") _,                         // rcx is used to store old rip
        out("r11") _,                         // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret as isize
}
