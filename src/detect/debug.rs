use core::arch::asm;
use std::ffi::{c_int, c_long, c_uint, c_void};
use std::fs;

use libloading::Symbol;

use crate::utils::get_lib;

pub fn is_traced() -> Result<bool, Box<dyn std::error::Error>> {
    let status = fs::read_to_string("/proc/self/status").unwrap();

    for line in status.lines() {
        if line.contains("TracerPid") {
            let status = line
                .split_whitespace()
                .last()
                .ok_or_else(|| "Error getting tracer pid")?
                .parse::<isize>()?;
            if status != 0 {
                return Ok(true);
            }
        };
    }

    Ok(false)
}

type PtraceFn = unsafe extern "C" fn(
    request: *const c_uint,
    pid: c_int,
    addr: *mut c_void,
    data: *mut c_void,
) -> c_long;

pub fn get_ptrace() -> Result<PtraceFn, Box<dyn std::error::Error>> {
    let lib = get_lib("libc.so.6")?;
    unsafe {
        let ptrace: Symbol<PtraceFn> = lib.get(b"ptrace\0")?;
        Ok(*ptrace.into_raw())
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

pub fn is_ptraced_syscall() -> Result<bool, Box<dyn std::error::Error>> {
    let res = unsafe { syscall_ptrace(0, 0, 0, 0) };
    if res == 0 {
        // If the process wasn't already being traced, return false
        Ok(false)
    } else {
        // If the process was being traced, return true
        Ok(true)
    }
}
