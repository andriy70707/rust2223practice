#[test]
fn main() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(120, 7).unwrap();
    println!("{},{}", v1, v2);
}