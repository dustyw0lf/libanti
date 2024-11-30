use libanti::debug::is_ptraced_dynamic;

fn main() {
    if is_ptraced_dynamic().unwrap() {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
