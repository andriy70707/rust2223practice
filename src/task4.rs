#[test]
fn main() {
    let x = define_x();  // Отримуємо значення x з функції define_x
    println!("{}, world", x);
}

fn define_x() -> &'static str {  // Функція повертає значення типу &'static str
    let x = "hello";
    x  // Повертаємо значення x
}
