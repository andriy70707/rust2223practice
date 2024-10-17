#[test]
fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}


//Код викликає функцію define_x(), яка повертає рядок "hello".
// Цей рядок присвоюється змінній x, після чого виводиться у форматі "hello, world"