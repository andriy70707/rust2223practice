#[test]
fn main() {
    let x: i32 = 10;

    {
        let y: i32 = 5;
        println!("x: {}, y: {}", x, y);
    }

    println!("x: {}", x);
}


//Код оголошує змінну x (10) для всієї функції і y (5) всередині блоку {}.
// У блоці обидві змінні виводяться, після блоку доступна лише x.