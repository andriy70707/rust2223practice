#[test]
fn main() {
    let x: u32 = 5;
    assert_eq!("u32", type_of(&x));

    println!("Успіх!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
