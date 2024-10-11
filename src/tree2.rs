#[test]
fn draw_trees() {
    let mut height = 3;
    for tree in 0..3 {
        for i in 0..height {
            let spaces = " ".repeat(height - i - 1);
            let stars = "*".repeat(2 * i + 1);
            println!("{}{}", spaces, stars);
        }
        println!("");
        height += 1;
    }
}
