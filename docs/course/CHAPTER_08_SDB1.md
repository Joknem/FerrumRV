# Chapter 8.1：SDB-1——把模拟器变成可观察的机器

## 核心问题

CPU 执行错误时，怎样观察它实际经历的状态转移，而不是只看最后一个错误结果？

NEMU 将 monitor 作为模拟器的重要组成部分，提供单步、寄存器查看和内存查看等调试能力。[NEMU README](https://github.com/NJU-ProjectN/nemu) FerrumRV 这一章实现一个小得多的版本：先建立观察和控制 CPU 的边界，再逐步加入命令行交互。

## 本章完成后应理解

- monitor 是 CPU 模拟器外部的控制层，不是 CPU 执行逻辑的一部分；
- `step` 是最小执行控制原语，`continue` 只是重复调用它直到停止或出错；
- 调试器要区分控制操作和状态观察，不能让命令行任意修改 CPU 内部字段；
- 命令行输入库只负责读取可编辑的一行文本，命令解析和 CPU 操作仍由项目自己实现；
- Rust 中 `String`、`&str`、迭代器、`Result` 和模块可见性如何组合成一个小型 REPL。

## 项目结构

Chapter 8.1 采用足够小但职责清晰的结构：

```text
FerrumRV/
├── Cargo.toml              # 项目依赖
├── src/
│   ├── main.rs             # 组装 CPU、Memory、Monitor 并启动程序
│   ├── cpu.rs              # CPU 状态、取指、解码、执行、step
│   ├── memory.rs           # guest memory 访问
│   ├── program.rs          # 程序镜像描述和加载
│   └── monitor.rs          # readline、命令解析、命令分派、调试输出
├── programs/               # 固定入口的独立 guest 自检镜像
│   ├── README.md            # 文件格式、退出协议和程序目录
│   ├── 00_add_sub.hex       # 每行地址 + 机器码 + 汇编说明
│   └── ...                 # 共 15 个 .hex 测试文件
└── docs/course/
    ├── CHAPTER_TEMPLATE.md
    └── CHAPTER_08_SDB1.md
```

本章不要把 monitor 逻辑长期堆进 `main.rs`。`main.rs` 只负责创建对象并调用 monitor 的入口。当前不提前拆成 `monitor/command.rs`、`monitor/output.rs` 等多个文件；等命令解析和输出明显变复杂时，再迁移为子目录。

建议的依赖关系：

```text
main.rs
  ├── program.rs
  └── monitor.rs
        ├── cpu.rs
        └── memory.rs
```

## Guest 程序与加载

使用 [programs/README.md](../../programs/README.md) 中的 15 个独立镜像。每个程序直接对应 `programs/<name>.hex`，不再有程序子目录、独立 .s 文件或 entry 行。入口固定为 `0x80000000`，按每行声明的地址加载；RAM 使用 4096 字节，数据区从 `0x80000800` 起。

```text
0x80000000: 0x00a00093    # addi x1, x0, 10
```

`program.rs` 负责运行时读取文本、去掉注释、按冒号拆出地址和机器码、分别解析成 u32，再按声明的地址写入 Memory。当前先实现最小加载器，无需自定义 `ProgramError`，也不把整程序预检作为本章要求；加载出错时 main 报错并退出，不进入 monitor。main 创建 Memory、调用加载函数、创建固定入口的 CPU，再启动 monitor。模块声明要包含 `mod program;`。

例如使用 `cargo run -- 09_sum_loop`，运行时读取 `programs/09_sum_loop.hex`，从仓库根目录启动。新增文件无需修改 Rust 中的名字映射。加载器仍由学生实现。

guest 自检以 a0/x10 为状态码，EBREAK 停机后由 monitor 解释：0 表示 PASS，非零表示 FAIL。错误镜像触发 StepError，由 monitor 显示故障，不要把故障说成正常通过。CPU 的 EBREAK 语义保持不变。详细约定、数据地址和程序用途见程序目录 README。

## Rust 与工具知识

### `rustyline` 与标准输入

标准库的 `std::io::stdin().read_line(...)` 只能提供基础的按行读取。`rustyline` 提供 readline 风格的行编辑、历史记录和 Ctrl-C/Ctrl-D 处理；它不解析 FerrumRV 命令，也不是现成的 debugger framework。可以把它看成终端输入适配器。

在 `Cargo.toml` 中加入依赖：

```toml
[dependencies]
rustyline = "18"
```

你仍然需要自己完成：

```text
一行文本 → 命令名和参数 → 参数解析 → 调用 CPU/Memory → 输出结果
```

### `String`、`&str` 和命令参数

readline 返回一个拥有内容的 `String`。命令解析通常只需要借用它的片段：

```text
String
  └── split_whitespace()
        └── 若干个 &str
```

输入 `x 0x80000000 4` 后，参数应被分成命令名 `x`、地址文本和长度文本。不要把字符串解析和内存读取混在一个大分支里；先验证参数，再执行操作，错误信息会更清楚。

### `Result` 与错误边界

下面几类错误应留在 monitor 层处理：

- 未知命令；
- 参数数量错误；
- 数字格式错误；
- 不支持的查看长度；
- Memory 返回的越界错误；
- CPU `step` 返回的 Fetch/Decode/Execute 错误。

monitor 不应使用 `unwrap()` 把用户输入错误变成 host panic。命令处理函数可以返回一个自己的 monitor 错误，或者在 REPL 层打印错误后继续等待下一条命令。

### 借用和状态观察

执行一步需要对 CPU 和 Memory 取得可变借用；打印状态通常只需要共享借用。把一次可变操作放在较小的作用域中，可以让后面的观察继续借用对象。不要通过 `clone` CPU 来规避借用，也不要把内部字段全部改成 `pub`。

## 任务范围

实现一个最小 monitor，支持以下命令：

```text
si
continue
info registers
x 地址 长度
quit
```

命令契约：

### `si`

执行一条指令。成功时打印至少：

```text
pc
raw instruction
执行结果
```

如果 CPU 已经 halted，不能再次取指或改变状态。

### `continue`

重复执行 `step`，直到：

- CPU 进入 halted；
- Fetch、Decode 或 Execute 错误；
- 用户中断（如果你选择让 readline 的 Ctrl-C 终止当前 continue）。

每次循环都应使用新的 CPU 状态，不能在循环外缓存寄存器或 PC。

### `info registers`

打印 32 个寄存器和 PC。输出格式可以自行设计，但必须能区分寄存器编号和值，并显示当前 PC。

### `x 地址 长度`

从给定 guest 地址开始读取指定数量的字节。第一版只要求支持一个明确的长度范围，例如 `1..=16`；超出范围或越界时打印错误并继续进入 monitor。

地址参数至少支持十六进制形式，例如：

```text
x 0x80000000 4
```

### `quit`

退出 monitor，返回到 host 程序。

## 实现顺序

这是一个完整模块任务，不按单个命令逐轮验收。建议按以下顺序完成：

1. 建立 `monitor.rs` 和模块入口；
2. 加入 `rustyline`，让程序能够读取一行命令；
3. 设计命令表示和参数解析错误；
4. 公开 CPU 需要的最小观察/控制接口；
5. 实现 `quit`、`si`、`info registers`；
6. 实现 `continue` 和 `x`；
7. 用一个会执行 EBREAK 的小程序手动运行 monitor；
8. 再写少量自测，验证解析和错误输入。

## 小提示

- `continue` 不应该复制 `step` 的取指、解码和执行逻辑；它只负责循环和停止条件。
- `info registers` 需要的是只读观察能力；不要为了打印而暴露可写的 `regs` 字段。
- `x` 的地址是 guest 地址，不是 `Vec` 下标；应继续通过 Memory 的访问接口检查边界。
- `0x` 前缀的解析与十进制解析是同一个“用户输入格式”问题，先统一成一个小函数。
- 如果命令错误后程序直接退出，你还没有实现 monitor 的交互错误边界。

## 需要回答的问题

1. 为什么 `continue` 应调用 `step`，而不应该复制一份执行循环？
2. 为什么 monitor 需要读取 PC 和寄存器，却不应该直接公开整个 `Cpu` 的字段？
3. `x 0x80000000 4` 中的地址为什么不能直接当作 `Vec` 的下标？
4. 用户输入 `x hello 4` 时，错误应该在哪一层产生？为什么不应该让它触发 panic？
5. 如果 `step` 返回 `StepError::Execute`，monitor 应该继续执行下一条命令，还是退出？你的选择是什么？
6. `rustyline` 负责了哪些工作，哪些工作仍然属于 FerrumRV monitor？

## 学生自测建议

先写少量帮助调试的测试即可：

- 空行和未知命令不会 panic；
- 十六进制地址能被解析；
- 参数数量错误能被报告；
- `quit` 能结束循环；
- `si` 只调用一次 CPU step。

不需要提前写完整的教师验收测试。你认为模块完成后，告诉 Codex；教师会再加入独立验收测试。

## 完成条件

- `monitor.rs` 与 `main.rs` 职责分离；
- 五个命令可用；
- 非法命令和参数错误不会使程序 panic；
- `continue` 能处理 EBREAK 和 StepError；
- `x` 通过 Memory 接口查看 guest memory；
- 你能回答本章问题；
- 教师验收测试和完整回归通过。
