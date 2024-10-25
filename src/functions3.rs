#[test]
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    panic!("Данна функція ніколи не повернеться!");
}
