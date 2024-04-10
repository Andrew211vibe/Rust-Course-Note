// #[allow(unused_variables)]
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("{}", s1);
// }

// fn main() {
//     let s = String::from("hello");  // s 进入作用域

//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效
//     println!("在move进函数后使用s: {}", s); // 报错无效的引用

//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动到函数里，
//                                     // 但 i32 是 Copy 的，所以在后面可继续使用 x

// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership 将返回值
//                                         // 移给 s1

//     let s2 = String::from("hello");     // s2 进入作用域

//     let s3 = takes_and_gives_back(s2);  // s2 被移动到
//                                         // takes_and_gives_back 中,
//                                         // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
//                                              // 调用它的函数

//     let some_string = String::from("hello"); // some_string 进入作用域.

//     some_string                              // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

//     a_string  // 返回 a_string 并移出给调用的函数
// }


fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
    // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age 
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
    //println!("The person struct is {:?}", person);

    // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
    println!("The person's age from person struct is {}", person.age);
}