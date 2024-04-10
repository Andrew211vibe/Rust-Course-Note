#### 有理数和复数

- 不包含于标准库内
- Rust数值库：num

按照以下步骤来引入 `num` 库：

1. 创建新工程 `cargo new complex-num && cd complex-num`
2. 在 `Cargo.toml` 中的 `[dependencies]` 下添加一行 `num = "0.4.0"`
3. 编写代码，通过 `usenum::complex::Complex;`导入库
4. 运行 `cargo run`
