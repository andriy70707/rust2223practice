#[test]
fn main() {
    let mut x: i32 = 1;
    x = 7;
    let x = x;

    let y = 4;
    // Затінення
    let y = "Я також можу бути прив'язаний до тексту!";

    println!("Success!");
}
