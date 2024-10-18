#[test]
fn main() {
    let mut sum = 0;
    for i in -3..=1 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}


