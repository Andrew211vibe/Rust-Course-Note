### 语句与表达式

- Rust的函数体是由一系列语句组成，最后由一个表达式来返回值
- ```rust
  fn add_with_extra(x: i32, y: i32) {
      let x = x + 1; // 语句
      let y = y + 5; // 语句
      x + y // 表达式
  }
  ```

  - 语句会执行一些操作但不会返回一个值，但是表达式会在求值后返回一个值
  - **这种基于语句和表达式的方式非常重要，注意明确区分**
  - **表达式总要返回值**

#### 语句

```rust
let a = 8;
let b: Vec<f64> = Vec::new();
let (a, c) = ("hi", false);
```

- 以上语句完成了一个具体的操作，但是并没有返回值，因此是语句

```rust
let a = (let b = 0);

/*
error: expected expression, found statement (`let`) // 期望表达式，却发现`let`语句
 --> src/main.rs:2:13
  |
2 |     let a = let b = 0;
  |             ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement `let`是一条语句
*/
```

- 由于 `let`是语句，因此不能将 `let`语句赋给其它值

#### 表达式

- 表达式会进行求值后返回一个值

  - `5 + 6`是一个表达式，求值后返回 `11`
- 表达式可以成为语句的一部分

  - `let x = 6`中的 `6`就是一个表达式，它在求值后返回 `6`
- 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值，它就是表达式：

  - ```rust
    fn main() {
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {}", y);
    }

    // 使用一个语句块表达式将值赋给y
    {
        let x = 3;
        x + 1
    }
    ```
  - 该语句块是表达式的原因是：它的最后一行是表达式，返回了 `x + 1`的值，注意 `x + 1`不能以 `;`结尾，否则就会从表达式变成语句（**表达式不能包含分号**）
  - 一旦在表达式后加上分号，它就会变成一个语句，再也不会返回一个值
- 表达式若不返回任何值，将隐式地返回一个 `()`

```rust
fn main() {
    assert_eq!(ret_unit_type(), ())
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let z = if x % 2 == 1 { "odd" } else { "even" };
}
```
