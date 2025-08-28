fn main() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    return "hello".to_string();
}
