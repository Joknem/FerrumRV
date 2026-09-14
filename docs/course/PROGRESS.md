# FerrumRV 课程进度

- 更新日期：2026-09-14
- Teaching Mode：ON
- 当前 Phase：A
- 当前 Chapter：0
- 当前 Milestone：0.2 — Rust 所有权热身
- 已通过 milestone：0.1、0.2
- 当前状态：0.2 PASS；所有权、借用、slice 与函数参数练习已验证，等待学生提交本 milestone。
- 当前待验收项：无；提交后解锁 Chapter 1 / Milestone 1.1。
- 下一目标：提交 `ch00: practice Rust ownership`，然后开始设计 CPU state；Chapter 1 当前未开始。

## 教学约定

遵循仓库根目录 FerrumRV_COURSE.md。学生亲手完成项目创建与核心实现；教师提供逐级提示、检查和课程记录。默认从 Hint 0 或 Hint 1 开始，每次只推进一个 milestone。

## Milestone 0.1 验收记录

- [x] Works：学生创建最小可执行项目，cargo build 成功，程序可运行（学生截图；教师已读取 Cargo.toml 与 src/main.rs）。
- [x] Tested：cargo test 成功；已记录正常构建、无改动重复构建和一个可恢复的失败案例。学生在讲解后确认零用例未验证 Hello world 是否正确输出。
- [x] Explained：学生已解释 Cargo.toml、src/main.rs 的角色、build 生成可执行程序、一个 package 包含多个 crate，以及 debug/release 改变构建配置。
- [x] Debuggable：学生根据 could not compile 判断编译阶段失败，指出 cannot find macro 与 src/main.rs:2:5 定位信息，并根据教师与编译器提示修正宏名称；恢复已验证。
- [x] Integrated：教师读取源码确认已恢复最小 Hello world 程序，并重新 build/test 成功；当前无历史测试，测试用例数为 0。

### 已确认的理解与证据

- 学生截图显示 cargo、rustc 均可调用，版本均为 1.98.1。
- 学生正确区分 Cargo.toml 描述项目、src/main.rs 描述程序行为。
- 学生预测 cargo build 只构建、不执行当前程序，并用 build/run 结果验证。
- 学生正确识别 Hello, world! 为程序输出，其余 Compiling、Finished、Running 为 Cargo 提示。
- 截图中 build 出现 Compiling；随后 run 未出现 Compiling，并运行 target/debug/ferrumrv。教师已结合后续重复构建截图讲解产物复用。
- 学生最初把零用例理解为仅编译或没有测试程序；教师区分测试程序与测试用例后，学生确认没有用例就没有验证 Hello world 是否正确输出。此概念后续用实际测试继续巩固。
- 失败实验：pritln! 拼写错误导致编译失败；学生依据 could not compile 正确判断失败阶段。学生已改回 println!，教师复验 build/test 均退出成功。
- 学生正确回答库与命令行程序同属一个 package、包含多个 crate；教师补充该例为两个 crate。
- 学生截图显示 release 构建为 optimized，随后默认构建为 unoptimized + debuginfo；学生正确解释为构建配置变化。
- 无改动重复默认构建截图仅显示 Finished，无 Compiling；教师已指出复用了已有产物。

## Milestone 0.2 学习记录

- 起点：教师检查提交、工作区与当前 main.rs；开始前工作区干净，源码仍为最小 Hello world。最近 build/test 成功，测试用例数为 0。
- 当前练习：学习函数参数声明和调用，观察 String 按值传参时的 move，再改为借用对照。
- [x] 变量绑定与 mut 热身：学生提供 E0384 诊断并亲手添加 mut；教师读取源码、执行 cargo run，输出 apples = 4。仍有初值 3 在读取前被覆盖的 unused_assignments 警告，待结合数据流解释。
- [x] 亲手经历 move 后原变量不能继续使用：学生提供 E0382 截图，正确解释 received 拥有字符串，并亲手移除对 message 的后续使用；教师读取源码、运行 cargo run，成功输出 hello，无警告。
- 整数 Copy 对照：学生正确预测 saved 保留 3，并解释复制整数值而非建立指针关系；之前曾在源码中保留练习注释，当前已替换为借用练习。
- [x] immutable borrow：学生解释借用不转移所有权，预测不能通过 &String 修改字符串，并用 push_str 触发 E0596 验证。
- [x] mutable borrow：学生通过 &mut 修改原字符串；亲手触发 E0502，理解后续使用 reader 会使借用持续并与读取 message 冲突；调整顺序后教师检查源码并运行 cargo run，输出两行 hello!，无警告。cargo test 成功，仍为 0 个测试用例。
- [x] slice：正常范围 1..3 输出 [20, 30]、长度 2；学生截图验证 4..4 为空、长度 0；1..5 编译成功后在 src/main.rs:3:36 panic。学生正确区分运行时与编译时失败，并解释切片借用原数组元素而非复制。学生恢复 1..3 后，教师读取源码、执行 cargo run，再次确认输出 [20, 30] 和 2。
- [x] 函数参数传值与传引用：学生先观察 `show_message(text: String)` 导致 E0382，再改为 `&String` 并以 `show_message(&message)` 调用；教师读取源码、运行 cargo run，函数内和调用后均输出 joknem，确认所有权仍由 message 持有。
- [x] 五级回归：`cargo fmt --check`、`cargo build`、`cargo test`、`cargo run` 均成功；运行输出 `joknem` 与修改后的 `joknemasd`。测试仍为 0 个用例，不能证明具体行为覆盖。
- 验收：各练习能运行、验证正常及边界/失败行为、解释值与借用关系、定位诊断；最终恢复可构建和可测试状态并检查回归。
- 约束：只做与 CPU 无关的小练习；不以 clone 或 unsafe 绕过所有权学习，不替学生写实现。

## 工程纪律

建议本 milestone 验收通过后由学生形成小步提交。Chapter 0 结束时必须更新记录并形成 git commit。

## 基础摸底

- Rust：完全没有基础。
- ownership：完全不了解。
- RISC-V：读过 ISA 手册，对 I 指令集部分较了解。
- 机器码：较熟悉。
- ELF 与工具链：用 readelf 观察过 ELF，使用过 objdump；另提及“copy”，具体工具尚未确认。
- 教学调整：Rust 与所有权从零讲起；机器码与工具链讲解结合已有经验，但仍逐项验收，不跳过 milestone。当前只推进 0.2。
