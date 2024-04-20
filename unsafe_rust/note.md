### 五种兵器

#### 解引用裸指针

裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性

- 和引用有所不同，裸指针长这样: `*const T` 和 `*mut T`，它们分别代表了不可变和可变
- `*`操作符，可以用于解引用，但是在裸指针 `*const T` 中，这里的 `*`只是类型名称的一部分，并没有解引用的含义
- 可以绕过Rust的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
- 并不能保证指向合法的内存
- 可以是 `null`
- 没有实现任何自动的回收 (drop)

##### 基于引用创建裸指针

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

- 将引用 `&num / &mut num`强转为相应的裸指针 `*const i32 / *mut i32`
- **创建裸指针是安全的行为，而解引用裸指针才是不安全的行为**

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }
}
```

##### 基于内存地址创建裸指针

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

试图使用任意的内存地址往往是一种未定义的行为(undefined behavior)，因为该内存地址有可能存在值，也有可能没有，就算有值，也大概率不是你需要的值

编译器也有可能会优化这段代码，会造成没有任何内存访问发生，甚至程序还可能发生段错误(segmentation fault)

**总之，你几乎没有好的理由像上面这样实现代码，虽然它是可行的**

```rust
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
  let string = "Hello World!";
  let pointer = string.as_ptr() as usize;
  let length = string.len();
  (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
  unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn main() {
  let (pointer, length) = get_memory_location();
  let message = get_str_at_location(pointer, length);
  println!(
    "The {} bytes at 0x{:X} stored: {}",
    length, pointer, message
  );
  // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
  // let message = get_str_at_location(1000, 10);
}
```

- 先取地址，再使用，而不是凭空捏造一个地址

##### 使用 `*`解引用

使用 `*`可以对裸指针进行解引用，由于该指针的内存安全性并没有任何保证，因此需要使用 `unsafe`来包裹解引用的逻辑(切记，`unsafe`语句块的范围一定要尽可能的小）

```rust
let a = 1;
let b: *const i32 = &a as *const i32;
let c: *const i32 = &a;
unsafe {
    println!("{}", *c);
}
```

- 除了使用 `as`来显式的转换，还使用了隐式的转换方式 `let c: *const i32 = &a;`
- 建议使用 `as`来转换

##### 基于智能指针创建裸指针

```rust
let a: Box<i32> = Box::new(10);
// 需要先解引用a
let b: *const i32 = &*a;
// 使用 into_raw 来创建
let c: *const i32 = Box::into_raw(a);
```

##### 小结

> 使用裸指针可以让我们创建两个可变指针都指向同一个数据，如果使用安全的 Rust，你是无法做到这一点的，违背了借用规则，编译器会对我们进行无情的阻止。因此裸指针可以绕过借用规则，但是由此带来的数据竞争问题，就需要大家自己来处理了
>
> 除了之前提到的性能等原因，还有一个重要用途就是跟 `C` 语言的代码进行交互( FFI )

#### 调用 `unsafe`函数或方法

`unsafe`函数从外表上来看跟普通函数并无区别，唯一的区别就是它需要使用 `unsafe fn`来进行定义

- 为了告诉调用者：当调用此函数时，你需要注意它的相关需求，因为Rust无法担保调用者在使用该函数时能满足它所需的一切需求

```rust
unsafe fn dangerous() {}
fn main() {
    unsafe {
        dangerous();
    }
}
```

> **使用 unsafe 声明的函数时，一定要看看相关的文档，确定自己没有遗漏什么**
>
> 在 `unsafe`函数体中使用 `unsafe`语句块是多余的行为

#### 用安全抽象包裹 `unsafe`代码

一个函数包含了 `unsafe`代码不代表我们需要将整个函数都定义为 `unsafe fn`

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

- 报错：试图在自定义的 `split_at_mut`函数中，可变借用 `slice`两次
- 对于Rust的借用检查器来说，它无法理解我们是分别借用了同一个切片的两个不同部分

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

* `as_mut_ptr`会返回指向 `slice`首地址的裸指针 `*mut i32`
* `slice::from_raw_parts_mut` 函数通过指针和长度来创建一个新的切片，简单来说，该切片的初始地址是 `ptr`，长度为 `mid`
* `ptr.add(mid)`可以获取第二个切片的初始地址，由于切片中的元素是 `i32`类型，每个元素都占用了 4 个字节的内存大小，因此我们不能简单的用 `ptr + mid`来作为初始地址，而应该使用 `ptr + 4 * mid`，但是这种使用方式并不安全，因此 `.add`方法是最佳选择
* 秘诀就在于 `assert!(mid <= len);` ，通过这个断言，我们保证了裸指针一定指向了 `slice`切片中的某个元素，而不是一个莫名其妙的内存地址
* **虽然 split_at_mut 使用了 `unsafe`，但我们无需将其声明为 `unsafe fn`** ，这种情况下就是使用安全的抽象包裹 `unsafe`代码，这里的 `unsafe`使用是非常安全的，因为我们从合法数据中创建了的合法指针

```rust
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
println!("{:?}",slice);
```

> 这段代码从一个任意的内存地址，创建了一个10000长度的 `i32`切片，我们无法保证切片中的元素都是合法的 `i32`值，这种访问就是一种未定义行为(UB = undefined behavior)

#### FFI

`FFI`（Foreign Function Interface）可以用来与其它语言进行交互

果需要使用某个库，但是它是由其它语言编写的，那么往往只有两个选择：

* 对该库进行重写或者移植
* 使用 `FFI`

除了 `FFI`还有一个办法可以解决跨语言调用的问题，那就是将其作为一个独立的服务，然后使用网络调用的方式去访问，HTTP，gRPC 都可以

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

> 必须使用 `unsafe`才能进行进行调用，原因在于其它语言的代码并不会强制执行Rust的规则，因此Rust无法对这些代码进行检查，最终还是要靠开发者自己来保证代码的正确性和程序的安全性

##### ABI

> 在 `extern "C"`代码块中，我们列出了想要调用的外部函数的签名。其中 `"C"`定义了外部函数所使用的**应用二进制接口** `ABI` (Application Binary Interface)：`ABI`定义了如何在汇编层面来调用该函数

##### 在其它语言中调用Rust函数

可以使用 `extern`来创建一个接口，其它语言可以通过该接口来调用相关的Rust函数

别忘了指定相应的 `ABI`

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

- 上面的代码可以让 `call_from_c`函数被C语言的代码调用，当然，前提是将其编译成一个共享库，然后链接到C语言中
- `#[no_mangle]`，用于告诉Rust编译器：不要乱改函数的名称
  - `Mangling`的定义是：当Rust因为编译需要去修改函数的名称，例如为了让名称包含更多的信息，这样其它的编译部分就能从该名称获取相应的信息，这种修改会导致函数名变得相当不可读

#### 实现unsafe特征

之所以会有 `unsafe`的特征，是因为该特征至少有一个方法包含有编译器无法验证的内容

```rust
unsafe trait Foo {
    // 方法列表
}

unsafe impl Foo for i32 {
    // 实现相应的方法
}

fn main() {}
```

- 通过 `unsafe impl`的使用，我们告诉编译器：相应的正确性由我们自己来保证

#### 访问union中的字段

主要用于跟C代码进行交互

访问 `union`的字段是不安全的，因为 Rust 无法保证当前存储在 `union`实例中的数据类型

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```

- 所有字段都共享同一个存储空间，意味着往 `union`的某个字段写入值，会导致其它字段的值会被覆盖

#### 一些实用工具（库）

##### rust-bindgen和cbindgen

对于 `FFI`调用来说，保证接口的正确性是非常重要的，这两个库可以帮我们自动生成相应的接口，其中 [`rust-bindgen`](https://github.com/rust-lang/rust-bindgen)用于在 Rust 中访问 C 代码，而 [`cbindgen`](https://github.com/eqrion/cbindgen/)则反之

```c
typedef struct Doggo {
    int many;
    char wow;
} Doggo;

void eleven_out_of_ten_majestic_af(Doggo* pupper);
```

- 自动生成的可以调用上面代码的Rust代码

```rust
/* automatically generated by rust-bindgen 0.99.9 */

#[repr(C)]
pub struct Doggo {
    pub many: ::std::os::raw::c_int,
    pub wow: ::std::os::raw::c_char,
}

extern "C" {
    pub fn eleven_out_of_ten_majestic_af(pupper: *mut Doggo);
}
```

##### CXX

如果需要跟C++代码交互，非常推荐使用 [`cxx`](https://github.com/dtolnay/cxx)，它提供了双向的调用，最大的优点就是安全：是的，你无需通过 `unsafe`来使用它

##### Miri

[`miri`](https://github.com/rust-lang/miri) 可以生成Rust的中间层表示MIR，对于编译器来说，Rust代码首先会被编译为MIR ，然后再提交给LLVM进行处理

可以通过 `rustup component add miri`来安装它，并通过 `cargo miri`来使用，同时还可以使用 `cargo miri test`来运行测试代码

`miri`可以帮助我们检查常见的未定义行为(UB = Undefined Behavior)，以下列出了一部分:

* 内存越界检查和内存释放后再使用(use-after-free)
* 使用未初始化的数据
* 数据竞争
* 内存对齐问题

但是需要注意的是，它只能帮助识别被执行代码路径的风险，那些未被执行到的代码是没办法被识别的

##### Clippy

[`clippy`](https://github.com/rust-lang/rust-clippy)检查器提供了有限的 `unsafe`支持，虽然不多，但是至少有一定帮助

Rust编译器并不会默认开启所有检查，大家可以调用 `rustc -W help`来看看最新的信息

##### Prusti

[`prusti`](https://viperproject.github.io/prusti-dev/user-guide/)需要大家自己来构建一个证明，然后通过它证明代码中的不变量是正确被使用的，当你在安全代码中使用不安全的不变量时，就会非常有用。具体的使用文档见[这里](https://viperproject.github.io/prusti-dev/user-guide/)

##### 模糊测试（fuzz testing）

在[Rust Fuzz Book](https://rust-fuzz.github.io/book/)中列出了一些Rust可以使用的模糊测试方法。

还可以使用 [`rutenspitz`](https://github.com/jakubadamw/rutenspitz)这个过程宏来测试有状态的代码

### 内联汇编

#### 基本用法

插入一个 `NOP`指令( 空操作 ) 到编译器生成的汇编代码中，其中指令作为 `asm!`的第一个参数传入

```rust
use std::arch::asm;

unsafe {
    asm!("nop");
}
```

- 注意 `unsafe`语句块依然是必不可少的，因为可能在里面插入危险的指令，最终破坏代码的安全性

#### 输入和输出

将 `5`赋给 `u64`类型的变量 `x`

```rust
use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);
```

- `asm!`的指令参数实际上是一个格式化字符串
- 传给格式化字符串的参数：
  - 首先，需要说明目标变量是作为内联汇编的输入还是输出，在本例中，是一个输出 `out`
  - 最后，要指定变量将要使用的寄存器，本例中使用通用寄存器 `reg`，编译器会自动选择合适的寄存器

```rust
use std::arch::asm;

let i: u64 = 3;
let o: u64;
unsafe {
    asm!(
        "mov {0}, {1}",
        "add {0}, 5",
        out(reg) o,
        in(reg) i,
    );
}
assert_eq!(o, 8);
```

- 使用了输入 `in`，将 `5`加到输入的变量 `i`上，然后将结果写到输出变量 `o`
- `asm!`允许使用多个格式化字符串，每一个作为单独一行汇编代码存在，看起来跟阅读真实的汇编代码类似
- 输入变量通过 `in`来声明
- 和以前见过的格式化字符串一样，可以使用多个参数，通过 `{0}`, `{1}`来指定，这种方式特别有用，毕竟在代码中，变量是经常复用的，而这种参数的指定方式刚好可以复用

```rust
use std::arch::asm;

let mut x: u64 = 3;
unsafe {
    asm!("add {0}, 5", inout(reg) x);
}
assert_eq!(x, 8);
```

- `inout`关键字，说明 `x`即是输入又是输出，这种方式可以保证使用同一个寄存器来完成任务
- 可以在使用 `inout`的情况下，指定不同的输入和输出

```rust
use std::arch::asm;

let x: u64 = 3;
let y: u64;
unsafe {
    asm!("add {0}, 5", inout(reg) x => y);
}
assert_eq!(y, 8);
```

#### 延迟输出操作数

Rust编译器对于操作数分配是较为保守的，它会假设 `out` 可以在任何时间被写入，因此 `out` 不会跟其它参数共享它的位置

然而为了保证最佳性能，使用尽量少的寄存器是有必要的，这样它们不必在内联汇编的代码块内保存和重加载

为了达成这个目标( 共享位置或者说寄存器，以实现减少寄存器使用的性能优化 )，Rust提供一个 `lateout`关键字，可以用于任何只在所有输入被消费后才被写入的输出，与之类似的还有一个 `inlateout`

`inlateout`在某些场景中是无法使用的：

```rust
use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
let c: u64 = 4;
unsafe {
    asm!(
        "add {0}, {1}",
        "add {0}, {2}",
        inout(reg) a,
        in(reg) b,
        in(reg) c,
    );
}
assert_eq!(a, 12);
```

- 一旦用了 `inlateout`后，上面的代码就只能运行在 `Debug`模式下，原因是 `Debug`并没有做任何优化，但是 `release`发布不同，为了性能是要做很多编译优化的

> 在编译优化时，编译器可以很容易的为输入 `b`和 `c`分配同样的是寄存器，因为它知道它们有同样的值。如果这里使用 `inlateout`， 那么 `a`和 `c`就可以被分配到相同的寄存器，在这种情况下，第一条指令将覆盖掉 `c`的值，最终导致汇编代码产生错误的结果

- 因此这里使用 `inout`，那么编译器就会为 `a`分配一个独立的寄存器

```rust
use std::arch::asm;

let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
}
assert_eq!(a, 8);
```

- 输出只有在所有寄存器都被读取后，才被修改。因此，即使 `a`和 `b`被分配了同样的寄存器，代码也会正常工作，不存在之前的覆盖问题

#### 显式指定寄存器

一些指令会要求操作数只能存在特定的寄存器中，因此Rust的内联汇编提供了一些限制操作符

`reg`是适用于任何架构的通用寄存器，意味着编译器可以自己选择合适的寄存器，但是当你需要显式地指定寄存器时，很可能会变成平台相关的代码，适用移植性会差很多

```rust
use std::arch::asm;

let cmd = 0xd1;
unsafe {
    asm!("out 0x64, eax", in("eax") cmd);
}
```

- 调用 `out`指令将 `cmd`变量的值输出到 `0x64`内存地址中
- 由于 `out`指令只接收 `eax`和它的子寄存器，因此我们需要使用 `eax`来指定特定的寄存器

> 显式寄存器操作数无法用于格式化字符串中，例如我们之前使用的 {}，只能直接在字符串中使用 `eax`。同时，该操作数只能出现在最后，也就是在其它操作数后面出现

```rust
use std::arch::asm;

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            // The x86 mul instruction takes rax as an implicit input and writes
            // the 128-bit result of the multiplication to rax:rdx.
            "mul {}",
            in(reg) a,
            inlateout("rax") b => lo,
            lateout("rdx") hi
        );
    }

    ((hi as u128) << 64) + lo as u128
}
```

- 使用了 `mul`指令，将两个64位的输入相乘，生成一个128位的结果

> 首先将变量 `a` 的值存到寄存器 `reg`中，其次显式使用寄存器 `rax`，它的值来源于变量 `b`。结果的低 64 位存储在 `rax`中，然后赋给变量 `lo`，而结果的高 64 位则存在 `rdx`中，最后赋给 `hi`

#### Clobbered寄存器

在很多情况下，无需作为输出的状态都会被内联汇编修改，这个状态被称之为 "clobbered"

需要告诉编译器相关的情况，因为编译器需要在内联汇编语句块的附近存储和恢复这种状态

```rust
use std::arch::asm;

fn main() {
    // three entries of four bytes each
    let mut name_buf = [0_u8; 12];
    // String is stored as ascii in ebx, edx, ecx in order
    // Because ebx is reserved, the asm needs to preserve the value of it.
    // So we push and pop it around the main asm.
    // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)

    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // We use a pointer to an array for storing the values to simplify
            // the Rust code at the cost of a couple more asm instructions
            // This is more explicit with how the asm works however, as opposed
            // to explicit register outputs such as `out("ecx") val`
            // The *pointer itself* is only an input even though it's written behind
            in("rdi") name_buf.as_mut_ptr(),
            // select cpuid 0, also specify eax as clobbered
            inout("eax") 0 => _,
            // cpuid clobbers these registers too
            out("ecx") _,
            out("edx") _,
        );
    }

    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name);
}
```

- 使用 `cpuid`指令来读取 CPU ID，该指令会将值写入到 `eax` 、`edx`和 `ecx`中
- 即使 `eax`从没有被读取，依然需要告知编译器这个寄存器被修改过，这样编译器就可以在汇编代码之前存储寄存器中的值
  - 需要通过将输出声明为 `_` 而不是一个具体的变量名，代表着该输出值被丢弃
- 这段代码也会绕过一个限制： `ebx`是一个LLVM保留寄存器，意味着LLVM会假设它拥有寄存器的全部控制权，并在汇编代码块结束时将寄存器的状态恢复到最开始的状态
  - 由于这个限制，该寄存器无法被用于输入或者输出，除非编译器使用该寄存器的满足一个通用寄存器的需求(例如 `in(reg)` )
  - 但这样使用后， `reg`操作数就在使用保留寄存器时变得危险起来，原因是我们可能会无意识的破坏输入或者输出，毕竟它们共享同一个寄存器
- 为了解决这个问题，使用 `rdi`来存储指向输出数组的指针，通过 `push`的方式存储 `ebx`：在汇编代码块的内部读取 `ebx`的值，然后写入到输出数组。后面再可以通过 `pop`的方式来回复 `ebx`到初始的状态
- `push`和 `pop`使用完成的64位 `rbx`寄存器，来确保整个寄存器的内容都被保存
  - 如果是在32位机器上，代码将使用 `ebx`替代

```rust
use std::arch::asm;

// Multiply x by 6 using shifts and adds
let mut x: u64 = 4;
unsafe {
    asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
}
assert_eq!(x, 4 * 6);
```

[Rust Reference](https://doc.rust-lang.org/reference/inline-assembly.html) 和 [Rust By Example](https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#clobbered-registers)
