### 包和模块

将大的代码文件拆分成包和模块，还允许我们实现代码抽象和复用：将你的代码封装好后提供给用户，那么用户只需要调用公共接口即可，无需知道内部该如何实现

* **项目(Package)** ：可以用来构建、测试和分享包
* **工作空间(WorkSpace)** ：对于大型项目，可以进一步将多个包联合在一起，组织成工作空间
* **包(Crate)** ：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
* **模块(Module)** ：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元

#### 包和Package

##### 包crate

包是一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库

一个包会将相关联的功能打包在一起，使得该功能可以很方便的在多个项目中分享

同一个包中不能有同名的类型，但是在不同包中就可以

##### 项目Package

由于 `Package` 就是一个项目，因此它包含有独立的 `Cargo.toml` 文件，以及因为功能性被组织在一起的一个或多个包

一个 `Package` 只能包含**一个**库(library)类型的包，但是可以包含**多个**二进制可执行类型的包

###### 二进制Package

```rust
cargo new my-project
```

- Cargo 有一个惯例： **`src/main.rs` 是二进制包的根文件，该二进制包的包名跟所属 `Package` 相同，在这里都是 `my-project`** ，所有的代码执行都从该文件中的 `fn main()` 函数开始

###### 库Package

```rust
cargo new my-lib --lib
```

- 如果试图运行 `my-lib`，会报错
- 库类型的 `Package`只能作为三方库被其它项目引用，而不能独立运行，只有二进制 `Package`才可以运行
- 如果一个 `Package`包含有 `src/lib.rs`，意味它包含有一个库类型的同名包 `my-lib`，该包的根文件是 `src/lib.rs`

###### 易混淆的Package和包

`Package` 是一个项目工程，而包只是一个编译单元，基本上也就不会混淆这个两个概念了：`src/main.rs` 和 `src/lib.rs` 都是编译单元，因此它们都是包

###### 典型的Package结构

> 如果一个 `Package` 同时拥有 `src/main.rs` 和 `src/lib.rs`，那就意味着它包含两个包：库包和二进制包，这两个包名也都是 `my-project` —— 都与 `Package` 同名
>
> 一个真实项目中典型的 `Package`，会包含多个二进制包，这些包文件被放在 `src/bin` 目录下，每一个文件都是独立的二进制包，同时也会包含一个库包，该包只能存在一个 `src/lib.rs`

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

* 唯一库包：`src/lib.rs`
* 默认二进制包：`src/main.rs`，编译后生成的可执行文件与 `Package` 同名
* 其余二进制包：`src/bin/main1.rs` 和 `src/bin/main2.rs`，它们会分别生成一个文件同名的二进制可执行文件
* 集成测试文件：`tests` 目录下
* 基准性能测试 `benchmark` 文件：`benches` 目录下
* 项目示例：`examples` 目录下

#### 模块Module

构成包的基本单元

##### 创建嵌套模块

```rust
// 餐厅前厅，用于吃饭
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- 创建了三个模块，有几点需要注意的：
  - 使用 `mod` 关键字来创建新模块，后面紧跟着模块名称
  - 模块可以嵌套，这里嵌套的原因是招待客人和服务都发生在前厅，因此我们的代码模拟了真实场景
  - 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
  - 所有模块均定义在同一个文件中

##### 模块树

`src/main.rs` 和 `src/lib.rs` 被称为包根(crate root)

这两个文件的内容形成了一个模块 `crate`，该模块位于包的树形结构(由模块组成的树形结构)的根部

```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

模块之间**彼此的嵌套**关系 -- **模块树**

`crate` 包根是 `src/lib.rs` 文件，包根文件中的三个模块分别形成了模块树的剩余部分

###### 父子模块

如果模块 `A` 包含模块 `B`，那么 `A` 是 `B` 的父模块，`B` 是 `A` 的子模块

- `如front_of_house`是 `hosting`和 `serving`的父模块，反之，后两者是前者的子模块
- 文件系统中每个文件都有自己的路径，用户可以通过这些路径使用它们，在Rust中通过路径的方式来引用模块

##### 用路径引用模块

想要调用一个函数，就需要知道它的路径，在Rust中，这种路径有两种形式：

* **绝对路径** ，从包根开始，路径名以包名或者 `crate` 作为开头
* **相对路径** ，从当前模块开始，以 `self`，`super` 或当前模块的标识符作为开头

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

###### 绝对路径引用

因为 `eat_at_restaurant` 和 `add_to_waitlist` 都定义在一个包中，因此在绝对路径引用时，可以直接以 `crate` 开头，然后逐层引用，每一层之间使用 `::` 分隔

```
crate::front_of_house::hosting::add_to_waitlist();

crate
 └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

类比文件系统：类似于使用绝对路径调用可执行程序， `/front_of_house/hosting/add_to_waitlist`，使用 `crate`作为开始就和使用 `/`作为开始一样

###### 相对路径引用

因为 `eat_at_restaurant` 和 `front_of_house` 都处于包根 `crate` 中，因此相对路径可以使用 `front_of_house` 作为开头

```rust
front_of_house::hosting::add_to_waitlist();
```

类比文件系统：类似于调用同一个目录下的程序 `front_of_house/hosting/add_to_waitlist`

###### 绝对还是相对

如果只是为了引用到指定模块中的对象，那么两种都可以，但是在实际使用时，需要遵循一个原则：**当代码被挪动位置时，尽量减少引用路径的修改**

```rust
crate
 └── customer_experience
    └── eat_at_restaurant
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
```

- 若将 `front_of_house` 模块和 `eat_at_restaurant` 移动到一个模块中 `customer_experience`，那么绝对路径的引用方式就必须进行修改 `crate::customer_experience::front_of_house::hosting::add_to_waitlist()`，而相对路径则无需修改（相对位置没变）

```
crate
 └── dining
     └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
```

- 其它的都不动，把 `eat_at_restaurant` 移动到模块 `dining` 中，如果使用相对路径，你需要修改该路径 `super::front_of_house::hosting::add_to_waitlist()`，但如果使用的是绝对路径，就无需修改
- 考虑优先使用绝对路径，因为调用的地方和定义的地方往往是分离的，而定义的地方较少会变动

##### 代码可见性

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

- 运行报错：`hosting` 模块是私有的，无法在包根进行访问
- 因为 `front_of_house`和 `eat_at_restaurant`同属于一个包根作用域内，同一个模块内的代码自然不存在私有化问题
- 模块不仅仅对于组织代码很有用，它还能定义代码的私有化边界：在这个边界内，什么内容能让外界看到，什么内容不能，都有很明确的定义
- 默认情况下，所有的类型都是私有化的，包括函数、方法、结构体、枚举、常量，是的，就连模块本身也是私有化的
- **父模块完全无法访问子模块中的私有项，但是子模块却可以访问父模块、父父..模块的私有项**

###### `pub`关键字

控制模块和模块中指定项的可见性

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

/*--- snip ----*/
```

- 模块可见还不够，还需要将函数 `add_to_waitlist` 标记为可见
- 模块可见性不代表模块内部项的可见性，模块的可见性仅仅是允许其它模块去引用它，但是想要引用它内部的项，还得继续将对应的项标记为 `pub`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*--- snip ----*/
```

##### 使用 `super`引用模块

相对路径有三种方式开始：`self`、`super`和 `crate` 或者模块名

通过 `super` 的方式引用模块项，`super` 代表的是父模块为开始的引用方式，非常类似于文件系统中的 `..` 语法

```rust
fn serve_order() {}

// 厨房模块
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

- 也可以使用`crate::serve_order`的方式

##### 使用`self`引用模块

`self` 其实就是引用自身模块中的项

```rust
fn serve_order() {
    self::back_of_house::cook_order()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::serve_order();
    }

    pub fn cook_order() {}
}
```

##### 结构体和枚举的可见性

* 将结构体设置为 `pub`，但它的所有字段依然是私有的
* 将枚举设置为 `pub`，它的所有字段也将对外可见

##### 模块与文件分离

把 `front_of_house` 前厅分离出来，放入一个单独的文件中 `src/front_of_house.rs`

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

* `mod front_of_house;`告诉 Rust 从另一个和模块 `front_of_house`同名的文件中加载该模块的内容
* 使用绝对路径的方式来引用 `hosting`模块：`crate::front_of_house::hosting;`
* 模块的声明和实现是分离的，实现是在单独的 `front_of_house.rs`文件中，然后通过 `mod front_of_house;`这条声明语句从该文件中把模块内容加载进来
  * 可以认为，模块 `front_of_house`的定义还是在 `src/lib.rs`中，只不过模块的具体内容被移动到了 `src/front_of_house.rs`文件中
* 关键字 `use`，来将外部模块中的项引入到当前作用域中来，这样无需冗长的父模块前缀即可调用：`hosting::add_to_waitlist();`

当一个模块有许多子模块时，我们也可以通过文件夹的方式来组织这些子模块

创建一个目录 `front_of_house`，然后在文件夹里创建一个 `hosting.rs` 文件

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

- 错误：如果需要将文件夹作为一个模块，我们需要进行显示指定暴露哪些子模块

  - 在 `front_of_house`目录里创建一个 `mod.rs`，如果你使用的 `rustc` 版本 `1.30` 之前，这是唯一的方法
  - 在 `front_of_house`**同级**目录里创建一个与模块（目录）**同名**的 `.rs`文件 `front_of_house.rs`，在新版本里，更建议使用这样的命名方式来避免项目中存在大量同名的 `mod.rs` 文件
  - 之后在新创建 `.rs`文件中定义子模块

    ```rust
    pub mod hosting;
    // pub mod serving;
    ```

#### 使用`use`及受限可见性

使用 `use` 关键字把路径提前引入到当前作用域中，随后的调用就可以省略该路径，极大地简化了代码

##### 基本引入方式

引入模块中的项有两种方式：绝对路径和相对路径

###### 绝对路径引入模块

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- 使用 `use` 和绝对路径的方式，将 `hosting` 模块引入到当前作用域中
  - 只需通过 `hosting::add_to_waitlist` 的方式，即可调用目标模块中的函数
  - 相比 `crate::front_of_house::hosting::add_to_waitlist()` 的方式要简单的多

###### 相对路径引入模块中的函数

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

###### 引入模块还是函数

从使用简洁性来说，引入函数自然是更甚一筹，但是在某些时候，引入模块会更好：

* 需要引入同一个模块的多个函数
* 作用域中存在同名函数

> **优先使用最细粒度(引入函数、结构体等)的引用方式，如果引起了某种麻烦(例如前面两种情况)，再使用引入模块的方式**

##### 避免同名引用

保证同一个模块中不存在同名项

###### 模块::函数

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

使用模块引入的方式，具体的 `Result`通过 `模块::Result`的方式进行调用

避免同名冲突的关键，就是使用 **父模块的方式来调用**

###### `as`别名引用

可以通过`as`关键字给引入的项起一个别名

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

通过 `use std::io::Result` 将 `Result` 引入到作用域，然后使用 `as` 给予它一个全新的名称 `IoResult`，这样就不会再产生冲突：

* `Result` 代表 `std::fmt::Result`
* `IoResult` 代表 `std:io::Result`

##### 引入项再导出

当外部的模块项 `A` 被引入到当前模块中时，它的可见性自动被设置为私有的，如果你希望允许其它外部代码引用我们的模块项 `A`，那么可以对它进行再导出

- 使用 `pub use` 即可实现
- `use` 代表引入 `hosting` 模块到当前作用域，`pub` 表示将该引入的内容再度设置为可见

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

##### 使用第三方包

引入外部依赖：

1. 修改 `Cargo.toml` 文件，在 `[dependencies]` 区域添加一行：`rand = "0.8.3"`
2. 此时，如果你用的是 `VSCode` 和 `rust-analyzer` 插件，该插件会自动拉取该库，你可能需要等它完成后，再进行下一步

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

###### crate.io和lib.rs

> 可以在 `crates.io` 或者 `lib.rs` 中检索和使用，从目前来说查找包更推荐 `lib.rs`，搜索功能更强大，内容展示也更加合理，但是下载依赖包还是得用 `crates.io`

##### 使用`{}`简化引入方式

```rust
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;
```

- 使用 `{}`一起引入

```rust
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io};
```

- 同时引入模块和模块中的项

```rust
use std::io;
use std::io::Write;

use std::io::{self, Write};
```

###### self

* `use self::xxx`，表示加载当前模块中的 `xxx`。此时 `self` 可省略
* `use xxx::{self, yyy}`，表示，加载当前路径下模块 `xxx` 本身，以及模块 `xxx` 下的 `yyy`

##### 使用`*`引入模块下的所有项

当使用 `*` 来引入的时候要格外小心，因为你很难知道到底哪些被引入到了当前作用域中，有哪些会和你自己程序中的名称相冲突

```rust
use std::collections::*;

struct HashMap;
fn main() {
   let mut v =  HashMap::new();
   v.insert("a", 1);
}
```

- `std::collection::HashMap`被 `*`引入到当前作用域，但是由于存在另一个同名的结构体，因此 `HashMap::new`根本不存在，因为对于编译器来说，本地同名类型的优先级更高

##### 受限的可见性

控制哪些人能看

- 可以通过 `pub(crate) item;`这种方式来实现：`item`虽然是对外可见的，但是只在当前包内可见，外部包无法引用到该 `item`
- 如果我们想要让某一项可以在整个包中都可以被使用，那么有两种办法：
  - 在包根中定义一个非 `pub`类型的 `X`(父模块的项对子模块都是可见的，因此包根中的项对模块树上的所有模块都可见)
  - 在子模块中定义一个 `pub`类型的 `Y`，同时通过 `use`将其引入到包根

```rust
mod a {
    pub mod b {
        pub fn c() {
            println!("{:?}",crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}",Y);
}
```

希望对于某些特定的模块可见，但是对于其他模块又不可见

```rust
// 目标：`a` 导出 `I`、`bar` and `foo`，其他的不导出
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        mod c {
            const J: i32 = 4;
        }
    }
}
```

- 报错，因为与父模块中的项对子模块可见相反，子模块中的项对父模块是不可见的
  - `semisecret` 方法中，`a` -> `b` -> `c` 形成了父子模块链，那 `c` 中的 `J` 自然对 `a` 模块不可见

想保持 `J` 私有，同时让 `a` 继续使用 `semisecret` 函数的办法是将该函数移动到 `c` 模块中，然后用 `pub use` 将 `semisecret` 函数进行再导出

```rust
pub mod a {
    pub const I: i32 = 3;

    use self::b::semisecret;

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub use self::c::semisecret;
        mod c {
            const J: i32 = 4;
            pub fn semisecret(x: i32) -> i32 {
                x + J
            }
        }
    }
}
```

如果想保持代码逻辑，同时又只让 `J` 在 `a` 内可见

```rust
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}
```

- 通过 `pub(in crate::a)` 的方式，我们指定了模块 `c` 和常量 `J` 的可见范围都只是 `a` 模块中，`a` 之外的模块是完全访问不到它们的

###### 限制可见性语法

`pub(crate)` 或 `pub(in crate::a)` 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见

* `pub` 意味着可见性无任何限制
* `pub(crate)` 表示在当前包可见
* `pub(self)` 在当前模块可见
* `pub(super)` 在父模块可见
* `pub(in <path>)` 表示在某个路径代表的模块中可见，其中 `path` 必须是父模块或者祖先模块

```rust
// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前包中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
```
