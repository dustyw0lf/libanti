use libanti::debug::is_ptraced;

fn main() {
    if is_ptraced().unwrap() {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
