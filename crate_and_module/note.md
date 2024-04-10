### 包和模块

当工程规模变大时，把代码写到一个甚至几个文件中，都是不太聪明的做法，可能存在以下问题：

1. 单个文件过大，导致打开、翻页速度大幅变慢
2. 查询和定位效率大幅降低，类比下，你会把所有知识内容放在一个几十万字的文档中吗？
3. 只有一个代码层次：函数，难以维护和协作，想象一下你的操作系统只有一个根目录，剩下的都是单层子目录会如何：`disaster`
4. 容易滋生 Bug

同时，将大的代码文件拆分成包和模块，还允许我们实现代码抽象和复用

Rust 也提供了相应概念用于代码的组织管理：

* **项目(Packages)**：一个 `Cargo` 提供的 `feature`，可以用来构建、测试和分享包
* **工作空间(WorkSpace)** ：对于大型项目，可以进一步将多个包联合在一起，组织成工作空间
* **包(Crate)**：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
* **模块(Module)**：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元

### 包crate

> 对于 Rust 而言，包是一个独立的可编译单元，它编译后会生成一个可执行文件或者一个库
>
> 一个包会将相关联的功能打包在一起，使得该功能可以很方便的在多个项目中分享
>
> - 例如准库中没有提供但是在三方库中提供的 `rand` 包，它提供了随机数生成的功能，我们只需要将该包通过 `use rand;` 引入到当前项目的作用域中，就可以在项目中使用 `rand` 的功能：`rand::XXX`
>
> 同一个包中不能有同名的类型，但是在不同包中就可以
>
> - 例如，虽然 `rand` 包中，有一个 `Rng` 特征，可是我们依然可以在自己的项目中定义一个 `Rng`，前者通过 `rand::Rng` 访问，后者通过 `Rng` 访问，对于编译器而言，这两者的边界非常清晰，不会存在引用歧义

#### 项目Package

由于 `Package` 就是一个项目，因此它包含有独立的 `Cargo.toml` 文件，以及因为功能性被组织在一起的一个或多个包

一个 `Package` 只能包含**一个**库(library)类型的包，但是可以包含**多个**二进制可执行类型的包

##### 二进制Package

```
cargo new my-package
```

> Cargo 有一个惯例： **`src/main.rs` 是二进制包的根文件，该二进制包的包名跟所属 `Package` 相同，在这里都是 `my-package`** ，所有的代码执行都从该文件中的 `fn main()` 函数开始

##### 库Package

```
cargo new my-lib --lib
```

> 如果你试图运行 `my-lib`，会报错，原因是库类型的 `Package` 只能作为三方库被其它项目引用，而不能独立运行，只有之前的二进制 `Package` 才可以运行

##### 易混淆的Package和包

`Package` 是一个项目工程，而包只是一个编译单元，基本上也就不会混淆这个两个概念了：`src/main.rs` 和 `src/lib.rs` 都是编译单元，因此它们都是包

##### 典型的Package结构

如果一个 `Package` 同时拥有 `src/main.rs` 和 `src/lib.rs`，那就意味着它包含两个包：库包和二进制包，这两个包名也都与 `Package` 同名

一个真实项目中典型的 `Package`，会包含多个二进制包，这些包文件被放在 `src/bin` 目录下，每一个文件都是独立的二进制包，同时也会包含一个库包，该包只能存在一个 `src/lib.rs`

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

### 模块Module

Rust 的代码构成单元：模块

使用模块可以将包中的代码按照功能性进行重组，最终实现更好的可读性及易用性

灵活地控制代码的可见性，进一步强化 Rust 的安全性

#### 创建嵌套模块

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

* 使用 `mod` 关键字来创建新模块，后面紧跟着模块名称
* 模块可以嵌套，这里嵌套的原因是招待客人和服务都发生在前厅，因此我们的代码模拟了真实场景
* 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
* 所有模块均定义在同一个文件中

> 使用模块将功能相关的代码组织到一起，然后通过一个模块名称来说明这些代码为何被组织在一起

#### 模块树

模块之间**彼此的嵌套**关系 -- **模块树**

```
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

其中 `crate` 包根是 `src/lib.rs` 文件，包根文件中的三个模块分别形成了模块树的剩余部分

##### 父子模块

如果模块 `A` 包含模块 `B`，那么 `A` 是 `B` 的父模块，`B` 是 `A` 的子模块

#### 用路径引用模块

* **绝对路径** ，从包根开始，路径名以包名或者 `crate` 作为开头
* **相对路径** ，从当前模块开始，以 `self`，`super` 或当前模块的标识符作为开头

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

##### 绝对路径引用

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

##### 相对路径引用

```rust
front_of_house::hosting::add_to_waitlist();
```

##### 绝对还是相对？

**当代码被挪动位置时，尽量减少引用路径的修改**

```rust
crate
 └── customer_experience
    └── eat_at_restaurant
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
// 绝对路径引用需要改变，但相对路径不变

crate
 └── dining
     └── eat_at_restaurant
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
// 对eat_at_restaurant的绝对路径引用不会改变，但是相对路径会变
```

如果不确定哪个好，你可以考虑优先使用绝对路径，因为调用的地方和定义的地方往往是分离的，而定义的地方较少会变动

#### 代码可见性

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

- 运行后报错： `hosting` 模块是私有的，无法在包根进行访问
- 模块不仅仅对于组织代码很有用，它还能定义代码的私有化边界：在这个边界内，什么内容能让外界看到，什么内容不能，都有很明确的定义
- Rust 出于安全的考虑，默认情况下，所有的类型都是私有化的，包括函数、方法、结构体、枚举、常量，是的，就连模块本身也是私有化的
- **父模块完全无法访问子模块中的私有项，但是子模块却可以访问父模块、父父..模块的私有项**

##### pub关键字

Rust 提供了 `pub` 关键字，通过它你可以控制模块和模块中指定项的可见性

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
```

- 仍然报错
- 模块可见性不代表模块内部项的可见性
  - 模块的可见性仅仅是允许其它模块去引用它，但是想要引用它内部的项，还得继续将对应的项标记为 `pub`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

#### 使用super引用模块

`super` 代表的是父模块为开始的引用方式

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

#### 使用self引用模块

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

#### 结构体和枚举的可见性

* 将结构体设置为 `pub`，但它的所有字段依然是私有的
* 将枚举设置为 `pub`，它的所有字段也将对外可见

#### 模块与文件分离

当模块变多或者变大时，需要将模块放入一个单独的文件中，让代码更好维护

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

* `mod front_of_house;` 告诉 Rust 从另一个和模块 `front_of_house` 同名的文件中加载该模块的内容
* 使用绝对路径的方式来引用 `hosting` 模块：`crate::front_of_house::hosting;`

> 模块的声明和实现是分离的，实现是在单独的 `front_of_house.rs` 文件中，然后通过 `mod front_of_house;` 这条声明语句从该文件中把模块内容加载进来

- 当一个模块有许多子模块时，我们也可以通过文件夹的方式来组织这些子模块

```rust
// 创建一个目录 front_of_house，然后在文件夹里创建一个 hosting.rs 文件
pub fn add_to_waitlist() {}
```

- 错误，如果需要将文件夹作为一个模块，我们需要进行显示指定暴露哪些子模块
  * 在 `front_of_house` 目录里创建一个 `mod.rs`，如果你使用的 `rustc` 版本 `1.30` 之前，这是唯一的方法。
  * 在 `front_of_house` **同级**目录里创建一个与模块（目录）**同名**的 rs 文件 `front_of_house.rs`，在新版本里，更建议使用这样的命名方式来避免项目中存在大量同名的 `mod.rs` 文件
