# FerrumRV 课程进度

- 更新日期：2026-09-09
- Teaching Mode：ON
- 当前 Phase：A
- 当前 Chapter：0
- 当前 Milestone：0.1 — 建立空项目
- 已通过 milestone：0.1
- 当前状态：Milestone 0.1 PASS；待学生完成本里程碑 Git 提交。
- 当前待验收项：无；零测试证据边界已在教师讲解后由学生确认，后续实际编写测试时再巩固。
- 下一目标：学生提交 0.1；0.2 已解锁，尚未开始。

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

## 工程纪律

建议本 milestone 验收通过后由学生形成小步提交。Chapter 0 结束时必须更新记录并形成 git commit。

## 基础摸底

- Rust：完全没有基础。
- ownership：完全不了解。
- RISC-V：读过 ISA 手册，对 I 指令集部分较了解。
- 机器码：较熟悉。
- ELF 与工具链：用 readelf 观察过 ELF，使用过 objdump；另提及“copy”，具体工具尚未确认。
- 教学调整：Rust 与所有权从零讲起；机器码与工具链讲解结合已有经验，但仍逐项验收，不跳过 milestone。当前只推进 0.1。
