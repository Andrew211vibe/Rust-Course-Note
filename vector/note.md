### 动态数组Vector

- 动态数组允许你存储多个值，这些值在内存中一个紧挨着另一个排列，因此访问其中某个元素的成本非常低
- 动态数组只能存储相同类型的元素，如果想存储不同类型的元素，可以使用之前讲过的枚举类型或者特征对象

#### 创建动态数组

##### `Vec::new`

调用了 `Vec` 中的 `new` 关联函数

```rust
let v: Vec<i32> = Vec::new();
```

- 显式声明类型为 `Vec<i32>`，因为Rust无法从 `Vec::new()`中得到任何关于元素类型的信息，也就无法推导出 `v`的类型

```rust
let mut v = Vec::new();
v.push(1);
```

- 此时Rust从 `v.push(1)`得到元素类型为 `i32`，因此推导出 `v`的类型是 `Vec<i32>`

> 如果预先知道要存储的元素个数，可以使用 `Vec::with_capacity(capacity)` 创建动态数组，这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能

##### `vec![]`

使用宏 `vec!` 来创建数组，能在创建同时给予初始化值

```rust
let v = vec![1, 2, 3];
```

#### 更新Vector

向数组尾部添加元素，可以使用 `push` 方法（需提前声明为`mut`）

```rust
let mut v = Vec::new();
v.push(1);
```

#### Vector与其元素共存亡

```rust
{
    let v = vec![1, 2, 3];

    // ...
} // <- v超出作用域并在此处被删除
```

当 `Vector` 被删除后，它内部存储的所有内容也会随之被删除（当心对`Vector`引用造成空悬指针）

#### 从Vector中读取元素

* 通过下标索引访问。
* 使用 `get` 方法。

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {third}"),
    None => println!("去你的第三个元素，根本没有！"),
}
```

- `&v[2]` 表示借用 `v` 中的第三个元素，最终会获得该元素的引用
- `v.get(2)` 也是访问第三个元素，但是有所不同的是，它返回了 `Option<&T>`

##### 下标索引与 `.get`的区别

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

- 访问越界元素时，`&v[100]` 的访问方式会导致程序无情报错退出，但是 `v.get` 就不会，它在内部做了处理，有值的时候返回 `Some(T)`，无值的时候返回 `None`，因此 `v.get` 的使用方式非常安全

#### 同时借用多个数组元素

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```

- 根据引用规则，该代码报错，因为不可变引用`first`在`v.push(6)`之后使用过

> 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存

#### 迭代遍历Vector中的元素

使用迭代的方式去遍历数组，这种方式比用下标的方式去遍历数组更安全也更高效（每次下标访问都会触发数组边界检查）

```rust
let v = vec![1, 2, 3];
for i in &v {
    println!("{i}");
}
```

也可以在迭代过程中，修改 `Vector` 中的元素

```rust
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i += 10
}
```

#### 存储不同类型的元素

通过使用枚举类型和特征对象来实现不同类型元素的存储

```rust
// 通过枚举
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}
fn main() {
    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    for ip in v {
        show_addr(ip)
    }
}
fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}

// 通过特征对象
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
fn main() {
    // 必须手动地指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是特征 IpAddr 的对象
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
```

#### Vector常用方法

初始化的更多方式

```rust
fn main() {
    let v = vec![0; 3];   // 默认值为 0，初始长度为 3
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);
}
```

> 动态数组意味着我们增加元素时，如果 **容量不足就会导致 vector 扩容** （目前的策略是重新申请一块 2 倍大小的内存，再将所有元素拷贝到新的内存位置，同时更新指针数据），显然，当频繁扩容或者当元素数量较多且需要扩容时，大量的内存拷贝会降低程序的性能

在初始化时就指定一个实际的预估容量

```rust
fn main() {
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);    // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100);        // 调整 v 的容量，至少要有 100 的容量
    println!("Vector（reserve） 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.shrink_to_fit();     // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!("Vector（shrink_to_fit） 长度是: {}, 容量是: {}", v.len(), v.capacity());
}
```

常见方法

```rust
let mut v =  vec![1, 2];
assert!(!v.is_empty());         // 检查 v 是否为空

v.insert(2, 3);                 // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3] 
assert_eq!(v.remove(1), 2);     // 移除指定位置的元素并返回, v: [1, 3]
assert_eq!(v.pop(), Some(3));   // 删除并返回 v 尾部的元素，v: [1]
assert_eq!(v.pop(), Some(1));   // v: []
assert_eq!(v.pop(), None);      // 记得 pop 方法返回的是 Option 枚举值
v.clear();                      // 清空 v, v: []

let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
v.append(&mut v1);              // 将 v1 中的所有元素附加到 v 中, v1: []
v.truncate(1);                  // 截断到指定长度，多余的元素被删除, v: [11]
v.retain(|x| *x > 10);          // 保留满足条件的元素，即删除不满足条件的元素

let mut v = vec![11, 22, 33, 44, 55];
// 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
let mut m: Vec<_> = v.drain(1..=3).collect();  

let v2 = m.split_off(1);        // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
```

切片

```rust
fn main() {
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);
}
```

#### Vector的排序

rust 里，实现了两种排序算法，分别为稳定的排序 `sort` 和 `sort_by`，以及非稳定排序 `sort_unstable` 和 `sort_unstable_by`

- 在 `稳定` 排序算法里，对相等的元素，不会对其进行重新排序，而在 `不稳定` 的算法里则不保证这点

##### 整数数组的排序

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];  
    vec.sort_unstable();  
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

##### 浮点数数组的排序

```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];  
    vec.sort_unstable();  
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
} // Error
```

- 浮点数当中，存在一个 `NAN` 的值，这个值无法与其他的浮点数进行对比，因此，浮点数类型并没有实现全数值可比较 `Ord` 的特性，而是实现了部分可比较的特性 `PartialOrd`
- 确定在我们的浮点数数组当中，不包含 `NAN` 值，那么可以使用 `partial_cmp` 来作为大小判断的依据

```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];  
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());  
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```

##### 对结构体数组排序

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);
}
```

- 实现 `Ord` 需要我们实现 `Ord`、`Eq`、`PartialEq`、`PartialOrd` 这些属性，或直接 `derive`
- `derive` `Ord` 相关特性，需要确保你的结构体中所有的属性均实现了 `Ord` 相关特性，否则会发生编译错误
- `derive` 的默认实现会依据属性的顺序依次进行比较

```rust
#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}
```
