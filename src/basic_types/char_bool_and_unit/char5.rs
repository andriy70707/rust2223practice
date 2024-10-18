#[test]
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2, 3));

    implicitly_ret_unit();

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}
