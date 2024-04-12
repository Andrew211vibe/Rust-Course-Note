### 认识生命周期

生命周期，简而言之就是引用的有效作用域

* 就像编译器大部分时候可以自动推导类型 <-> 一样，编译器大多数时候也可以自动推导生命周期
* 在多种类型存在时，编译器往往要求我们手动标明类型 <-> 当多个生命周期存在，且编译器无法推导出某个引用的生命周期时，就需要我们手动标明生命周期

#### 悬垂指针和生命周期

生命周期的主要作用是避免悬垂引用（程序引用了不该引用的数据）

```rust
{
    let r;
    {
        let x = 5;
        r = &x; // 悬垂指针，引用了提前被释放的变量x
    }
    println!("r: {}", r);
}
```

* `let r;` 的声明方式貌似存在使用 `null` 的风险，实际上，当我们不初始化它就使用时，编译器会给予报错
* `r`引用了内部花括号中的 `x`变量，但是 `x`会在内部花括号 `}` 处被释放，因此回到外部花括号后，`r` 会引用一个无效的 `x`
* `r`拥有更大的作用域，或者说**活得更久** 。如果Rust不阻止该悬垂引用的发生，当 `x` 被释放后，`r` 所引用的值就不再是合法的，导致程序发生异常行为

#### 借用检查

为了保证 Rust 的所有权和借用的正确性，Rust 使用了一个借用检查器(Borrow checker)，来检查我们程序的借用正确性

```rust
{
    let r;                // ---------+-- 'a
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
    println!("r: {}", r); //          |
}                         // ---------+

```

- 生命周期 `'b` 比 `'a` 小很多

> 在编译期，Rust 会比较两个变量的生命周期，结果发现 `r` 明明拥有生命周期 `'a`，但是却引用了一个小得多的生命周期 `'b`，在这种情况下，编译器会认为我们的程序存在风险，因此拒绝运行

- 只要 `'b`比 `'a`大就能够编译通过

```rust
{
    let x = 5;            // ----------+-- 'b
    let r = &x;           // --+-- 'a  |
    println!("r: {}", r); //   |       |
}                         // --+-------+

```

#### 函数中的生命周期

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- 编译器无法知道该函数的返回值到底引用 `x` 还是 `y` ，**因为编译器需要知道这些，来确保函数调用后的引用生命周期分析**
- 在存在多个引用时，编译器有时会无法自动推导生命周期，此时就需要我们手动去标注

#### 生命周期标注语法

> 生命周期标注并不会改变任何引用的实际作用域
>
> 标注语法：以 `'` 开头，名称往往是一个单独的小写字母 `'a`，需要对生命周期进行声明 `<'a>`
>
> 如果是引用类型的参数，那么生命周期会位于引用符号 `&` 之后，并用一个空格来将生命周期和引用参数分隔开

```rust
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用
```

##### 函数签名中的生命周期标注

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* 和泛型一样，使用生命周期参数，需要先声明 `<'a>`
* `x`、`y` 和返回值至少活得和 `'a` 一样久(因为返回值要么是 `x`，要么是 `y`)
* 虽然两个参数的生命周期都是标注了 `'a`，但是实际上这两个参数的真实生命周期可能是不一样的(生命周期 `'a` 不代表生命周期等于 `'a`，而是大于等于 `'a`)

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

- `result` 的生命周期等于参数中生命周期最小的，因此要等于 `string2` 的生命周期，也就是说，`result` 要活得和 `string2` 一样久

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

- 因为 `result` 的生命周期是 `'a`，因此 `'a` 必须持续到 `println!`
- 在 `longest` 函数中，`string2` 的生命周期也是 `'a`，由此说明 `string2` 也必须活到 `println!` 处，可是 `string2` 在代码中实际上只能活到内部语句块的花括号处 `}`，小于它应该具备的生命周期 `'a`

##### 深入思考生命周期标注

使用生命周期的方式往往取决于函数的功能

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
} // 永远只返回第一个参数x，因此y的生命周期与x和返回值的生命周期没有任何关系，意味着我们也不必再为y标注生命周期，只需要标注x参数和返回值即可
```

 **函数的返回值如果是一个引用类型，那么它的生命周期只会来源于** ：

* 函数参数的生命周期
* 函数体中某个新建引用的生命周期

若是后者情况，就是典型的悬垂引用场景

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // 返回值引用了函数体内创建的字符串
}
```

- `result`在函数结束后就被释放，但对其的引用依然在继续
- 最好的办法就是返回内部字符串的所有权

```rust
fn longest<'a>(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

fn main() {
   let s = longest("not", "important");
}
```

> 生命周期语法用来将函数的多个引用参数和返回值的作用域关联到一起，一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是内存安全的

#### 结构体中的生命周期

在结构体中使用引用：为结构体中的每一个引用标注上生命周期

```rust
// 生命周期标注说明结构体ImportantExcerpt所引用的字符串str生命周期需要大于等于该结构体的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
} // ImportantExcerpt从第四行开始到main函数结束，被引用字符串novel从第一行开始到main函数结束
// 生命周期大于等于，编译成功

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}",i);
} // 被引用字符串novel生命周期到代码块后结束，ImportantExcerpt变量i的生命周期则从第一行到main函数结束
// 生命周期小于，编译失败
```

#### 生命周期消除

对于编译器来说，每一个引用类型都有一个生命周期，**为了简化用户的使用，运用了生命周期消除大法**

若函数返回引用，且类型返回值的引用是获取自参数，这就意味着参数和返回值的生命周期是一样的，所以就算不标注生命周期也不会产生歧义

* 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期
* **函数或者方法中，参数的生命周期被称为 `输入生命周期`，返回值的生命周期被称为 `输出生命周期`**

##### 三条消除规则

第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上

若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期

1. **每一个引用参数都会获得独自的生命周期**

   - 例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注:`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。
2. **若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期** ，也就是所有返回值的生命周期都等于该输入生命周期

   - 例如函数 `fn foo(x: &i32) -> &i32`，`x` 参数的生命周期会被自动赋给返回值 `&i32`，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`
3. **若存在多个输入生命周期，且其中一个是 `&self` 或 `&mut self`，则 `&self` 的生命周期被赋给所有的输出生命周期**

   - 拥有 `&self` 形式的参数，说明该函数是一个 `方法`，该规则让方法的使用便利度大幅提升

```rust
fn first_word(s: &str) -> &str { // 实际项目中的手写代码
fn first_word<'a>(s: &'a str) -> &str { // 编译器应用第一条规则自动为参数添加生命周期
fn first_word<'a>(s: &'a str) -> &'a str { // 编译器应用第二条规则自动为返回值添加生命周期

fn longest(x: &str, y: &str) -> &str { // 实际项目中的手写代码
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { // 编译器应用第一条规则为参数添加生命周期
// 无法应用第二条规则-输入生命周期有两个，也不符合第三条规则因为它是函数，因此报错提示需要手动标注生命周期
```

#### 方法中的生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```


* `impl` 中必须使用结构体的完整名称，包括 `<'a>`，因为 *生命周期标注也是结构体类型的一部分* ！
* 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 应用第一条规则给每个输入参数加上生命周期
impl<'a> ImportantExcerpt<'a> {
    // 编译器不知道 announcement 的生命周期到底多长，因此它无法简单的给予它生命周期 'a，而是重新声明了一个全新的生命周期 'b
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 应用第三条规则将&self的生命周期赋给返回值&str
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

- 若将方法返回的生命周期改为 `'b`，编译器将会报错
  - 由于 `&'a self` 是被引用的一方，因此引用它的 `&'b str` 必须要活得比它短，否则会出现悬垂引用，因此说明生命周期 `'b` 必须要比 `'a` 小

```rust
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

* `'a: 'b`，是生命周期约束语法，跟泛型约束非常相似，用于说明 `'a` 必须比 `'b` 活得久
* 可以把 `'a` 和 `'b` 都在同一个地方声明（如上），或者分开声明但通过 `where 'a: 'b` 约束生命周期关系

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

#### 静态生命周期

`'static`，拥有该生命周期的引用可以和整个程序活得一样久

- 字符串字面量

> 遇到因为生命周期导致的编译不通过问题，首先想的应该是：是否是我们试图创建一个悬垂引用，或者是试图匹配不一致的生命周期，而不是简单粗暴的用 `'static` 来解决问题
>
> `'static` 确实可以帮助我们解决非常复杂的生命周期问题甚至是无法被手动解决的生命周期问题，那么此时就应该放心大胆的用，只要确定：**所有引用的生命周期都是正确的，只是编译器太笨不懂罢了**

#### 一个复杂的例子

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display, // 因为要用格式化 {} 来输出 ann，因此需要它实现 Display 特征
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
