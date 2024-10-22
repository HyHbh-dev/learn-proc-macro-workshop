# Rust Latam：程序宏研讨会

*该 repo 包含一系列旨在学习编写 Rust 代码的精选项目。
程序宏&mdash; 生成 Rust 代码的 Rust 代码。
*这些项目中的每一个都取材于令人信服的实际用例。从
这里的 5 个项目中，有 3 个是我亲自在 Rust 中实现的宏。
工业代码库中实现的宏，另外两个宏作为库存在于 crates.io
上的库。

<br>

## 内容

- [**Suggested prerequisites**](#suggested-prerequisites)
- [**Projects**](#projects) — 每个项目的介绍
  - [**派生宏:** `derive(Builder)`](#derive-macro-derivebuilder)
  - [**派生宏:** `derive(CustomDebug)`](#derive-macro-derivecustomdebug)
  - [**函数宏:** `seq!`](#function-like-macro-seq)
  - [**属性宏:** `#[sorted]`](#attribute-macro-sorted)
  - [**属性宏:** `#[bitfield]`](#attribute-macro-bitfield)
  - [**Project recommendations**](#project-recommendations) —  工作内容
取决于你的兴趣
- [**Test harness**](#test-harness) — 测试设置说明
- [**Workflow**](#workflow) —  通过 研讨会开展工作的建议方式
- [**Debugging tips**](#debugging-tips)

<br>

## 建议的先决条件

本讲座包括属性宏、派生宏和类函数宏。
过程宏。
请注意，工作坊的内容和本软件仓库中的解释将
假定对结构体、枚举、特质、特质/impls、泛型
参数和特质边界。欢迎您以任何
但您可能会发现，这些基础知识要容易得多。
但您可能会发现，这些基础知识在宏的环境之外更容易学习。

<br>

## project

下面是每个项目的介绍。在底部，我给出了
根据你的兴趣，建议你按照什么顺序来完成它们。请注意
每个项目的深度都比
介绍的深度。

### 派生宏：派生宏： `derive(Builder)`

此宏生成实现 [builder] 所需的模板代码。
[模式] 的模板代码。构造器是一种实例化结构体的机制，尤其是
结构体的实例化机制，尤其是当这些结构体中有许多字段是可选的或
随着时间的推移，字段集可能需要向后兼容增长。

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

在 Rust 中表达建造者有几种不同的可能性。除非
你有强烈的预设偏好，为了保持简洁
项目中，为了保持简洁，我建议仿效标准库中的
[std::process::Command`]构建器的例子，其中的设置器方法分别接收和
返回 `&mut self` 以允许链式方法调用。

[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

调用者将以如下方式调用宏。

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
}
```

该项目包括

- 遍历语法树
- 构建输出源代码；
- 处理辅助属性以定制生成的代码。

_Project skeleton is located under the <kbd>builder</kbd> directory._

### 派生宏：派生宏： `derive(CustomDebug)`

该宏实现了标准库 [`std::fmt::Debug`]的派生。
特质的派生宏，比由
更可定制。

[`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html

特别是，我们希望能够选择用于单个
字段所使用的格式。
格式化宏（如 `format!` 和 `println!`）所期望的风格提供格式化字符串，从而选择用于单个结构体字段的格式化。

```rust
use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}
```

在这里，上述结构体的一个可能实例可以通过其
生成的 "调试 "隐式结构体打印出来：

```console
Field { name: "st0", bitmask: 0b00011100 }
```

该项目包括

- 遍历语法树
- 构建输出源代码；
- 处理辅助属性
- 处理生命周期参数和类型参数；
- 推断性状 impls 通用参数的性状界限；
- 派生指令在发出普遍正确的特质界限方面的局限性。

项目骨架位于 <kbd>debug</kbd> 目录下。

### 类函数宏：`seq!`

该宏提供了一种语法，用于按顺序标出一个
的顺序索引副本的语法。
例如，我们的应用程序可能需要一个按顺序编号的
变量，如 ` Cpu0``Cpu1``Cpu2 `...Cpu511`。但请注意，同样的 `seq!
宏应适用于任何类型的编译时循环；没有任何特定的
枚举变体。不同的调用者可能会使用它来生成一个
表达式，如 `tuple.0 + tuple.1 + ...+ tuple.511`。

```rust
use seq::seq;

seq!(N in 0..512 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Processor {
        #(
            Cpu~N,
        )*
    }
});

fn main() {
    let cpu = Processor::Cpu8;

    assert_eq!(cpu as u8, 8);
    assert_eq!(cpu, Processor::Cpu8);
}
```

该项目包括

- 解析自定义语法；
- 标记流的底层表示；
- 构建输出源代码。
  项目骨架位于 <kbd>seq</kbd> 目录下。

### 属性宏：`#[排序]`属性宏

当你的同事（或你自己）似乎无法将枚举
变体排序时使用的宏。该宏将
会在编译时检测到未排序的变体，并发出错误提示，指出哪个
不按顺序排列的变体。

```rust
#[sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}
```

该项目包括

- 编译时错误报告；
- 应用访问者模式遍历语法树；
- 目前稳定的宏 API 的局限性和一些解决方法
  的局限性和一些解决方法。
  项目骨架位于 <kbd>sorted</kbd> 目录下。

### 属性宏：`#[bitfield]`属性宏

该宏为在打包的二进制结构中定义结构体提供了一种机制
访问比特范围的机制，类似于语言级的
位字段]的语言级支持。
[C 语言中的位字段]：https://en.cppreference.com/w/cpp/language/bit_field
宏将把其中一个结构体概念化为 0...N 的位序列。
这些比特按照由以下结构写入的结构所指定的顺序分组到字段中
调用者编写的结构体所指定的顺序分组。#[位字段]"属性将调用者的结构重写为一个
私有字节数组表示，每个字节数组都有公共 getter 和 setter 方法。
字段的公共 getter 和 setter 方法。
总位数 N 必须是 8 的倍数（这将是
将在编译时检查）。
例如，下面的调用会生成一个总大小为 32 的结构体
位或 4 字节。它将字段 `a` 置于第一个
字段 `a` 放在第一个字节的最小有效位，字段 `b` 放在接下来的三个最小有效位，字段 `c` 放在最后一个字节的最小有效位。
字段 `c` 放在第一个字节的其余四个最有效位上，字段 `d` 跨过第一个字节。
跨接下来的三个字节。

```rust
use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

```text
                               least significant bit of third byte
                                 ┊           most significant
                                 ┊             ┊
                                 ┊             ┊
║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
╟───────────────╫───────────────╫───────────────╫───────────────╢
║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
╟─╫─────╫───────╫───────────────────────────────────────────────╢
║a║  b  ║   c   ║                       d                       ║
                 ┊                                             ┊
                 ┊                                             ┊
               least significant bit of d         most significant
```

该结构的 `#[位字段]`宏发出的代码如下。
请注意，字段获取器和设置器使用的是 `u8`、`u16`、`u32` 中的任意一个、
u64`中最小的一个，同时至少与
位数。

```rust
impl MyFourBytes {
    // Initializes all fields to 0.
    pub fn new() -> Self;

    // Field getters and setters:
    pub fn get_a(&self) -> u8;
    pub fn set_a(&mut self, val: u8);
    pub fn get_b(&self) -> u8;
    pub fn set_b(&mut self, val: u8);
    pub fn get_c(&self) -> u8;
    pub fn set_c(&mut self, val: u8);
    pub fn get_d(&self) -> u32;
    pub fn set_d(&mut self, val: u32);
}
```

该项目包括

- 遍历语法树
- 处理辅助属性；
- 构建输出源代码；
- 与标准库以外的特质和结构体交互；
- 需要类型信息的编译时断言技术，通过
  从生成的代码中以有趣的方式利用特质系统；
- 棘手的代码。
  项目骨架位于 <kbd>bitfield</kbd> 目录下。

### 项目建议

如果您是第一次使用程序宏，我建议您
从 `derive(Builder)` 项目开始。这将让你熟悉
遍历语法树和构建输出源代码。这两个
过程宏的两个基本组成部分。
在这之后，同样可以跳转到任何一个
derive(CustomDebug)`、`seq!`或`#[sorted]`。

- 如果您有兴趣探索宏如何使用，请选择 `derive(CustomDebug)` 。
  操作特质边界，而这正是
  代码生成最复杂的方面之一。本项目
  对特质边界进行了平易近人的介绍，并深入探讨了许多
  具有挑战性的方面。
- 如果你对自己解析自定义输入语法感兴趣，可以选择 `seq!`。
  其他项目都将主要依赖于已经完成的解析器。
  编写并作为库发布的解析器，因为它们的输入是普通的 Rust
  语法。
- 如果您对生成诊断信息（自定义的
  错误）。本项目的一部分还涵盖了一种不同的
  处理输入语法树的不同方法；其他项目将通过
  if let`.访问者方法更适合某些类型的宏
涉及语句或表达式的宏。
match "臂已排序。
[Serde]: https://serde.rs/
我建议，只有在您觉得自己已经掌握了 `#[位域]`之后，才开始使用 `#[位域]`。
  其他项目中的至少两个项目。请注意，完成
  完整的预期设计将涉及编写所有三种类型中的至少一种
  程序宏，代码量也比其他项目多得多。

<br>

## 测试线束

彻底测试宏往往很棘手。Rust 和 Cargo 内置了
测试框架，可用于测试成功案例、
但我们也非常关心我们的宏在发生错误时是否能产生良好的错误信息。
在编译时检测到问题；Cargo 并不能说在编译时未能
编译失败被认为是成功的，也不能比较出错的
编译器生成的错误信息是否与我们所期望的一致。
该资源库中的项目骨架使用另一种测试工具，名为
[trybuild].

[trybuild]: https://github.com/dtolnay/trybuild

<p align="center">
<a href="#test-harness">
<img src="https://user-images.githubusercontent.com/1940490/55197640-eb390080-5191-11e9-8c1f-1183935c0c26.png" width="600">
</a>
</p>

测试线束的目的是迭代一个
程序宏的执行，观察程序宏执行失败后产生的错误。
并测试这些错误是否符合预期。

<br>

## 工作流程

每个项目的 <kbd>tests</kbd> 下都已编写了测试套件
目录下。(但您也可以随意添加更多测试，删除功能测试。
或修改测试，使其与您的项目相一致。
实现）。
在 5 个顶级项目目录中的任意目录下运行 `cargo test` 以运行
测试套件。
最初，每个项目都会禁用所有测试。打开
项目的 _tests/progress.rs_ 文件，并在工作过程中逐个启用测试。
逐个启用测试。\*\*测试文件（例如 _tests/01-parse.rs_ 文件）
测试文件（例如 _tests/01-parse.rs_）都包含一个注释，解释要测试的功能，并给出一些
我建议按编号来完成测试。
我建议按编号顺序完成测试，每次多启用一个测试并确保其通过后再继续。
测试分为两种：一种是应成功编译并运行的测试，另一种是应成功编译并运行的测试。
应编译失败并带有特定的错误信息。
如果测试应成功编译并运行，但却失败了，测试运行程序将
会显示编译器错误或运行时错误输出。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197637-eb390080-5191-11e9-9197-5832071639ea.png" width="600">
</a>
</p>

对于编译失败的测试，我们将编译输出与
该测试的预期错误文件进行比较。如果这些错误匹配，则测试
视为通过。如果不匹配，测试运行程序将显示
预期输出和实际输出。
预期输出将放入与测试同名的文件中，但会在文件中加上
扩展名为 _\*.stderr_，而不是 _\*.rs_。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197639-eb390080-5191-11e9-9c8f-a47cab89652d.png" width="600">
</a>
</p>

如果本应编译失败的测试没有 _\*.stderr_ 文件、
测试运行器会将编译器的输出保存到一个名为
<kbd>tests</kbd>目录相邻的 <kbd>wip</kbd> 目录中。因此，更新
预期 "输出的方法是删除现有的 _\*.stderr_ 文件，然后运行测试
再次运行测试，使输出写入 _wip_，然后将新输出从
移到 _tests_。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197642-ebd19700-5191-11e9-8f00-2d7c5f4be1a9.png" width="600">
</a>
</p>

<br>

## 调试技巧

要查看宏扩展到哪些代码，请安装 [cargo expand] 货物
子命令，然后在版本库根目录下运行 `cargo expand` （在任何
项目目录之外），展开该目录中的 main.rs 文件。您可以
将任何测试用例复制到该 main.rs 文件中，并在迭代时对其进行调整。
宏。

[cargo expand]: https://github.com/dtolnay/cargo-expand

如果宏正在生成语法上无效的代码（不仅仅是无法识别的代码
类型检查失败的代码），那么 cargo expand 将无法显示它。取而代之的是
宏生成的令牌流（TokenStream）打印到 stderr，然后再返回令牌。

```rust
eprintln!("TOKENS: {}", tokens);
```

然后在版本库根目录下进行 "货物检查"（如果使用 main.rs 进行迭代的话）
或相应项目目录中的 `cargo test` 会显示以下输出结果
会在宏扩展时显示此输出。
Stderr 也是查看语法树结构的一个有用方法。
的语法树结构。

```rust
eprintln!("INPUT: {:#?}", syntax_tree);
```

请注意，为了让 Syn 的语法树类型提供调试 impls，您需要
需要在 Syn 的依赖关系中设置 `features = ["extra-traits"]` 。这是
因为添加数百个调试 impls 会增加大量编译
的编译时间，而且我们只需要在一个
而不是在向用户发布完成的宏时。
<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
