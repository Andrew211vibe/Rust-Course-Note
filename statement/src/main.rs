fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn ret_unit_type() {
    let x = 1;
    // if语句块也是表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符?:
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}

fn main() {
    println!("{}", add_with_extra(0, 0));
    
    // 以下都是语句
    let a = 8; // 8是表达式，它在求值后返回8
    let b: Vec<f64> = Vec::new();
    let (a, c) = ("hi", false);

    // let是语句，因此let不能赋值给其它值
    // let a = (let b = 0);

    let y = {
        let x = 3;
        x + 1
    }; // 表达式语句块，返回值x + 1
    println!("The value of y is: {}", y);

    // 表达式若不返回任何值，会隐式地返回一个()
    assert_eq!(ret_unit_type(), ());

    let x = 5;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 下方表达式的返回值将赋值给y
        x_cube + x_squared + x
    };

    let z = {
        // 分号将表达式变为语句，返回的不再是2 * x的值，而是语句的值()
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
