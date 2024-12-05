use libanti::debug::is_ptraced_syscall;

fn main() {
    if is_ptraced_syscall().unwrap() {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
