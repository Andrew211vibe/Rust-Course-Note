fn r() -> &'static str {
    "hello"
}

fn main() {
    println!("Hello, world!");

    let x = r();
    println!("{}", x);
}
