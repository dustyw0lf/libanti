use core::arch::asm;
use std::fs;

use crate::error::{Error, Result};

pub fn is_traced() -> Result<bool> {
    let status = fs::read_to_string("/proc/self/status")?;

    for line in status.lines() {
        if line.contains("TracerPid") {
            let status = line
                .split_whitespace()
                .last()
                .ok_or_else(|| Error::ProcFsParse("missing TracerPid value".to_string()))?
                .parse::<isize>()
                .map_err(|_| Error::ProcFsParse("invalid TracerPid value".to_string()))?;
            if status != 0 {
                return Ok(true);
            }
        };
    }

    Ok(false)
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

/// Detects if a debugger is present by calling the ptrace syscall directly.
///
/// Returns `Ok(true)` if a debugger is detected and `Ok(false)` if no debugger is present.
/// The function is currently infallible, but returns `Result` for consistency with other functions.
pub fn is_ptraced() -> Result<bool> {
    let res = unsafe { syscall_ptrace(0, 0, 0, 0) };

    // If the process was already being traced, return true
    // If the process wasn't already being traced, return false
    Ok(res != 0)
}
