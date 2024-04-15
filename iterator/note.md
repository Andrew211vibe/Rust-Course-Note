### 迭代器

迭代器允许我们迭代一个连续的集合，在此过程中，只需关心集合中的元素如何处理，而无需关心如何开始、如何结束、按照什么样的索引去访问等问题

#### For循环与迭代器

迭代器跟 `for` 循环颇为相似，都是去遍历一个集合，但是实际上它们存在不小的差别，其中最主要的差别就是：**是否通过索引来访问集合**

```rust
let arr = [1, 2, 3];
for v in arr {
    println!("{}",v);
}
```

数组实现了 `IntoIterator` 特征，Rust 通过 `for` 语法糖，自动把实现了该特征的数组类型转换为迭代器（你也可以为自己的集合类型实现此特征），最终让我们可以直接对一个数组进行迭代

```rust
for i in 1..10 {
    println!("{}", i);
}
```

直接对数值序列进行迭代，也是很常见的使用方式

`IntoIterator` 特征拥有一个 `into_iter` 方法，因此我们还可以显式的把数组转换成迭代器

```rust
let arr = [1, 2, 3];
for v in arr.into_iter() {
    println!("{}", v);
}
```

#### 惰性初始化

迭代器是惰性的，意味着如果你不使用它，那么它将不会发生任何事

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
for val in v1_iter {
    println!("{}", val);
}
```

这种惰性初始化的方式确保了创建迭代器不会有任何额外的性能损耗，其中的元素也不会被消耗，只有使用到该迭代器的时候，一切才开始

#### `next`方法

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // 省略其余有默认实现的方法
}
```

**迭代器之所以成为迭代器，就是因为实现了 `Iterator` 特征** 

要实现该特征，最主要的就是实现其中的 `next` 方法，该方法控制如何从集合中取值，最终返回值的类型是关联类型 `Item`

`for` 循环通过不停调用迭代器上的 `next` 方法，来获取迭代器中的元素

```rust
fn main() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();

    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}
```

* `next` 方法返回的是 `Option` 类型，当有值时返回 `Some(i32)`，无值时返回 `None`
* 遍历是按照迭代器中元素的排列顺序依次进行的，因此我们严格按照数组中元素的顺序取出了 `Some(1)`，`Some(2)`，`Some(3)`
* 手动迭代必须将迭代器声明为 `mut` 可变，因为调用 `next` 会改变迭代器其中的状态数据（当前遍历的位置等），而 `for` 循环去迭代则无需标注 `mut`，因为它会帮我们自动完成

`next` 方法对 **迭代器的遍历是消耗性的** ，每次消耗它一个元素，最终迭代器中将没有任何元素，只能返回 `None`

##### 例子：模拟实现for循环

```rust
let values = vec![1, 2, 3];

{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{}", x); },
                None => break,
            }
        },
    };
    result
}
```

`IntoIterator::into_iter` 是使用*完全限定*的方式去调用 `into_iter` 方法，这种调用方式跟 `values.into_iter()` 是等价的

#### `IntoIterator`特征

迭代器自身也实现了 `IntoIterator`

```rust
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}

fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter().into_iter().into_iter() {
        println!("{}",v)
    }
}
```

##### `into_iter`/`iter`/`iter_mut`

* `into_iter` 会夺走所有权
* `iter` 是借用
* `iter_mut` 是可变借用

```rust
fn main() {
    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
}
```

* `.iter()`方法实现的迭代器，调用 `next`方法返回的类型是 `Some(&T)`
* `.iter_mut()` 方法实现的迭代器，调用 `next` 方法返回的类型是 `Some(&mut T)`，因此在 `if let Some(v) = values_iter_mut.next()`中，`v`的类型是 `&mut i32`，最终我们可以通过 `*v = 0`的方式修改其值

##### `Iterator`和`IntoIterator`的区别

* `Iterator` 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 `next`
* `IntoIterator` 强调的是某一个类型如果实现了该特征，它可以通过 `into_iter`，`iter` 等方法变成一个迭代器

#### 消费者与适配器

消费者是迭代器上的方法，它会消费掉迭代器中的元素，然后返回其类型的值

这些消费者都有一个共同的特点：在它们的定义中，都依赖 `next`方法来消费元素，因此这也是为什么迭代器要实现 `Iterator`特征，而该特征必须要实现 `next` 方法的原因

##### 消费者适配器

只要迭代器上的某个方法 `A` 在其内部调用了 `next`方法，那么 `A`就被称为**消费性适配器**

* 因为 `next`方法会消耗掉迭代器上的元素，所以方法 `A`的调用也会消耗掉迭代器上的元素

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}",v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);
}
```

从 `sum` 源码中也可以清晰看出，`self` 类型的方法参数拿走了所有权

```rust
fn sum<S>(self) -> S
where
    Self: Sized,
    S: Sum<Self::Item>,
{
    Sum::sum(self)
}
```

##### 迭代器适配器

迭代器适配器，会返回一个新的迭代器，这是实现链式方法调用的关键：`v.iter().map().filter()...`

迭代器适配器是惰性的，意味着你**需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值**

```rust
let v1: Vec<i32> = vec![1, 2, 3];
v1.iter().map(|x| x + 1); // Error: map是惰性的

let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

##### `collect`

`collect` 方法，该方法就是一个消费者适配器，使用它可以将一个迭代器中的元素收集到指定类型中

可以收集成多种不同的集合类型，`Vec<T>` 仅仅是其中之一，因此我们必须显式的告诉编译器我们想要收集成的集合类型

`map` 会对迭代器中的每一个值进行一系列操作，然后把该值转换成另外一个新值

```rust
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}",folks);
}
```

`zip` 是一个迭代器适配器，它的作用就是将两个迭代器的内容压缩到一起，形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器

* 在此处就是形如 `[(name1, age1), (name2, age2)]` 的迭代器

> 通过 `collect` 将新迭代器中 `(K, V)` 形式的值收集成 `HashMap<K, V>`，同样的，这里必须显式声明类型，然后 `HashMap` 内部的 `KV` 类型可以交给编译器去推导，最终编译器会推导出 `HashMap<&str, i32>`

##### 闭包作为适配器参数

好处不仅在于可以就地实现迭代器中元素的处理，还在于可以捕获环境值

```rust
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

`filter` 是迭代器适配器，用于对迭代器中的每个值进行过滤

#### 实现Iterator特征

创建自己的迭代器 —— 只要为自定义类型实现 `Iterator` 特征即可

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // 将该特征的关联类型设置为u32

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

每次调用 `next` 方法，都会让计数器的值加一，然后返回最新的计数值，一旦计数大于 5，就返回 `None`

##### 实现Iterator特征的其它方法

`Iterator`特征中其它方法都具有默认实现，且都是基于 `next`方法实现的

```rust
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
assert_eq!(18, sum);
```

`zip`，`map`，`filter`是迭代器适配器：

* `zip`把两个迭代器合并成一个迭代器，新迭代器中，每个元素都是一个元组，由之前两个迭代器的元素组成
  * 例如将**形如** `[1, 2, 3, 4, 5]`和 `[2, 3, 4, 5]`的迭代器合并后，新的迭代器形如 `[(1, 2),(2, 3),(3, 4),(4, 5)]`
* `map`是将迭代器中的值经过映射后，转换成新的值[2, 6, 12, 20]
* `filter`对迭代器中的元素进行过滤，若闭包返回 `true` 则保留元素[6, 12]，反之剔除

`sum`是消费者适配器，对迭代器中的所有元素求和，最终返回一个 `u32`值 `18`

##### `enumerate`

```rust
let v = vec![1u64, 2, 3, 4, 5, 6];
for (i,v) in v.iter().enumerate() {
    println!("第{}个值是{}",i,v)
}
```

`v.iter()` 创建迭代器，其次 调用 `Iterator` 特征上的方法 `enumerate`，该方法产生一个新的迭代器，其中每个元素均是元组 `(索引，值)`

因为 `enumerate` 是迭代器适配器，因此我们可以对它返回的迭代器调用其它 `Iterator` 特征方法

```rust
let v = vec![1u64, 2, 3, 4, 5, 6];
let val = v.iter()
    .enumerate()
    // 每两个元素剔除一个
    // [1, 3, 5]
    .filter(|&(idx, _)| idx % 2 == 0)
    .map(|(_, val)| val)
    // 累加 1+3+5 = 9
    .fold(0u64, |sum, acm| sum + acm);

println!("{}", val);
```

#### 迭代器的性能

```rust
#![feature(test)]

extern crate rand;
extern crate test;

fn sum_for(x: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum::<f64>()
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use rand::{Rng,thread_rng};
    use super::*;

    const LEN: usize = 1024*1024;

    fn rand_array(cnt: u32) -> Vec<f64> {
        let mut rng = thread_rng();
        (0..cnt).map(|_| rng.gen::<f64>()).collect()
    }

    #[bench]
    fn bench_for(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_for(&samples)
        })
    }

    #[bench]
    fn bench_iter(b: &mut Bencher) {
        let samples = rand_array(LEN as u32);
        b.iter(|| {
            sum_iter(&samples)
        })
    }
}
```

> 迭代器要更快一点
>
> 迭代器是 Rust 的  **零成本抽象** （zero-cost abstractions）之一，意味着抽象并不会引入运行时开销
>
> 与C++的**零开销**如出一辙
>
> C++的实现遵循零开销原则：没有使用时，你不必为其买单。 更进一步说，需要使用时，你也无法写出更优的代码了

编译器还可以通过循环展开（Unrolling）、向量化、消除边界检查等优化手段，使得迭代器和 `for` 循环都有极为高效的执行效率
