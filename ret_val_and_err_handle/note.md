### 返回值和错误处理

Rust 中的错误主要分为两类：

* **可恢复错误** ，通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
* **不可恢复错误** ，刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的

`Result<T, E>` 用于可恢复错误，`panic!` 用于不可恢复错误

#### `panic!`与不可恢复错误

- 对于这些严重到影响程序运行的错误，触发 `panic` 是很好的解决方式
- 在 Rust 中触发 `panic` 有两种方式：被动触发和主动调用

##### 被动触发

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
```

##### 主动调用

`panic!` 宏，当调用执行该宏时，**程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序**

- 一定是不可恢复的错误，不知道如何处理时，才调用 `panic!` 处理

```rust
fn main() {
    panic!("crash and burn");
}

// thread 'main' panicked at 'crash and burn', src/main.rs:2:5
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* `main` 函数所在的线程崩溃了，发生的代码位置是 `src/main.rs` 中的第 2 行第 5 个字符（包含该行前面的空字符）
* 在使用时加上一个环境变量可以获取更详细的栈展开信息：
  * Linux/macOS 等 UNIX 系统： `RUST_BACKTRACE=1 cargo run`
  * Windows 系统（PowerShell）： `$env:RUST_BACKTRACE=1 ; cargo run`

#### Backtrace栈展开

错误往往涉及到很长的调用链甚至会深入第三方库，如果没有栈展开技术，错误将难以跟踪处理

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

- 缓冲区溢出错误
- 使用 `RUST_BACKTRACE=1 cargo run` 或 `$env:RUST_BACKTRACE=1 ; cargo run` 来再一次运行程序

```shell
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:101:14
   2: core::panicking::panic_bounds_check
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:77:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/slice/index.rs:184:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/slice/index.rs:15:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/alloc/src/vec/mod.rs:2465:9
   6: world_hello::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

- 栈展开(也称栈回溯)，包含了函数调用的顺序，按照逆序排列：最近调用的函数排在列表的最上方
- 排在最顶部最后一个调用的函数是 `rust_begin_unwind`，该函数的目的就是进行栈展开
- 要获取到栈回溯信息，你还需要开启 `debug` 标志

#### panic时的两种终止方式

出现 `panic!` 时，程序提供了两种方式来处理终止流程：**栈展开**和**直接终止**

- 默认的方式就是 `栈展开`，这意味着 Rust 会回溯栈上数据和函数调用，因此也意味着更多的善后工作，好处是可以给出充分的报错信息和栈调用信息，便于事后的问题复盘
- `直接终止`，顾名思义，不清理数据就直接退出程序，善后工作交与操作系统来负责

通过修改 `Cargo.toml`文件实现release模式下遇到`panic`直接终止

```toml
[profile.release]
panic = 'abort'
```

#### 线程panic后，程序是否会终止？

如果是 `main` 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响 `main` 线程

因此，尽量不要在 `main` 线程中做太多任务，将这些任务交由子线程去做，就算子线程 `panic` 也不会导致整个程序的结束

#### 何时使用`panic!`

对于 `Result` 返回我们有很多处理方法，最简单粗暴的就是 `unwrap` 和 `expect`

```rust
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

- `parse` 方法试图将字符串 `"127.0.0.1"<span> </span>`解析为一个IP地址类型 `IpAddr`，它返回一个 `Result<IpAddr, E>` 类型
  - 如果解析成功，则把 `Ok(IpAddr)` 中的值赋给 `home`
  - 如果失败，则不处理 `Err(E)`，而是直接 `panic`
- `unwrap` 简而言之：成功则返回值，失败则 `panic`，总之不进行任何错误处理

##### 示例、原型、测试

这几个场景下，需要快速地搭建代码，错误处理会拖慢编码的速度，也不是特别有必要，因此通过 `unwrap`、`expect` 等方法来处理是最快的

同时，当我们回头准备做错误处理时，可以全局搜索这些方法，不遗漏地进行替换

##### 确切知道程序时正确时，可以使用panic

因为 `panic` 的触发方式比错误处理要简单，因此可以让代码更清晰，可读性也更加好，当我们的代码注定是正确时，你可以用 `unwrap` 等方法直接进行处理，反正也不可能 `panic`

##### 可能导致全局有害状态时

有害状态大概分为几类：

* 非预期的错误
* 后续代码的运行会受到显著影响
* 内存安全的问题

当错误预期会出现时，返回一个错误较为合适

- **因为错误是可预期的，因此也是可以处理的**

当启动时某个流程发生了错误，对后续代码的运行造成了影响，那么就应该使用 `panic`，而不是处理错误后继续运行，当然你可以通过重试的方式来继续

#### panic原理剖析

当调用 `panic!` 宏时，它会

1. 格式化 `panic`信息，然后使用该信息作为参数，调用 `std::panic::panic_any()`函数
2. `panic_any`会检查应用是否使用了[`panic hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html)，如果使用了，该 `hook`函数就会被调用
   * `hook`是一个钩子函数，是外部代码设置的，用于在 `panic`触发时，执行外部代码所需的功能
3. 当 `hook`函数返回后，当前的线程就开始进行栈展开：从 `panic_any`开始
   * 如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
4. 展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃
   * 但是在展开过程中，你可能会遇到被用户标记为 `catching`的帧
     * 通过 `std::panic::catch_unwind()`函数标记
   * 此时用户提供的 `catch`函数会被调用，展开也随之停止
   * 如果 `catch`选择在内部调用 `std::panic::resume_unwind()`函数，则展开还会继续

还有一种情况，在展开过程中，如果展开本身 `panic` 了，那展开线程会终止，展开也随之停止

一旦线程展开被终止或者完成，最终的输出结果是取决于哪个线程 `panic`：

- 对于 `main` 线程，操作系统提供的终止功能 `core::intrinsics::abort()` 会被调用，最终结束当前的 `panic` 进程
- 是其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 `std::thread::join()` 进行收集

#### 可恢复的错误Result

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

> 获知变量类型或者函数的返回类型
>
> * 查询标准库或三方库文档
> * 通过IDE插件辅助查看
> * 故意标注一个错误类型，让编译器检测提示
>   ```rust
>   let f: u32 = File::open("hello.txt");
>
>   /**
>   error[E0308]: mismatched types
>    --> src/main.rs:4:18
>     |
>   4 |     let f: u32 = File::open("hello.txt");
>     |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
>   `std::result::Result`
>     |
>     = note: expected type `u32`
>                found type `std::result::Result<std::fs::File, std::io::Error>`
>   */
>   ```

通过Result枚举获取调用成功或失败信息

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

#### 对返回的错误进行处理

需要对部分错误进行特殊处理，而不是所有错误都直接崩溃

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

匹配出 `error` 后，又对 `error` 进行了详细的匹配解析，最终结果：

* 如果是文件不存在错误 `ErrorKind::NotFound`，就创建文件，这里创建文件 `File::create` 也是返回 `Result`，因此继续用 `match` 对其结果进行处理：创建成功，将新的文件句柄赋值给 `f`，如果失败，则 `panic`
* 剩下的错误，一律 `panic`

#### 失败就`panic`：`unwrap`和`expect`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
} // 成功取出Result<T>，否则panic

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
} // expect会带上自定义的错误提示信息
```

#### 传播错误

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}
```

* 该函数返回一个 `Result<String, io::Error>` 类型，当读取用户名成功时，返回 `Ok(String)`，失败时，返回 `Err(io:Error)`
* `File::open` 和 `f.read_to_string` 返回的 `Result<T, E>` 中的 `E` 就是 `io::Error`

- 该函数将 `io::Error` 的错误往上进行传播，该函数的调用者最终会对 `Result<String,io::Error>` 进行再处理

#### 传播界的大明星：`?`

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

其实 `?` 就是一个宏，它的作用与 `match` 几乎一模一样

```rust
let mut f = match f {
    // 打开文件成功，将file句柄赋值给f
    Ok(file) => file,
    // 打开文件失败，将错误返回(向上传播)
    Err(e) => return Err(e),
};
```

如果结果是 `Ok(T)`，则把 `T` 赋值给 `f`，如果结果是 `Err(E)`，则返回该错误

错误之间很可能会存在上下级关系

- 例如标准库中的 `std::io::Error<span> </span>`和 `std::error::Error`，前者是 IO 相关的错误结构体，后者是一个最最通用的标准错误特征，同时前者实现了后者，因此 `std::io::Error` 可以转换为 `std:error::Error`

`?`**可以自动进行类型提升（转换）**

```rust
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}
```

- 可以用一个大而全的 `ReturnError` 来覆盖所有错误类型，只需要为各种子错误类型实现`From`特征转换即可

`?` **还能实现链式调用**

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

从文件读取数据到字符串中，`fs::read_to_string` 函数

- 该函数内部会打开一个文件、创建 `String`、读取文件内容最后写入字符串并返回

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
    fs::read_to_string("hello.txt")
}
```

##### `?`用于 `Option`的返回

`?` 不仅仅可以用于 `Result` 的传播，还能用于 `Option` 的传播

```rust
fn first(arr: &[i32]) -> Option<&i32> {
   let v = arr.get(0)?;
   Some(v)
}

fn first(arr: &[i32]) -> Option<&i32> {
   arr.get(0)
}
```

- 如果 `get` 的结果是 `None`，则直接返回 `None`，如果是 `Some(&i32)`，则把里面的值赋给 `v`

##### 新手用`?`常会犯的错误

`?` **操作符需要一个变量来承载正确的值**

```rust
fn first(arr: &[i32]) -> Option<&i32> {
   arr.get(0)?
}
```

这个函数只会返回 `Some(&i32)` 或者 `None`，只有错误值能直接返回，正确的值不行，所以如果数组中存在 0 号元素，那么函数第二行使用 `?` 后的返回类型为 `&i32` 而不是 `Some(&i32)`

`?` 只能用于以下形式：

* `let v = xxx()?;`
* `xxx()?.yyy()?;`

##### 带返回值的main函数

`?` 要求 `Result<T, E>` 形式的返回值，而 `main` 函数的返回是 `()`，因此无法满足

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

> 因为 `std::error:Error` 是 Rust 中抽象层次最高的错误，其它标准库中的错误都实现了该特征，因此我们可以用该特征对象代表一切错误，就算 `main` 函数中调用任何标准库函数发生错误，都可以通过 `Box<dyn Error>` 这个特征对象进行返回

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

- 至于 `main` 函数可以有多种返回值，那是因为实现了 [std::process::Termination](https://doc.rust-lang.org/std/process/trait.Termination.html) 特征

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

#### `map`和`and_then`

```rust
use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
   n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
```

```rust
use std::num::ParseIntError;

// 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
// 但是这种写法实在过于啰嗦..
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1)  => {
            match n2_str.parse::<i32>() {
                Ok(n2)  => {
                    Ok(n1 * n2)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

// 重写上面的 `multiply` ，让它尽量简洁
// 提示：使用 `and_then` 和 `map`
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // 实现...
    n1_str.parse::<i32>().and_then(|a| n2_str.parse::<i32>().map(|b| a * b))
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply1("10", "2");
    print(twenty);

    // 下面的调用会提供更有帮助的错误信息
    let tt = multiply("t", "2");
    print(tt);

    println!("Success!")
}
```

#### 类型别名

```rust
use std::num::ParseIntError;

// 填空
type Res<T> = Result<T, ParseIntError>;

// 使用上面的别名来引用原来的 `Result` 类型
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 同样, 这里也使用了类型别名来简化代码
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!")
}
```
