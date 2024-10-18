#[test]
fn main() {
    let epsilon = 0.00001_f64;
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < epsilon);

    println!("Success!");
}
