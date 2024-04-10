### 创建一个项目

```rust
cargo new [OPTIONS] <PATH>
// 早期需要添加--bin参数，现在是默认行为
cargo new world_hello
cargo new world_hello --bin
```

- Rust 项目主要分为两个类型：`bin` 和 `lib`，前者是一个可运行的项目，后者是一个依赖库项目

### 运行项目

1. `cargo run `
2. 手动编译运行
   ```rust
   // 编译
   cargo build
   // 运行
   ./target/debug/world_hello.exe
   ```

- 默认 `debug`模式
  - 编译速度快（编译器未进行优化）
  - 运行速度慢

```rust
// 以release模式编译运行
cargo run --release

cargo build --release
./target/release/world_hello.exe
```

* 快速检查代码能否通过编译 `cargo check`

### Cargo.toml和Cargo.lock

- `Cargo.toml`是项目数据描述文件
- `Cargo.lock`是 `cargo`根据同一项目的 `toml`文件生成的项目依赖详细清单
  - 项目为可运行程序，上传 `Cargo.lock`
  - 项目为依赖库，`.gitignore`忽略上传

#### package配置段落

- 记录项目描述信息

```toml
[package]
name = "world_hello"
version = "0.1.0"
edition = "2021"
```

- `name`字段定义项目名称
- `version`字段定义当前版本
- `edition`字段定义使用的Rust大版本

#### 定义项目依赖

- `Cargo.toml`文件内通过依赖段落描述项目依赖项
  * 基于 Rust 官方仓库 `crates.io`，通过版本说明来描述
  * 基于项目源代码的 git 仓库地址，通过 URL 来描述
  * 基于本地项目的绝对路径或者相对路径，通过类 Unix 模式的路径来描述

    ```toml
    [dependencies]
    rand = "0.3"
    hammer = { version = "0.5.0"}
    color = { git = "https://github.com/bjz/color-rs" }
    geometry = { path = "crates/geometry" }
    ```

---

- Rust原生支持 `UTF-8`编码的字符串
- `println!("{}", &region);`中的 `!`为**宏操作符**，使用 `{}`作为占位符（底层自动识别输出类型）
- `for region in regions.iter(){...}`集合类型 `regions`不能直接进行循环，需要通过 `.iter()`方法转换成迭代器才能用于迭代循环
  - 2021 edition后可直接写为 `for region in regions{...}`，`for`将隐式地将 `regions`转换成迭代器

```rust
fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个fields变量，类型为vector（Vec缩写），可伸缩的集合类型（动态数组）
        // <_>表示Vec中的元素类型由编译器自行推断
        let fields : Vec<_> = record.split(',').map(|field| field.trim()).collect();
        // debug模式条件编译，release模式下不会运行
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把fields[1]的值转换为f32类型的浮点数，如果成功，则把f32值赋给length变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出length的值：
        //  1）当=右边的表达式执行成功，则会返回一个Ok(f32)的类型，若失败，则会返回一个Err(e)类型，if let的作用就是仅匹配Ok也就是成功的情况，如果是错误，就直接忽略
        //  2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的f32值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}
```

* 控制流：`for` 和 `continue` 连在一起使用，实现循环控制。
* 方法语法：由于 Rust 没有继承，因此 Rust 不是传统意义上的面向对象语言，但是它却从 `OO` 语言那里偷师了方法的使用 `record.trim()`，`record.split(',')` 等。
* 高阶函数编程：函数可以作为参数也能作为返回值，例如 `.map(|field| field.trim())`，这里 `map` 方法中使用闭包函数作为参数，也可以称呼为 `匿名函数`、`lambda 函数`。
* 类型标注：`if let Ok(length) = fields[1].parse::<f32>()`，通过 `::<f32>` 的使用，告诉编译器 `length` 是一个 `f32` 类型的浮点数。这种类型标注不是很常用，但是在编译器无法推断出你的数据类型时，就很有用了。
* 条件编译：`if cfg!(debug_assertions)`，说明紧跟其后的输出（打印）只在 `debug` 模式下生效。
* 隐式返回：Rust 提供了 `return` 关键字用于函数返回，但是在很多时候，我们可以省略它。因为 Rust 是[**基于表达式的语言**](https://course.rs/basic/base-type/statement-expression.html)
