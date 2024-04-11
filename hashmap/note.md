### KV存储HashMap

`HashMap` 中存储的是一一映射的 `KV` 键值对，并提供了平均复杂度为 `O(1)` 的查询方法

#### 创建HashMap

使用 `HashMap` 需要手动通过 `use ...` 从标准库中引入到我们当前的作用域中来

##### 使用new方法创建

使用 `new` 方法来创建 `HashMap`，然后通过 `insert` 方法插入键值对

```rust
use std::collections::HashMap;

// 创建一个HashMap，用于存储宝石种类和对应的数量
let mut my_gems = HashMap::new();

// 将宝石类型和对应的数量写入表中
my_gems.insert("红宝石", 1);
my_gems.insert("蓝宝石", 2);
my_gems.insert("河边捡的误以为是宝石的破石头", 18);
```

- 所有的集合类型都是动态的，意味着它们没有固定的内存大小，因此它们底层的数据都存储在内存堆上，然后通过一个存储在栈中的引用类型来访问
- `HashMap` 也是内聚性的，即所有的 `K` 必须拥有同样的类型，`V` 也是如此

> 如果预先知道要存储的 `KV` 对个数，可以使用 `HashMap::with_capacity(capacity)` 创建指定大小的 `HashMap`，避免频繁的内存分配和拷贝，提升性能

##### 使用迭代器和collect方法创建

从另外一个数据结构中，获取到对应的数据，最终生成 `HashMap`

```rust
fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // 笨办法-迭代
    // let mut teams_map = HashMap::new();
    // for team in &teams_list {
    //     teams_map.insert(&team.0, team.1);
    // }

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
  
    println!("{:?}",teams_map)
}
```

- `collect` 方法在内部实际上支持生成多种类型的目标集合，因此我们需要通过类型标注 `HashMap<_,_>` 来告诉编译器：请帮我们收集为 `HashMap` 集合类型，具体的 `KV` 类型，麻烦编译器您老人家帮我们推导

#### 所有权转移

* 若类型实现 `Copy` 特征，该类型会被复制进 `HashMap`，因此无所谓所有权
* 若没实现 `Copy` 特征，所有权将被转移给 `HashMap` 中
* **如果使用引用类型放入 HashMap 中** ，请确保该引用的生命周期至少跟 `HashMap` 活得一样久

```rust
fn main() {
    use std::collections::HashMap;

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    std::mem::drop(name); // 'name' move out
    println!("因为过于无耻，{:?}已经被除名", handsome_boys); // Error: borrowed 'name'
    println!("还有，他的真实年龄远远不止{}岁", age);
}
```

#### 查询HashMap

通过 `get` 方法可以获取元素

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name);
```

* `get` 方法返回一个 `Option<&i32>` 类型：当查询不到时，会返回一个 `None`，查询到时返回 `Some(&i32)`
* `&i32` 是对 `HashMap` 中值的借用，如果不使用借用，可能会发生所有权的转移

```rust
// 直接获得值类型的 score
let score: i32 = scores.get(&team_name).copied().unwrap_or(0);
```

通过循环的方式依次遍历`KV`对

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

#### 更新HashMap中的值

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
```

##### 在已有值的基础上更新

查询某个 `key` 对应的值，若不存在则插入新值，若存在则对已有的值进行更新

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();
// 根据空格来切分字符串(英文单词都是通过空格切分)
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

* `or_insert` 返回了 `&mut v` 引用，因此可以通过该可变引用直接修改 `map` 中对应的值
* 使用 `count` 引用时，需要先进行解引用 `*count`，否则会出现类型不匹配

#### 哈希函数

一个类型能否作为 `Key` 的关键就是是否能进行相等比较，或者说该类型是否实现了 `std::cmp::Eq` 特征

- f32 和 f64 浮点数，没有实现 `std::cmp::Eq` 特征，因此不可以用作 `HashMap` 的 `Key`

哈希函数：通过它把 `Key` 计算后映射为哈希值，然后使用该哈希值来进行存储、查询、比较等操作

- 若要追求安全，尽可能减少冲突，同时防止拒绝服务（Denial of Service, DoS）攻击，就要使用密码学安全的哈希函数，`HashMap` 就是使用了这样的哈希函数
- 反之若要追求性能，就需要使用没有那么安全的算法。

##### 高性能第三方库

crate.io寻找其它的哈希函数实现

```rust
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// 引入第三方的哈希函数
use twox_hash::XxHash64;

// 指定HashMap使用第三方的哈希函数XxHash64
let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
hash.insert(42, "the answer");
assert_eq!(hash.get(&42), Some(&"the answer"));
```

> `HashMap` 使用的哈希函数是 `SipHash`，它的性能不是很高，但是安全性很高。`SipHash` 在中等大小的 `Key` 上，性能相当不错，但是对于小型的 `Key` （例如整数）或者大型 `Key` （例如字符串）来说，性能还是不够好。若你需要极致性能，例如实现算法，可以考虑这个库：[ahash](https://github.com/tkaitchuck/ahash)
