### 引用与借用

- **获取变量的引用，称之为借用(borrowing)**
- 常规引用是一个指针类型，指向了对象存储的内存地址

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- 使用 `*y` 来解出引用所指向的值（也就是 **解引用** ）
- 否则将会报错，因为将整型和引用类型比较

### 不可变引用

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

1. 无需像上章一样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
2. `calculate_length` 的参数 `s` 类型从 `String` 变为 `&String`

- `&` 符号即是引用，它们允许你使用值，但是不获取所有权
  - 作用于函数参数时同理，`&String`指明是一个字符串引用类型
- 因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃
- 引用指向的值默认是不可变的，因此不可对其进行修改

### 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

#### 可变引用同时只能存在一个

* **同一作用域，特定数据只能有一个可变引用**
* 这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
  * 两个或更多的指针同时访问同一数据
  * 至少有一个指针被用来写入数据
  * 没有同步数据访问的机制

```rust
// 通过构建代码块，手动限制作用域
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
```

#### 可变引用与不可变引用不可同时存在

- 避免出现数据污染

> 注意，引用的作用域 `s` 从创建开始，一直持续到它最后一次使用的地方

#### NLL

> **Non-Lexical Lifetimes(NLL)** ，专门用于找到某个引用在作用域(`}`)结束前就不再被使用的代码位置

### 悬垂引用

- 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用
- 当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用

```rust
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！引用指向一个无效的变量

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // 直接返回String -> String的所有权将转移给调用者
```

### 借用规则

* 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
* 引用必须总是有效的

---

`ref` 与 `&` 类似，可以用来获取一个值的引用

```rust
fn main() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c;

    assert_eq!(*r1, *r2);
  
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1), get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
```
