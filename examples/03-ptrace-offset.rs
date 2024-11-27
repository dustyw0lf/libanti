use libc::{ptrace, PTRACE_TRACEME};

fn main() {
    let mut offset = 0;

    if unsafe { ptrace(PTRACE_TRACEME, 0, 0, 0) } == 0 {
        offset += 2;
    }

    if unsafe { ptrace(PTRACE_TRACEME, 0, 0, 0) } == -1 {
        offset *= 3;
    }

    if offset == 2 * 3 {
        println!("Normal");
    } else {
        println!("Debugger");
    }
}
