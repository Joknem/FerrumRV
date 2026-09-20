# FerrumRV 课程进度

- 更新日期：2026-09-20
- Teaching Mode：ON
- 当前 Phase：A
- 当前 Chapter：3
- 当前 Milestone：3.3 — 符号扩展
- 已通过 milestone：0.1、0.2、1.1、1.2、2.1、2.2、3.1、3.2、3.3
- 当前状态：Chapter 3 验收通过；七个测试与格式检查通过，无测试编译警告；课程记录已更新。
- 当前待验收项：本章无；待学生完成 Chapter 3 收尾提交。
- 下一目标：Milestone 4.1 ADDI execute 已解锁，尚未开始实现。

## Milestone 3.3 学习记录

- PASS：学生独立解释 u32 右移补零、i32 右移补符号位；教师补充移位后再转换不会恢复十二位符号扩展。负数用例、两端边界、错误路径与既有功能回归通过，能根据断言区分机器码错误与符号扩展错误，五级验收完成。

- 学生先修正负数测试机器码的 opcode/rs1/funct3 错误，再用 0xffd30293 复现实际 imm=4093、预期 imm=-3 的失败。
- 学生确认应先转为 i32 再右移，亲手改为 raw as i32 >> 20；负数 -3 与已有正数用例回归通过。
- 边界：0xfff30293 解码 imm=-1，0x80030293 解码 imm=-2048；rs1=6、rd=5 均正确，已有 0、2047 用例保留。
- 最新验证：cargo test 七个测试通过，cargo fmt --check 通过，无测试编译警告；章末解释已通过。

## Milestone 3.2 学习记录

- PASS：正常解码、寄存器 0/31 与立即数 0/2047 边界、独立错误 opcode/funct3 均通过；学生手工编码与提取字段，修正类型及返回值问题，旧测试无回归。
- 可读输出：cargo test -- --nocapture 显示 Ok(Addi { rs1: 0, imm: 2047, rd: 31 })。共享状态不参与解码，无借用规避或 unsafe。

- 学生手工编码 addi x5,x6,3 得到 0x00330293，并写出 opcode、rd、funct3、rs1、imm 的移位与掩码表达式。
- 学生选择 Result 与自定义 DecodeError::UnsupportedInst { raw: u32 }；地址由后续调用者补充，不作为解码输入。
- 当前接口为 impl Cpu 外的私有普通函数 decode(raw: u32) -> Result<Inst, DecodeError>，不依赖 CPU 状态。Inst::Addi 字段现命名为 rs1、imm、rd。
- 最新检查：cargo test 七个测试通过，cargo fmt --check 通过，无测试编译警告。inst_test 已实际调用 decode：0x00330293 返回预期 Addi；0x00330280（仅 opcode 改为零）、0x00331293（仅 funct3 改为 001）返回保留原始机器码的 UnsupportedInst；0x000f8013、0x7ff00f93 覆盖字段边界。
- 学生已修正寄存器字段 u32/u8 类型差异、match 返回值及穷尽分支；imm 暂用 (raw >> 20) as i32，只正确解释非负立即数，符号扩展尚未实现。

## Milestone 3.1 学习记录

- 验收收尾：学生正确区分 u8 可表示 32 与寄存器编号 32 不合法；结合构造拆解练习、源/目标纠错及七个测试回归，3.1 通过。

- 学生选择 Inst 枚举，Addi 携带 sreg: u8、imm: i32、dreg: u8；寄存器保存编号，执行时才读取其值。
- 已讲解 enum 构造、match 拆解及穷尽匹配；学生纠正 ADDI 源/目标寄存器填反的问题。
- inst_test 构造 addi x5,x6,-3 并拆解断言三个字段；本测试是语法练习，不证明机器码解码正确。
- 应学生请求补充 for、while、loop 及元组；学生在 main 中用 for 累加 [3,7,9]，教师运行确认输出 19。
- 最新验证：cargo test 七个测试通过，无测试编译警告；cargo fmt --check 通过。没有添加核心实现或替学生编写测试。

## Milestone 2.2 验收记录

- PASS / Works：fetch 使用当前 PC 调用 Memory::read_word，返回原始 u32 或 AddrError。
- Tested：两个不同 PC 的已知模式、最后合法起点、越界请求原始地址及长度均验证通过。
- Explained：学生解释低地址保存低有效字节；教师补充字节序列由 read_word 按小端规则组合成 Rust u32，章末解释题通过。
- Debuggable：通过准备数据的 Result 断言、取指期望值与错误字段断言区分准备失败、取值错误和错误传播问题；本章失败写入不回滚的问题已由学生解释并记录于 Bug 日记。
- Integrated：cargo test 六个测试通过且无警告，cargo fmt --check、cargo build 通过。普通构建有四组 dead_code 警告，因为 main 尚未使用 CPU/Memory，不阻塞本步验收。
- Rust 质量：共享借用合理，无 clone、unsafe 或 unwrap 绕过；不要求重复 Memory 单元测试。取指对齐仍是调用者前提，未实现运行时对齐检查。

## Milestone 2.2 学习记录

- 当前组织：main.rs 声明 cpu、memory；实现位于 cpu.rs、memory.rs，测试在 cpu/tests.rs、memory/tests.rs，由 #[cfg(test)] mod tests 加载。
- 学生提出文件拆分需求后，已讲解 mod 建立模块树、crate/super 路径、use 引入名字、pub(crate)/pub 可见性与 derive 能力。代码与测试均由学生移动。
- 最新回归：六个测试通过，cargo fmt --check 通过。取指测试覆盖两个正常 PC、最后合法起点 0x8000000c、越界 PC=0x80000010 的原始地址与长度 4；准备数据的写入 Result 均已检查。
- 学生指出重复验证 Memory 的冗余，调整验收：不在 CPU 测试重做内存读写测试或逐项内存快照。当前 fetch 使用 &self、&Memory，相关字段无内部可变性，状态不变结合类型约束与代码审查确认；已有 PC 和寄存器断言保留即可。

- 起点：教师读取当前代码和教案；最近 5 个测试、格式检查通过，本次开章未重复运行。
- 本步只取原始 32 位值，不解码、不执行、不更新 PC；继续沿用第一版取指 PC 需 4 字节对齐的契约。
- 验收范围：已知内存模式与取指结果一致；选择不同 PC 验证使用当前地址；边界和错误传播明确；状态不变由当前共享借用接口及实现确认。

## Milestone 2.1 验收记录

- PASS：学生亲手实现 8/16/32 位小端读写、Result 错误返回和自定义 AddrError。
- 表示：Memory 保存 base_addr: u32 与 bytes: Vec<u8>；当前固定起点 0x80000000、容量 16 字节、初始化全零。
- 错误契约：InvalidRange 保存原始请求地址与访问字节数；多字节接口用 match 转换子调用错误，不能直接传播内部一字节请求。
- 写入契约：地址加法溢出和整个范围上界在写入前检查，下界由首个字节写入检查且在修改前返回；失败不得留下部分修改。
- Works：各宽度读写由测试实际调用；CPU 既有功能保留。
- Tested：字节首末地址及两侧越界；16 位小端组合、最后完整起点、跨界和最大地址；32 位独立字节布局、最后完整起点、跨界及最大地址。非法写入后分别比较整块内存与独立期望值。
- Explained：学生解释 guest 地址到偏移、字节寻址、小端序，以及 ? 只提前返回而不回滚已写入状态。
- Debuggable：学生经历边界判断差一、误把单字节按四字节检查、复制导致重复读取同一地址、忽略 Result 导致部分写入等问题，并修正。
- Integrated：最新 cargo test 5 passed，无测试编译警告；cargo fmt --check 通过。包括两个原有 CPU 测试。
- 当前范围限制：固定、非空且不跨 u32 地址空间末端的 RAM；部分地址算术依赖此约定。普通数据访问当前不要求对齐；取指对齐尚待 2.2。
- 教师未代写核心实现或测试。教学中的移位、Result、Option、match、Debug、PartialEq 示例由学生应用到项目。

## Milestone 1.2 学习记录

- 学生已区分 PC 与停止状态、host panic 与 guest 异常，并明确 x0 的保证来自初始化和写入入口。
- Cpu 与 impl 移入 cpu 模块，必要接口公开，字段私有；测试移入模块，main 经接口读写。
- 学生报告外部直接写 regs 被拒绝；教师未重现失败操作，但已检查封装边界。
- 回归：cargo test 2 passed，cargo fmt --check 通过，cargo run 输出 x13=0、x14=0x12345678、x0=0。
- 当前限制：普通运行对 pc、halted 报未读取警告；测试会读取它们，执行循环尚未实现。本轮不以此警告阻塞封装验收。
- 章末口试：学生正确指出 regs、pc、halted 构成当前 Cpu 状态，完整机器还需内存；wreg(14, 7) 改变 x14，PC 与 halted 不变，教师补充其他寄存器也不变。
- 失败案例解释：学生说明私有字段令模块外直接访问被拒绝，公开 regs 会允许绕过写入接口、破坏 x0 不变量。
- 五级验收通过：状态接口可运行；既有初始化、正常及边界写入测试回归通过；四项契约已记录并区分已实现保证；学生解释访问失败与状态转移；模块拆分保留原有接口行为。

## Milestone 1.1 学习记录

- 初始化接口：学生先暴露全部字段作为参数，经不变量讨论后改为 new(pc: u32)，函数内保证 regs=[0; 32]、halted=false。两个不同初始 PC 的保存已通过自动断言验证。

- 写入与恢复：学生实现 wreg(&mut self, number: usize, value: u32)，对编号 0 提前返回；已恢复 regs 全零初始化。手工输出验证后，学生进一步编写自动断言测试。

- 寄存器读取：学生实现只读方法，后命名为 rreg；非零值与相邻寄存器对照已验证。合法编号为当前前提，非法下标会触发数组越界 panic；下一里程碑须明确编号范围契约。

- 学生选择 regs: [u32; 32]、pc: u32、halted: bool。
- 初始化：regs 全零、pc 接收调用者参数（示例为 0x80000000）、halted 为 false。
- x0 策略：初始化为零，wreg 忽略写编号 0；模块内直接改字段仍能绕过入口，访问边界尚待后续讨论。
- 最终验证：cargo test 运行 2 个测试全部通过；cargo fmt --check 通过，无编译警告。
- 初始化测试：两个初始地址分别保留、全部寄存器为零、halted=false。
- 读写测试：x2 和 x31 写入 0xffffffff 后读回一致；向 x0 写非零值仍为零；整个数组与独立期望数组比较，PC 和 halted 保持不变。
- 五级验收：Works（创建、读取、写入可运行）；Tested（正常、末端与全位值边界、禁止写 x0）；Explained（字段类型、初值、写入不变量经讨论确认）；Debuggable（能通过读回与期望值比较观察错误，测试期望值选择经纠正）；Integrated（保留初始化回归测试并通过全部现有测试）。
- 教学记录：曾将初始化测试替换为读写测试，后恢复；曾只比较两个 halted 相等而未比较 false，已修正。错误实现注入实验未收到运行证据，不标记为已完成。

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
- 教学调整：Rust 按需讲解；结构体、impl、关联函数、接收者、断言测试均已结合当前练习介绍。每次只推进一个 milestone。
