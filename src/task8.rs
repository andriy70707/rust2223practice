#[test]
fn main() {
    let (mut x, y) = (1, 2);  // Зробимо x змінною

    x += 2;  // Тепер можна змінити значення x

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
