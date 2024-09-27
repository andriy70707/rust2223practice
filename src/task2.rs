#[test]
fn main() {
    let mut x = 1; // Позначаємо змінну x як змінну (mut)
    x += 2; // Змінюємо значення x

    assert_eq!(x, 3);
    println!("Success!");
}
