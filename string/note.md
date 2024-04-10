### 字符串和切片

注意：`&str`和 `String`两个类型的区别

```rust
fn main() {
    let str = "hello"; // &str类型
    greet(str);
}

fn greet(name: String) { // 报错：函数参数类型为String，接收到的是&str类型
    println!("say my name: {}", name);
}
```

#### 切片

- 切片就是对 `String`类型中某一部分的引用
- 创建切片：`str[开始索引..终止索引]`

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

> 切片的索引必须落在字符之间的边缘位置，也就是UTF-8字符的边界（中文在UTF-8中占用三个字节）
>
> 若字符截取不完整将会直接崩溃退出
>
> ```rust
> let s = "中国人";
> // let a = &s[0..2]; // 崩溃退出
> let a = &s[0..3];
> println!("{}",a);
> ```

- 字符串切片类型标识为 `&str`

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // error!
    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}

/*
pub fn clear(&mut self)调用生成s的可变引用
与先前first_world函数返回s的切片不可变引用word冲突
*/
```

- 数组切片和字符串切片的工作方式是一样的

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

#### 字符串字面量是切片

- 指向内存中程序可执行文件的某个内存位置 -- 不可变引用

#### 字符串

- 字符串是由字符组成的连续集合
- **Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间**
- **字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)**
- `str` 类型是硬编码进可执行文件，也无法被修改，但是 `String` 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串

#### String与&str转换

```rust
// &str转String
String::from("hello,world")
"hello,world".to_string()
```

- `String`转 `&str`直接对 `String`取引用（切片）即可，这样的灵活使用是因为 `deref` 隐式强制转换

#### 字符串索引

- 因为字符串中字符所占的字节数是变化的，所以对字符串使用索引会导致报错
- 还有一个原因导致了 Rust 不允许去索引字符串：因为索引操作，我们总是期望它的性能表现是 O(1)，然而对于 `String` 类型来说，无法保证这一点，因为 Rust 可能需要从0开始去遍历字符串来定位合法位置
- Rust 提供了不同的字符串展现方式，这样程序可以挑选自己想要的方式去使用，而无需去管字符串从人类语言角度看长什么样

#### 操作字符串

##### 追加（push）

- `push()`方法追加 `char`，`push_str()`方法追加 `&str`
- **在原有的字符串上追加，并不会返回新的字符串**
- 由于字符串追加操作要修改原来的字符串，**字符串变量必须由 `mut` 关键字修饰**

```rust
fn main() {
    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);
}
```

##### 插入（insert）

- `insert()` 方法插入单个字符 `char`，也可以使用 `insert_str()` 方法插入 `&str`
  - 第一个参数是字符（串）插入位置的索引，第二个参数是要插入的字符（串），索引从 0 开始计数，如果越界则会发生错误
- 由于字符串插入操作要**修改原来的字符串** ，**字符串变量必须由 `mut` 关键字修饰**

```rust
fn main() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
```

##### 替换（replace）

1. `replace()`方法

   1. 适用于 `String` 和 `&str`
   2. 接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串
   3. 替换所有匹配到的字符串。**该方法是返回一个新的字符串，而不是操作原来的字符串**

      ```rust
      fn main() {
          let string_replace = String::from("I like rust. Learning rust is my favorite!");
          let new_string_replace = string_replace.replace("rust", "RUST");
          dbg!(new_string_replace);
      }
      ```
2. `replacen()`方法

   1. 适用于 `String` 和 `&str`
   2. 接收三个参数，前两个参数与 `replace()` 方法一样，第三个参数则表示替换的个数
   3. **该方法是返回一个新的字符串，而不是操作原来的字符串**

      ```rust
      fn main() {
          let string_replace = "I like rust. Learning rust is my favorite!";
          let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
          dbg!(new_string_replacen);
      }
      ```
3. `replace_range()`方法

   1. 仅适用于 `String`
   2. 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串
   3. **方法是直接操作原来的字符串，不会返回新的字符串，需要使用 `mut` 关键字修饰**

      ```rust
      fn main() {
          let mut string_replace_range = String::from("I like rust!");
          string_replace_range.replace_range(7..8, "R");
          dbg!(string_replace_range);
      }
      ```

##### 删除（delete）

- `pop()`，`remove()`，`truncate()`，`clear()`
  - 仅适用于 `String`

```rust
/*
pop() - 删除并返回字符串的最后一个字符
直接操作原来的字符串
返回值是一个 Option 类型，如果字符串为空，则返回None
*/
fn main() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

/*
remove() - 删除并返回字符串中指定位置的字符
直接操作原来的字符串
返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置
按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
*/
fn main() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}

/*
truncate - 删除字符串中从指定位置开始到结尾的全部字符
直接操作原来的字符串，无返回值
按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
*/
fn main() {
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

/*
clear - 清空字符串
直接操作原来的字符串
*/
fn main() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}
```

##### 连接concatenate

```rust
/*
1. 使用+或者+=运算符连接字符串
	要求右边的参数必须为字符串的切片引用（Slice）类型
	调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法
	    fn add(self, s: &str) -> String
	+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰
2. 使用format!连接字符串
	适用于 String 和 &str，与println!用法类似
*/
fn main() {
    // 1.
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";
    println!("连接字符串 + -> {}", result);

    // 2.
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}
```

- 因为 `add()`方法的定义 `fn add(self, s: &str) -> String`，调用者的所有权将会传递进 `add()`方法中，`add()`执行结束后释放

```rust
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let s = s1 + "-" + &s2 + "-" + &s3;
}
```

#### 字符串转义

```rust
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);


    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

#### 操作UTF-8字符串

以Unicode方式遍历字符串

```rust
for c in "中国人".chars() {
    println!("{}", c);
}
```

以字节方式遍历字符串

```rust
for b in "中国人".bytes() {
    println!("{}", b);
}
```

#### 深度剖析

> 就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性
>
> 对于 `String` 类型，为了支持一个可变、可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容，这些都是在程序运行时完成的：
>
> * 首先向操作系统请求内存来存放 `String` 对象
> * 在使用完成后，将内存释放，归还给操作系统
>
> 其中第一部分由 `String::from` 完成，它创建了一个全新的 `String`
>
> 第二部分中，有**垃圾回收 GC** 的语言中，GC 来负责标记并清除这些不再使用的内存对象，这个过程都是自动完成，无需开发者关心，非常简单好用；但是在无 GC 的语言中，需要开发者手动去释放这些内存对象，就像创建对象需要通过编写代码来完成一样，未能正确释放对象造成的后果简直不可估量
>
> Rust中，变量在离开作用域后，就自动释放其占用的内存
>
> Rust 也提供了一个释放内存的函数： `drop`，但是不同的是，其它语言要手动调用 `free` 来释放每一个变量占用的内存，而 Rust 则在变量离开作用域时，自动调用 `drop` 函数
>
> ```rust
> {
>     let str: String = String::from("hello"); // 从此处起str有效
> } // 在这之后str无效，自动调用drop释放str
> ```

### 元组

> 元组使用多个括号将多个类型组合到一起（固定顺序长度

```rust
fn main() {
    let tup: (i32, f64, u8) = (666, 3.14, 1);
}
```

#### 用模式匹配解构元组

> 解构：用同样的形式把一个复杂对象中的值匹配出来

```rust
fn main() {
    let tup = (666, 3.14, 1);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}
```

#### **用 `.`访问元组**

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

#### 元组作为函数返回值使用

使用元组作为函数返回值返回多个

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
```

### 结构体

#### 定义结构体

一个结构体由几部分组成：

* 通过关键字 `struct` 定义
* 一个清晰明确的结构体**名称**
* 几个有名字的结构体**字段**

#### 结构体类型

1. 元组结构体

   ```rust
   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);

   let black = Color(255, 0, 0);
   let origin = Point(1, 2, 3);
   ```

   - 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用
2. C结构体

   ```rust
   struct User {
       active: bool,
       username: String,
       email: String,
       sign_in_count: u64,
   }
   ```
3. 单元结构体

   ```rust
   struct AlwaysEqual;

   let subject = AlwaysEqual;

   // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
   impl SomeTrait for AlwaysEqual { /*...*/ }
   ```

   - 没有任何字段和属性
   - 如果你定义一个类型，但是不关心该类型的内容，只关心它的行为时使用

#### 创建结构体实例

1. 初始化实例时，**每个字段**都需要进行初始化
2. 初始化时的字段顺序**不需要**和结构体定义时的顺序一致

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

#### 访问结构体字段

通过 `.` 操作符即可访问结构体实例内部的字段值：

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

- 必须要将结构体实例声明为可变的，才能修改其中的字段
  - 因为Rust不支持将某个结构体某个字段标记为可变

#### 结构体更新

根据已有的结构体实例，创建新的结构体实例，例如根据已有的 `user1` 实例来构建 `user2`：

```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

- 因为 `user2` 仅仅在 `email` 上与 `user1` 不同，因此我们只需要对 `email` 进行赋值，剩下的通过结构体更新语法 `..user1` 即可完成
- `..` 语法表明凡是我们没有显式声明的字段，全部从 `user1` 中自动获取。需要注意的是 `..user1` 必须在结构体的尾部使用

> `user1` 的部分字段所有权被转移到 `user2` 中：`username` 字段发生了所有权转移（未实现 `Copy`特征），作为结果，`user1` 无法再被使用
>
> `active` 和 `sign_in_count` 字段在赋值给 `user2` 时，仅仅发生了拷贝（实现了 `Copy`特征），而不是所有权转移
>
> 除了 `username`字段外其余字段仍然可用
>
> ```rust
> let user1 = User {
>     email: String::from("someone@example.com"),
>     username: String::from("someusername123"),
>     active: true,
>     sign_in_count: 1,
> };
> let user2 = User {
>     active: user1.active,
>     username: user1.username,
>     email: String::from("another@example.com"),
>     sign_in_count: user1.sign_in_count,
> };
> println!("{}", user1.active);
> // 下面这行会报错
> println!("{:?}", user1);
> ```

1. 元组结构体
   ```rust
   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);

   let black = Color(255, 0, 0);
   let origin = Point(1, 2, 3);
   ```

   - 元组结构体在你希望有一个整体名称，但是又不关心里面字段的名称时将非常有用
4. C结构体
   ```rust
   struct User {
       active: bool,
       username: String,
       email: String,
       sign_in_count: u64,
   }
   ```
5. 单元结构体

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual { /*...*/ }
```

- 没有任何字段和属性
- 如果你定义一个类型，但是不关心该类型的内容，只关心它的行为时使用

#### 结构体数据的所有权

- 可以让 `User` 结构体从其它对象借用数据，不过这么做，就需要引入**生命周期(lifetimes)**这个新概念
  - 生命周期能确保结构体的作用范围要比它所借用的数据的作用范围要小

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

/*
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter // 需要一个生命周期
  |
help: consider introducing a named lifetime parameter // 考虑像下面的代码这样引入一个生命周期
  |
1 ~ struct User<'a> {
2 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:3:12
  |
3 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     username: &str,
3 ~     email: &'a str,
  |
*/
```

#### 使用#[derive(Debug)]来打印结构体信息

使用 `#[derive(Debug)]` 对结构体进行了标记，这样才能使用 `println!("{:?}", s)`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

- 报错，需要实现 `Display`特征（基本类型默认实现）

```rust
println!("rect1 is {:?}", rect1);
```

- 使用 `{:?}`需要实现 `Debug`特征，或添加 `#[derive(Debug)]`（后者有一定限制）
- 使用 `{:#?}` 来替代 `{:?}`获得一定的美化输出格式

使用 `dbg!`宏输出debug信息

- 会移走表达式的所有权，然后打印出相应的文件名、行号等debug信息，**最终将表达式的所有权返回**

> `dbg!` 输出到标准错误输出 `stderr`，而 `println!` 输出到标准输出 `stdout`

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

/*
$ cargo run
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
*/
```

### 枚举

```rust
enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}
```

> **枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例**

通过 `::`操作符访问枚举内具体成员

```rust
fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;

    print_suit(heart);
    print_suit(diamond);
}

fn print_suit(card: PokerSuit) {
    // 需要在定义 enum PokerSuit 的上面添加上 #[derive(Debug)]，否则会报 card 没有实现 Debug
    println!("{:?}",card);
}
```

将数据信息关联到枚举成员

```rust
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
   let c1 = PokerCard::Spades(5);
   let c2 = PokerCard::Diamonds(13);
}
```

**任何类型的数据都可以放入枚举成员中**

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let (i, j) = Message::Move{x:1,y:1}; // 解构
    let m3 = Message::ChangeColor(255,255,0);
}
```

#### 同一化类型

例如我们有一个 WEB 服务，需要接受用户的长连接，假设连接有两种：`TcpStream` 和 `TlsStream`，但是我们希望对这两个连接的处理流程相同，也就是用同一个函数来处理这两个连接，代码如下：

```rust
fn new (stream: TcpStream) {
  let mut s = stream;
  if tls {
    s = negotiate_tls(stream)
  }

  // websocket是一个WebSocket<TcpStream>或者
  //   WebSocket<native_tls::TlsStream<TcpStream>>类型
  websocket = WebSocket::from_raw_socket(s, ...)
}

// 通过枚举类同一化类型
enum Websocket {
  Tcp(Websocket<TcpStream>),
  Tls(Websocket<native_tls::TlsStream<TcpStream>>),
}
```

#### Option枚举处理空值

`Option` 枚举包含两个成员，一个成员表示含有值：`Some(T)`, 另一个表示没有值：`None`

```rust
enum Option<T> {
    Some(T),
    None,
}
```

如果使用 `None` 而不是 `Some`，需要告诉 Rust `Option<T>` 是什么类型的，因为编译器只通过 `None` 值无法推断出 `Some` 成员保存的值的类型

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

> 当有一个 `Some` 值时，我们就知道存在一个值，而这个值保存在 `Some` 中。当有个 `None` 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值
>
>  `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，因此它比空值好 -- 避免了与空值进行运算

- 在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`。通常这能帮助我们捕获到空值最常见的问题之一：期望某值不为空但实际上为空的情况

通过 `match`处理不同 `Option`的情况

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 数组

- 最常用的数组有两种，第一种是速度很快但是长度固定的 `array`，第二种是可动态增长的但是有性能损耗的 `Vector`
  - 后者称为动态数组
- 数组的三要素：
  - 长度固定 -- **存储在栈上（相应地，**动态数组 `Vector` 是存储在堆上**）**
  - 元素必须有相同的类型
  - 依次线性排列

#### 创建数组

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    // 数组声明类型
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 初始化一个某个值重复出现 N 次的数组
    let a = [3; 5];
}
```

- 跟数组类型的声明语法其实是保持一致的：`[3; 5]` 和 `[类型; 长度]`

#### 访问数组元素

因为数组是连续存放元素的，因此可以通过索引的方式来访问存放其中的元素

```rust
fn main() {
    let a = [9, 8, 7, 6, 5];

    let first = a[0]; // 获取a数组第一个元素
    let second = a[1]; // 获取第二个元素
}
```

##### 越界访问

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
```

> 当你尝试使用索引访问元素时，Rust 将检查你指定的索引是否小于数组长度。如果索引大于或等于数组长度，Rust 会出现 ***panic***

- 这种检查只能在运行时进行

##### 数组元素为复合类型

- `let array=[3;5]`底层就是不断的Copy出来的，但很可惜复杂类型都没有深拷贝，只能一个个创建
  - 因此需要类型实现 `Copy`特征

```rust
// 错误
let array = [String::from("rust is good!"); 8];
println!("{:#?}", array);

// 正确
let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
println!("{:#?}", array);
```

#### 数组切片

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

* 切片的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
* 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
* 切片类型[T]拥有不固定的大小，而切片引用类型&[T]则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此&[T]更有用,`&str`字符串切片也同理

#### 总结

```rust
fn main() {
  // 编译器自动推导出one的类型
  let one             = [1, 2, 3];
  // 显式类型标注
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
  let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];

  // 借用arrays的元素用作循环中
  for a in &arrays {
    print!("{:?}: ", a);
    // 将a变成一个迭代器，用于循环
    // 你也可以直接用for n in a {}来进行循环
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t({:?} = {})", a, sum);
  }
}
```

* **数组类型容易跟数组切片混淆** ，`[T;n]`描述了一个数组的类型，而`[T]`描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用`[T;n]`的形式去描述
* `[u8; 3]`和 `[u8; 4]`是不同的类型，数组的长度也是类型的一部分
* **在实际开发中，使用最多的是数组切片[T]** ，我们往往通过引用的方式去使用 `&[T]`，因为后者有固定的类型大小
