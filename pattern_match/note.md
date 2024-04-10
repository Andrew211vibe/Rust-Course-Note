### `match`和 `if let`

##### `match`

```
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::West => println!("West"),
        Direction::South | Direction::North => println!("either South or North"),
        _ => println!("East"),
    };
}
```

* `match` 的匹配必须要穷举出所有可能，因此这里用 `_` 来代表未列出的所有可能性
* `match` 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
* **X | Y** ，类似逻辑运算符 `或`，代表该分支可以匹配 `X` 也可以匹配 `Y`，只要满足一个即可
* 如果分支有多行代码，那么需要用 `{}` 包裹，同时最后一行代码需要是一个表达式

> 当 `match` 表达式执行时，它将目标值按顺序依次与每一个分支的模式相比较，如果模式匹配了这个值，那么模式之后的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支

##### 使用 `match`表达式赋值

- `match` 本身也是一个表达式，因此可以用它来赋值

```rust
enum IpAddr {
   Ipv4,
   Ipv6
}

fn main() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);
}
```

##### 模式绑定

- 从模式中取出绑定的值

```rust
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```

##### 穷尽匹配

>  `match`匹配必须穷尽所有情况，若有情况为进行处理，则会发生编译期错误

##### `_`通配符

- 通过将 _ 其放置于其他分支后，_ 将会匹配所有遗漏的值

```rust
// () 表示返回单元类型与所有分支返回值的类型相同，所以当匹配到 _ 后，什么也不会发生
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

- 用一个变量来承载其他情况

```rust
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };
}
```

#### `if let`匹配

- 只关心一个值是否存在 -- 只有一个模式的值需要被处理，其它值直接忽略的场景

```rust
// match
fn main() {
    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
}

// if let
fn main() {
    if let Some(3) = v {
        println!("three");
    }
}
```

> **当你只要匹配一个条件，且忽略其他条件时就用 `if let` ，否则都用 `match`**

#### `matches!`宏

- 将一个表达式跟模式进行匹配，然后返回匹配的结果 `true` or `false`

```rust
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    println!("before filter: {:?}", v);
    // v.iter().filter(|x| x == MyEnum::Foo); // Error:无法将 x 直接跟一个枚举成员进行比较
    let res: Vec<_> = v.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("after filter: {:?}", res);
  
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));
}
```

#### 变量遮蔽

无论是 `match` 还是 `if let`，这里都是一个新的代码块，而且这里的绑定相当于新变量，如果你使用同名变量，会发生**变量遮蔽**：

```rust
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   if let Some(age) = age {
       println!("匹配出来的age是{}",age);
   }

   println!("在匹配后，age是{:?}",age);
}

fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   match age {
       Some(age) =>  println!("匹配出来的age是{}",age),
       _ => ()
   }
   println!("在匹配后，age是{:?}",age);
}
```

- **`match` 中的变量遮蔽其实不是那么的容易看出**，最好不要使用同名

### 解构`Option`

**`Option` 枚举：一个变量要么有值：`Some(T)`, 要么为空：`None`**

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### 模式使用场景

模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据

- 可在match、if let、while lt、for、fn、let中使用

```rust
// match
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}

// if let
if let PATTERN = SOME_VALUE { ... }

// while let，只要模式匹配就一直进行 while 循环
fn main() {
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
 
    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// for
// enumerate方法产生一个迭代器，该迭代器每次迭代会返回一个(索引，值)形式的元组，通过(index,value)来匹配
let v = vec!['a', 'b', 'c'];
for (index, value) in v.iter().enumerate() { // 
    println!("{} is at index {}", value, index);
}

// let
let PATTERN = EXPRESSION;
// 变量名也是一种模式，因此赋值语句是一种模式绑定，将匹配的值绑定到变量 x 上
let x = 5;
// 上面将一个元组与模式进行匹配(模式和值的类型必需相同！)
let (x, y, z) = (1, 2, 3);

// fn函数参数
fn foo(x: i32) { ... } // 参数x即是一种模式

fn print_coordinates(&(x, y): &(i32, i32)) { // &(3, 5) 会匹配模式 &(x, y)
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

#### `if`和`let if`

- 类似 `let` , `for`和 `match` 都必须要求完全覆盖匹配，才能通过编译( 不可驳模式匹配 )
- 类似 `if let` 允许匹配一种模式，而忽略其余的模式( 可驳模式匹配 )

```rust
// 右边的值可能不为 `Some`，而是 `None`，这种时候就不能进行匹配，遗漏了 `None` 的匹配
let Some(x) = some_option_value;
```

### 全模式列表

#### 匹配字面值

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### 匹配命名变量

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // y变量遮蔽，绑定匹配任何Some中的值
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

不想引入变量遮蔽，可以使用另一个变量名而非 `y`，或者使用**匹配守卫**(match guard)的方式

#### 单分支多模式

使用 `|` 语法匹配多个模式

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### 通过 `..=`匹配值的范围

```rust
let x = 5;
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

let x = 'c';
match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

- 相比使用 `|` 运算符表达相同的意思更为方便
- 序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空
  - 字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型

#### 解构并分解值

使用模式来解构结构体、枚举、元组、数组和引用

##### 解构结构体

- **模式中的变量名不必与结构体中的字段名一致**
- 对于匹配结构体字段的模式存在简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    // let Point {x, y} = p;
    // assert_eq!(0, x);
    // assert_eq!(7, y);
}
```

- 使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量
  - 允许测试一些字段为特定值的同时创建其他字段的变量

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

##### 解构枚举

```rust
#[allow(dead_code)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 125, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y }=> println!("Move in the x direction {} and in the y direction {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red-{}, green-{}, blue-{}", r, g, b),
    }
}
```

##### 解构嵌套的结构体和枚举

```rust
enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v);
        }
        _ => ()
    }
}
```

##### 解构结构体和元组

```rust
struct Point {
     x: i32,
     y: i32,
 }

let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

##### 解构数组

```rust
// 定长数组
let arr: [u16; 2] = [114, 514];
let [x, y] = arr;
assert_eq!(x, 114);
assert_eq!(y, 514);

// 不定长数组
let arr: &[u16] = &[114, 514];
if let [x, ..] = arr {
    assert_eq!(x, &114);
}
if let &[.., y] = arr {
    assert_eq!(y, 514);
}
let arr: &[u16] = &[];
assert!(matches!(arr, [..]));
assert!(!matches!(arr, [x, ..]));
```

#### 忽略模式中的值

模式中使用 `_` 模式，使用一个以下划线开始的名称，或者使用 `..` 忽略所剩部分的值

##### 使用 `_`忽略整个值

```rust
fn foo(_: i32, y: i32) { // 忽略第一个参数x
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

- 不再需要特定函数参数时，最好修改签名不再包含无用的参数
  - 实现特征时，当你需要特定类型签名但是函数实现并不需要某个参数时，此时编译器就**不会警告说存在未使用的函数参数** ，就跟使用命名参数一样

##### 使用嵌套的_忽略部分值

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}

```

##### 使用下划线开头忽略未使用的变量

> 只使用 `_` 和使用以下划线开头的名称有些微妙的不同：比如 **`_x` 仍会将值绑定到变量，而 `_` 则完全不会绑定**

```rust
let s = Some(String::from("Hello!"));

if let Some(_s) = s { // s的所有权被转移给_s
    println!("found a string");
}
// if let Some(_) = s {
    // println!("found a string");
// }

println!("{:?}", s); // Error
```

##### 用 `..`忽略剩余值

- 对于有多个部分的值，可以使用 `..` 语法来只使用部分值而忽略其它值

  - `..` 模式会忽略模式中剩余的任何没有显式匹配的值部分

```rust
// 忽略结构体中的剩余字段
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

// 忽略元组中的某些值
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

- 使用 `..` 必须是无歧义的

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => { // Error:匹配歧义
            println!("Some numbers: {}", second)
        },
    }
}
```

#### 匹配守卫提供额外条件

> **匹配守卫** （ *match guard* ）是一个位于 `match` 分支模式之后的额外 `if` 条件，它能为分支模式提供更进一步的匹配条件

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

- 也可以在匹配守卫中使用 **或** 运算符 `|` 来指定多个模式，**同时匹配守卫的条件会作用于所有的模式**

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"), // (4 | 5 | 6) if y => ...
    _ => println!("no"),
}

```

#### `@`绑定

> `@`运算符允许为一个字段绑定另外一个变量
>
> 既想要限定分支范围，又想要使用分支的变量时，就可以用 `@` 来绑定到一个新的变量上

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => { // 捕获任何匹配3..=7的值并绑定到id_variable
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

- 使用 `@` 还可以在绑定新变量的同时，对目标进行解构

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);


    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
```

##### 新特性

```rust
fn main() {
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
```
