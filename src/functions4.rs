#[test]
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42),
        _ => None,
    }
}

fn never_return_fn() -> ! {
    loop {}
}
