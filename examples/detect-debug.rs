use libanti::debug::is_traced;

fn main() {
    if is_traced().unwrap() {
        println!("Debugger");
    } else {
        println!("Normal");
    }
}
