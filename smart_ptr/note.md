### 智能指针

**指针是一个包含了内存地址的变量，该内存地址引用或者指向了另外的数据**

在 Rust 中，最常见的指针类型是引用，引用通过 `&` 符号表示

- 借用其它变量的值

智能指针通过比引用更复杂的数据结构，包含比引用更多的信息，例如元数据，当前长度，最大可用长度等

引用和智能指针的另一个不同在于前者仅仅是借用了数据，而后者往往可以拥有它们指向的数据，然后再为其它人提供服务

智能指针往往是基于结构体实现，它与我们自定义的结构体最大的区别在于它实现了 `Deref` 和 `Drop` 特征：

* `Deref`可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 `*T`
* `Drop`允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

常用智能指针：

* `Box<T>`，可以将值分配到堆上
* `Rc<T>`，引用计数类型，允许多所有权存在
* `Ref<T>` 和 `RefMut<T>`，允许将借用规则检查从编译期移动到运行期进行

#### `Box<T>`堆对象分配

`Box<T>` 允许你将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据

##### Rust中的堆栈

- 栈内存从高位地址向下增长，且栈内存是连续分配的，一般来说 **操作系统对栈内存的大小都有限制** ，因此无法创建任意长度的数组

  - 在Rust中，`main`线程的栈的大小[`8MB`](https://course.rs/compiler/pitfalls/stack-overflow.html)，普通线程是 `2MB`，在函数调用时会在其中创建一个临时栈空间，调用结束后 Rust 会让这个栈空间里的对象自动进入 `Drop` 流程，最后栈顶指针自动移动到上一个调用栈顶，无需程序员手动干预，因而栈内存申请和释放是非常高效的
- 堆上内存则是从低位地址向上增长， **堆内存通常只受物理内存限制** ，而且通常是不连续的，因此从性能的角度看，栈往往比堆更高

  - Rust堆上对象还有一个特殊之处，它们都拥有一个所有者，因此受所有权规则的限制：当赋值时，发生的是所有权的转移（只需浅拷贝栈上的引用或智能指针即可）
  - ```rust
    fn main() {
        let b = foo("world");
        println!("{}", b);
    }

    fn foo(x: &str) -> String {
        let a = "Hello, ".to_string() + x;
        a
    }
    ```

###### 堆栈的性能

* 小型数据，在栈上的分配性能和读取性能都要比堆上高
* 中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
* 大型数据，只建议在堆上分配和使用

> 栈的分配速度肯定比堆上快，但是读取速度往往取决于你的数据能不能放入寄存器或 CPU 高速缓存。 因此不要仅仅因为堆上性能不如栈这个印象，就总是优先选择栈，导致代码更复杂的实现

##### Box的使用场景

* 特意的将数据分配在堆上
* 数据较大时，又不想在转移所有权时进行数据拷贝
* 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
* 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

###### 使用 `Box<T>`将数据存储在堆上

```rust
fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
}
```

智能指针往往都实现了 `Deref` 和 `Drop` 特征，因此：

* `println!`可以正常打印出 `a`的值，是因为它隐式地调用了 `Deref`对智能指针 `a`进行了解引用
* 最后一行代码 `let b = a + 1`报错，是因为在表达式中，我们无法自动隐式地执行 `Deref`解引用操作，你需要使用 `*`操作符 `let b = *a + 1`，来显式的进行解引用
* `a`持有的智能指针将在作用域结束（`main`函数结束）时，被释放掉，这是因为 `Box<T>`实现了 `Drop`特征

###### 避免栈上数据的拷贝

当栈上数据转移所有权时，实际上是把数据拷贝了一份，最终新旧变量各自拥有不同的数据，因此所有权并未转移

而堆上则不然，底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，再将新的指针赋予新的变量，然后让拥有旧指针的变量失效，最终完成了所有权的转移

```rust
fn main() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}
```

###### 将动态大小类型变为Sized固定大小类型

Rust需要在编译时知道类型占用多少空间，如果一种类型在编译时无法知道具体的大小，那么被称为动态大小类型DST

- 其中一种无法在编译时知道大小的类型是**递归类型**
  - 在类型定义中又使用到了自身，或者说该类型的值的一部分可以是相同类型的其它值
  - 这种值的嵌套理论上可以无限进行下去，所以Rust不知道递归类型需要多少空间
- ```rust
  enum List {
      Cons(i32, List),
      Nil,
  }
  ```

若想解决这个问题，就可以使用我们的 `Box<T>`

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

只需要将 `List`存储到堆上，然后使用一个智能指针指向它，即可完成从DST到Sized类型(固定大小类型)的华丽转变

###### 特征对象

在Rust中，想实现不同类型组成的数组只有两个办法：枚举和特征对象，前者限制较多，因此后者往往是最常用的解决办法

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

fn main() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}
```

特征也是DST类型，而特征对象在做的就是将DST类型转换为固定大小类型

##### Box内存布局

`Vec<i32>` 的内存布局 -- 智能指针存储在栈中，然后指向堆上的数组数据

```
(stack)    (heap)
┌──────┐   ┌───┐
│ vec1 │──→│ 1 │
└──────┘   ├───┤
           │ 2 │
           ├───┤
           │ 3 │
           ├───┤
           │ 4 │
           └───┘
```

`Vec<Box<i32>>` 的内存布局

```
                    (heap)
(stack)    (heap)   ┌───┐
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec2 │──→│B1 │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │B2 │───→│ 2 │
           ├───┤    └───┘
           │B3 │─┐  ┌───┐
           ├───┤ └─→│ 3 │
           │B4 │─┐  └───┘
           └───┘ │  ┌───┐
                 └─→│ 4 │
                    └───┘
```

> 智能指针 `vec2` 依然是存储在栈上，然后指针指向一个堆上的数组，该数组中每个元素都是一个 `Box` 智能指针，最终 `Box` 智能指针又指向了存储在堆上的实际值
>
> 从数组中取出某个元素时，取到的是对应的智能指针 `Box`，需要对该智能指针进行解引用，才能取出最终的值

```rust
fn main() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
}
```

* 使用 `&`借用数组中的元素，否则会报所有权错误
* 表达式不能隐式的解引用，因此必须使用 `**`做两次解引用，第一次将 `&Box<i32>`类型转成 `Box<i32>`，第二次将 `Box<i32>`转成 `i32`

##### Box::leak

关联函数：`Box::leak`，它可以消费掉 `Box`并且强制目标值从内存中泄漏

可以把一个 `String` 类型，变成一个 `'static` 生命周期的 `&str` 类型

```rust
fn main() {
   let s = gen_static_str();
   println!("{}", s);
}

fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");

    Box::leak(s.into_boxed_str())
}
```

标注的 `'static`只是用来忽悠编译器的，但是超出作用域，一样被释放回收。而使用 `Box::leak`就可以将一个运行期的值转为 `'static`

###### 使用场景

**需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久** ，那么就可以使用 `Box::leak`

##### 总结

`Box`背后是调用 `jemalloc`来做内存管理，所以堆上的空间无需我们的手动管理

#### Deref解引用

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age}
    }

    fn display(self: &mut Person, age: u8) {
        let Person{name, age} = &self;
    }
}
```

智能指针的名称来源，主要就在于它实现了 `Deref` 和 `Drop` 特征，这两个特征可以智能地帮助我们节省使用上的负担：

* `Deref`可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 `*T`
* `Drop`允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

##### 通过`*`获取引用背后的值

常规引用是一个指针类型，包含了目标数据存储的内存地址。对常规引用使用 `*` 操作符，就可以通过解引用的方式获取到内存地址对应的数据值

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

##### 智能指针解引用

实现 `Deref`后的智能指针结构体，就可以像普通引用一样，通过 `*`进行解引用

```rust
fn main() {
    let x = Box::new(1);
    let sum = *x + 1;
}
```

###### 定义自己的智能指针

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

**为智能指针实现 `Deref`特征**

实现 `Deref`特征，以支持 `*`解引用操作符

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let y = MyBox::new(5);

    assert_eq!(5, *y);
}
```

* 在 `Deref`特征中声明了关联类型 `Target`，在之前章节中介绍过，关联类型主要是为了提升代码可读性
* `deref`返回的是一个常规引用，可以被 `*`进行解引用

##### `*`背后的原理

当我们对智能指针 `Box` 进行解引用时，实际上 Rust 为我们调用了以下方法

```rust
*(y.deref())
```

首先调用 `deref`方法返回值的常规引用，然后通过 `*`对常规引用进行解引用，最终获取到目标值

`*`不会无限递归替换，从 `*y`到 `*(y.deref())`只会发生一次，而不会继续进行替换

##### 函数和方法中的隐式`Deref`转换

对于函数和方法的传参，Rust提供了一个极其有用的隐式转换：`Deref`转换

若一个类型实现了 `Deref`特征，那它的引用在传给函数或方法时，会根据参数签名来决定是否进行隐式的 `Deref`转换

```rust
fn main() {
    let s = String::from("hello world");
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}
```

* `String` 实现了 `Deref` 特征，可以在需要时自动被转换为 `&str` 类型
* `&s`是一个 `&String`类型，当它被传给 `display`函数时，自动通过 `Deref`转换成了 `&str`
* 必须使用 `&s`的方式来触发 `Deref`(仅引用类型的实参才会触发自动解引用)

###### 连续的隐式`Deref`转换

`Deref` 可以支持连续的隐式转换，直到找到适合的形式为止

```rust
fn main() {
    let s = MyBox::new(String::from("hello world"));
    display(&s)
}

fn display(s: &str) {
    println!("{}",s);
}
```

这种行为完全不会造成任何的性能损耗，因为完全是在编译期完成

缺点就是：如果你不知道某个类型是否实现了 `Deref`特征，那么在看到某段代码时，并不能在第一时间反应过来该代码发生了隐式的 `Deref`转换

```rust
fn main() {
    let s = MyBox::new(String::from("hello, world"));
    let s1: &str = &s;
    let s2: String = s.to_string();
}
```

- 对于 `s1`，我们通过两次 `Deref` 将 `&str` 类型的值赋给了它（**赋值操作需要手动解引用**）
- 对于 `s2`，我们在其上直接调用方法 `to_string`，实际上 `MyBox` 根本没有没有实现该方法，能调用 `to_string`，完全是因为编译器对 `MyBox` 应用了 `Deref` 的结果（**方法调用会自动解引用**）

##### `Deref`规则总结

一个类型为 `T`的对象 `foo`，如果 `T: Deref<Target=U>`，那么，相关 `foo`的引用 `&foo`在应用的时候会自动转换为 `&U`

###### 引用归一化

Rust编译器实际上只能对 `&v`形式的引用进行解引用操作

Rust会在解引用时自动把智能指针和 `&&&&v`做引用归一化操作，转换成 `&v`形式，最终再对 `&v`进行解引用：

* 把智能指针（比如在库中定义的，Box、Rc、Arc、Cow等）从结构体脱壳为内部的引用类型，也就是转成结构体内部的 `&v`
* 把多重 `&`，例如 `&&&&&&&v`，归一成 `&v`

```rust
impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}
```

`&T` 被自动解引用为 `T`，也就是 `&T: Deref<Target=T>`

`&&&&T` 会被自动解引用为 `&&&T`，然后再自动解引用为 `&&T`，以此类推， 直到最终变成 `&T`

```rust
fn foo(s: &str) {}

// 由于 String 实现了 Deref<Target=str>
let owned = "Hello".to_string();

// 因此下面的函数可以正常运行：
foo(&owned);
```

```rust
use std::rc::Rc;
fn foo(s: &str) {}

// String 实现了 Deref<Target=str>
let owned = "Hello".to_string();
// 且 Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String ，然后 &String 再自动解引用为 &str
let counted = Rc::new(owned);

// 因此下面的函数可以正常运行:
foo(&counted);
```

```rust
struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

let f = &&Foo;

f.foo();
(&f).foo();
(&&f).foo();
(&&&&&&&&f).foo();
```

##### 三种`Deref`转换

Rust还支持将一个可变的引用转换成另一个可变的引用以及将一个可变引用转换成不可变的引用

* 当 `T: Deref<Target=U>`，可以将 `&T`转换成 `&U`，也就是我们之前看到的例子
* 当 `T: DerefMut<Target=U>`，可以将 `&mut T`转换成 `&mut U`
* 当 `T: Deref<Target=U>`，可以将 `&mut T`转换成 `&U`

```rust
struct MyBox<T> {
    v: T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox { v: x }
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

fn main() {
    let mut s = MyBox::new(String::from("hello, "));
    display(&mut s)
}

fn display(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}
```

* 要实现 `DerefMut`必须要先实现 `Deref`特征：`pub trait DerefMut: Deref`
* `T: DerefMut<Target=U>`解读：将 `&mut T` 类型通过 `DerefMut`特征的方法转换为 `&mut U`类型，对应上例中，就是将 `&mut MyBox<String>`转换为 `&mut String`

> Rust可以把可变引用隐式的转换成不可变引用，但反之则不行

##### 总结

> `Deref` 可以说是 Rust 中最常见的隐式类型转换，而且它可以连续的实现如 `Box<String> -> String -> &str` 的隐式转换，只要链条上的类型实现了 `Deref` 特征

#### `Drop`释放资源

##### Rust中的资源回收

在 Rust中，可以指定在一个变量超出作用域时，执行一段特定的代码，最终编译器将帮你自动插入这段收尾代码

这样，就无需在每一个使用该变量的地方，都写一段代码来进行收尾工作和资源释放

##### 一个不那么简单的`Drop`例子

```rust
struct HasDrop1;
struct HasDrop2;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HasDrop2!");
    }
}
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

fn main() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");
}
```

* `Drop`特征中的 `drop`方法借用了目标的可变引用，而不是拿走了所有权，这里先设置一个悬念，后边会讲
* 结构体中每个字段都有自己的 `Drop`

###### `Drop`的顺序

* **变量级别，按照逆序的方式** ，`_x`在 `_foo`之前创建，因此 `_x`在 `_foo`之后被 `drop`
* **结构体内部，按照顺序的方式** ，结构体 `_x`中的字段按照定义中的顺序依次 `drop`

###### 没有实现`Drop`的结构体

实际上，就算你不为 `_x`结构体实现 `Drop`特征，它内部的两个字段依然会调用 `drop`

原因在于，Rust自动为几乎所有类型都实现了 `Drop`特征，因此就算你不手动为结构体实现 `Drop`，它依然会调用默认实现的 `drop`函数，同时再调用每个字段的 `drop`方法

##### 手动回收

当使用智能指针来管理锁的时候，你可能希望提前释放这个锁，然后让其它代码能及时获得锁，此时就需要提前去手动 `drop`

`Drop::drop`只是借用了目标值的可变引用，所以，就算你提前调用了 `drop`，后面的代码依然可以使用目标值，但是这就会访问一个并不存在的值，非常不安全

```rust
#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

fn main() {
    let foo = Foo;
    foo.drop(); // Error
    println!("Running!:{:?}", foo);
}
```

对于Rust而言，不允许显式地调用析构函数（这是一个用来清理实例的通用编程概念）

使用 `drop` 函数 -- 能够拿走目标值的所有权

```rust
pub fn drop<T>(_x: T)
```

```rust
fn main() {
    let foo = Foo;
    drop(foo);
    // 以下代码会报错：借用了所有权被转移的值
    // println!("Running!:{:?}", foo);
}
```

##### `Drop`使用场景

对于 Drop 而言，主要有两个功能：

* 回收内存资源
* 执行一些收尾工作

##### 互斥的`Copy`和`Drop`

无法为一个类型同时实现 `Copy` 和 `Drop` 特征

- 因为实现了 `Copy` 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率

```rust
#[derive(Copy)] // Error
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}
```

#### `Rc`与`Arc`实现1vN所有权机制
