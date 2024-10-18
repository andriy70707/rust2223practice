#[test]
fn main() {
    let c1 = "中";
    if let Some(c) = c1.chars().next() {
        print_char(c);
    } else {
        println!("Рядок порожній.");
    }
}

fn print_char(c: char) {
    println!("{}", c);
}
