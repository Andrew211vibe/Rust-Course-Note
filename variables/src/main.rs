// fn main() {
//     // 变量绑定（所有权），可变与不可变
//     // let a = 5;
//     let mut a = 5;
//     println!("The value of a is {}", a);
//     a = 6; // Error: cannot assign twice to immutable variable `a`
//     println!("The value of a is {}", a);

//     // _开头命名变量标识未使用变量，Rust不会警告
//     // let _x = 1;
//     // let y = 2;

//     // 变量解构
//     let (x, mut y) : (bool, bool) = (true, false);
//     // x = true不可变，y = false可变
//     println!("x = {:?}, y = {:?}", x, y);

//     y = true;
//     assert_eq!(x, y);
// }

// 解构式赋值
// struct Struct {
//     e: i32
// }

// fn main() {
//     let (a, b, c, d, e);

//     (a, b) = (1, 2);
//     // _代表匹配一个值，但并不关心具体的值
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct {e, ..} = Struct {e: 5};

//     assert_eq!([1, 2, 3, 4, 5], [a, b, c, d, e]);
// }

// 变量遮蔽
fn main() {
    let x = 5;
    // 在main函数的作用域之内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前花括号内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }
    
    println!("The value of x is {}", x);

    let mut spaces = "    ";
    // spaces = spaces.len();
}

