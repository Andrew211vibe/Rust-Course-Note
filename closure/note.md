### 闭包

函数式语言的优秀特性：

* 使用函数作为参数进行传递
* 使用函数作为函数返回值
* 将函数赋值给变量

> 闭包是**一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值**

```rust
fn main() {
   let x = 1;
   let sum = |y| x + y;

    assert_eq!(3, sum(2));
}
```

#### 使用闭包来简化代码

##### 传统函数实现

```rust
use std::thread;
use std::time::Duration;

// 开始健身，好累，我得发出声音：muuuu...
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            muuuuu(intensity)
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            muuuuu(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            muuuuu(intensity)
        );
    }
}

fn main() {
    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}
```

> 不用 `muuuuu` 函数了，是不是得把所有 `muuuuu` 都替换成，比如说 `woooo` ？如果 `muuuuu` 出现了几十次，那意味着我们要修改几十处地方

##### 函数变量实现

把函数赋值给一个变量，然后通过变量调用

```rust
use std::thread;
use std::time::Duration;

// 开始健身，好累，我得发出声音：muuuu...
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity: u32, random_number: u32) {
    let action = muuuuu;
    if intensity < 25 {
        println!(
            "今天活力满满, 先做 {} 个俯卧撑!",
            action(intensity)
        );
        println!(
            "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
            action(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
            action(intensity)
        );
    }
}

fn main() {
    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}
```

> 若 `intensity` 也变了怎么办？例如变成 `action(intensity + 1)`，那你又得哐哐哐修改几十处调用

##### 闭包实现

使用闭包来捕获 `intensity`

```rust
use std::thread;
use std::time::Duration;

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }
}

fn main() {
    // 动作次数
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}
```

> 无论你要修改什么，只要修改闭包 `action` 的实现即可，其它地方只负责调用，完美解决了我们的问题

参数是通过 `|parm1|` 的形式进行声明，如果是多个参数就 `|param1, param2,...|`

```rust
|param1, param2,...| {
    语句1;
    语句2;
    返回表达式
}
// 如果只有一个返回表达式的话，定义可以简化为
|param1| 返回表达式
```

* **闭包中最后一行表达式返回的值，就是闭包执行后的返回值**
* `let action = ||...` 只是把闭包赋值给变量 `action`，并不是把闭包执行后的结果赋值给 `action`，因此这里 `action` 就相当于闭包函数，可以跟函数一样进行调用：`action()`

#### 闭包的类型推导

与函数相反，闭包并不会作为 API 对外提供，因此它可以享受编译器的类型推导能力，无需标注参数和返回值的类型

给闭包标注类型：

```rust
let sum = |x: i32, y: i32| -> i32 {
    x + y
}
```

- 不标注类型的闭包声明会更简洁些：`let sum = |x, y| x + y`
- 需要注意的是，针对 `sum` 闭包，如果你只进行了声明，但是没有使用，编译器会提示你为 `x, y` 添加类型标注，因为它缺乏必要的上下文

虽然类型推导很好用，但是它不是泛型，**当编译器推导出一种类型后，就会一直使用该类型**

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // Error: 上一行已推导类型为String，而非i32
```

#### 结构体中的闭包

```rust
// 简易缓存：一个闭包用于获取值；一个变量，用于存储该值
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}
```

- `Fn(u32) -> u32` 是一个特征，用来表示 `T` 是一个闭包类型
- 每一个闭包实例都有独属于自己的类型，即使于两个签名一模一样的闭包，它们的类型也是不同的，因此你无法用一个统一的类型来标注 `query` 闭包
- 标准库提供的 `Fn`系列特征，再结合特征约束，就能很好的解决了这个问题. `T: Fn(u32) -> u32`意味着 `query`的类型是 `T`，该类型必须实现了相应的闭包特征 `Fn(u32) -> u32`
- 特征 `Fn(u32) -> u32` 从表面来看，就对闭包形式进行了显而易见的限制：**该闭包拥有一个 `u32`类型的参数，同时返回一个 `u32`类型的值**

> 其实 `Fn`特征不仅仅适用于闭包，还适用于函数，因此上面的 `query`字段除了使用闭包作为值外，还能使用一个具名的函数来作为它的值

```rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher {
            query,
            value: None,
        }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

将 `u32` 替换成泛型 `E`，缓存任何类型

```rust
struct Cache<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cache<T, E> 
where
    T: Fn(E) -> E,
    E: Copy
{
    fn new(query: T) -> Cache<T, E> {
        Cache {
            query,
            value: None,
        }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn call_with_different_values() {
    let mut c = Cache::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 1);
}
```

#### 捕获作用域中的值

闭包还拥有一项函数所不具备的特性：捕获作用域中的值

```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
```

- `x` 并不是闭包 `equal_to_x` 的参数，但是它依然可以去使用 `x`，因为 `equal_to_x` 在 `x` 的作用域范围内

对于函数来说，就算你把函数定义在 `main` 函数体中，它也不能访问 `x`

```rust
fn main() {
    let x = 4;
    fn equal_to_x(z: i32) -> bool {
        z == x // Error
    }
    let y = 4;
    assert!(equal_to_x(y));
}
```

#### 闭包对内存的影响

> 当闭包从环境中捕获一个值时，会分配内存去存储这些值。对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担

#### 三种 `Fn`特征

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 `Fn` 特征也有三种

1. `FnOnce`，该类型的闭包会拿走被捕获变量的所有权
   * `Once` 顾名思义，说明该闭包只能运行一次
   * ```rust
     fn fn_once<F>(func: F) // Error
     where
         F: FnOnce(usize) -> bool,
     {
         println!("{}", func(3));
         println!("{}", func(4));
     }

     fn main() {
         let x = vec![1, 2, 3];
         fn_once(|z|{z == x.len()})
     }
     ```
   * **仅**实现 `FnOnce` 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用
   * 因为 `F` 没有实现 `Copy` 特征，所以会报错
   * ```rust
     fn fn_once<F>(func: F)
     where
         F: FnOnce(usize) -> bool + Copy,// 改动在这里
     {
         println!("{}", func(3));
         println!("{}", func(4));
     }

     fn main() {
         let x = vec![1, 2, 3];
         fn_once(|z|{z == x.len()})
     }
     ```
   * 实现了 `Copy` 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移
   * 强制闭包取得捕获变量的所有权，可以在参数列表前添加 `move` 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时
     * 例如将闭包返回或移入其他线程
   * ```rust
     use std::thread;
     let v = vec![1, 2, 3];
     let handle = thread::spawn(move || {
         println!("Here's a vector: {:?}", v);
     });
     handle.join().unwrap();
     ```
2. `FnMut`，它以可变借用的方式捕获了环境中的值，因此可以修改该值
   * ```rust
     fn main() {
         let mut s = String::new();

         let update_string =  |str| s.push_str(str); // Error
         update_string("hello");

         println!("{:?}",s);
     }
     ```
   * 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型
   * ```rust
     fn main() {
         let mut s = String::new();

         let mut update_string =  |str| s.push_str(str);
         update_string("hello");

         println!("{:?}",s);
     }
     ```
   * `FnMut`只是 `trait`的名字，声明变量为 `FnMut`和要不要 `mut`没啥关系，`FnMut`是推导出的特征类型，`mut`是rust语言层面的一个修饰符，用于声明一个绑定是可变的
   * ```rust
     fn main() {
         let mut s = String::new();

         let update_string =  |str| s.push_str(str);
         // let update_string: impl FnMut(&str) =  |str| s.push_str(str);

         exec(update_string); // 调用了exec函数，并将update_string闭包的所有权移交给它

         println!("{:?}",s);
     }

     fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
         f("hello")
     }
     ```
   * 在使用 `FnMut`类型闭包时需要捕获外界的可变借用，因此我们常常搭配 `mut`修饰符使用
     * 但我们要始终记住，二者是相互独立的
   * 闭包自动实现 `Copy`特征的规则是，只要闭包捕获的类型都实现了 `Copy`特征的话，这个闭包就会默认实现 `Copy`特征
   * ```rust
     let s = String::new();
     let update_string =  || println!("{}",s);

     // 拿所有权
     let s = String::new();
     let update_string = move || println!("{}", s);

     exec(update_string);
     // exec2(update_string); // 不能再用了

     // 可变引用
     let mut s = String::new();
     let mut update_string = || s.push_str("hello");
     exec(update_string);
     // exec1(update_string); // 不能再用了
     ```
   * 取得的是 `s`的不可变引用，所以是能 `Copy`的
   * 而如果拿到的是 `s`的所有权或可变引用，都是不能 `Copy`的
3. `Fn` 特征，它以不可变借用的方式捕获环境中的值
   * ```rust
     fn main() {
         let mut s = String::new();
         // Error: 闭包实现的是FnMut特征，需要的是可变借用
         let update_string =  |str| s.push_str(str);
         // exec中却标注了Fn特征，不匹配
         exec(update_string);

         println!("{:?}",s);
     }

     fn exec<'a, F: Fn(&'a str)>(mut f: F)  {
         f("hello")
     }

     // 正确方式
     fn main() {
         let s = "hello, ".to_string();
         let update_string =  |str| println!("{},{}",s,str);
         exec(update_string);
         println!("{:?}",s);
     }

     fn exec<'a, F: Fn(String) -> ()>(f: F)  {
         f("world".to_string())
     }
     ```

##### `move`和`Fn`

使用了 `move` 的闭包依然可能实现了 `Fn` 或 `FnMut` 特征

因为，**一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们**

`move` 本身强调的就是后者，闭包如何捕获变量

```rust
fn main() {
    let s = String::new();

    let update_string =  move || println!("{}",s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}
```

- 由于闭包对 `s` 的使用仅仅是不可变借用，因此该闭包实际上**还**实现了 `Fn` 特征
  - 不仅仅实现了 `FnOnce` 特征，还实现了 `Fn` 特征

```rust
fn main() {
    let s = String::new();

    let update_string =  move || println!("{}",s);

    exec(update_string);
}

fn exec<F: Fn()>(f: F)  {
    f()
}
```

##### 三种Fn的关系

一个闭包并不仅仅实现某一种 `Fn` 特征，规则如下：

* 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次
* 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征
* 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征

```rust
fn main() {
    let s = String::new();

    let update_string =  || println!("{}",s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}
```

虽然，闭包只是对 `s` 进行了不可变借用，实际上，它可以适用于任何一种 `Fn` 特征：

```rust
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}
```

- 闭包从捕获环境中移出了变量 `s` 的所有权，因此这个闭包仅自动实现了 `FnOnce`，未实现 `FnMut`和 `Fn`
- **一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们** ，跟是否使用 `move`没有必然联系

```rust
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

> 从特征约束能看出来 `Fn` 的前提是实现 `FnMut`，`FnMut` 的前提是实现 `FnOnce`，因此要实现 `Fn` 就要同时实现 `FnMut` 和 `FnOnce`
>
> `Fn` 获取 `&self`，`FnMut` 获取 `&mut self`，而 `FnOnce` 获取 `self`
>
> **建议先使用 `Fn` 特征** ，然后编译器会告诉你正误以及该如何选择

#### 闭包作为函数返回值

使用闭包作为函数返回值

```rust
fn factory() -> Fn(i32) -> i32 {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

- Rust 要求函数的参数和返回类型，必须有固定的内存大小
  - 不包括特征，因为特征类似接口，对于编译器来说，无法知道它后面藏的真实类型是什么，因为也无法得知具体的大小
  - 无法知道闭包的具体类型
- `impl Trait` 可以用来返回一个实现了指定特征的类型，那么这里 `impl Fn(i32) -> i32` 的返回值形式，说明我们要返回一个闭包类型，它实现了 `Fn(i32) -> i32` 特征

```rust
fn factory(x:i32) -> impl Fn(i32) -> i32 { // Error: if else返回值类型不一样
    let num = 5;

    if x > 1{
        move |x| x + num
    } else {
        move |x| x - num
    }
}
```

> `impl Trait` 的返回方式有一个非常大的局限，就是你只能返回同样的类型
>
> 就算签名一样的闭包，类型也是不同的，因此在这种情况下，就无法再使用 `impl Trait` 的方式去返回闭包

```rust
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```

用特征对象实现返回不同类型的闭包函数
