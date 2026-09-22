# FerrumRV 设计决策

## Chapter 1：学生选择的最小 CPU 状态

- Decision: 用一个 Cpu 结构体组织通用寄存器、PC 和模拟器停止状态。
- Context: 第一版模拟 RV32I，尚未实现取指、内存或执行指令。
- Options: 寄存器内容与编号须区分；PC 地址宽度取决于 guest，不能依赖 host 的 usize。
- Choice: regs 为 [u32; 32] 且全零；pc 为 u32，初值 0x80000000；halted 为 bool，初值 false。
- Reason: 寄存器内容保存 32 位位模式；将地址与停止状态分开表示。
- Trade-off: 初始 PC 是本机布局选择，尚未实现对应 RAM 映射；后续须明确 guest 地址到 host 下标的转换。

## Chapter 1：初始化接口（2026-09-16）

- Decision: Cpu::new 仅接收初始 PC。
- Context: 学生选择由调用者指定启动地址；最初将 regs 和 halted 也作为参数，会允许创建违反初始化约定的状态。
- Options: 所有字段由调用者提供；或只允许指定 PC。
- Choice: new(pc: u32)，内部设置 regs=[0; 32]、halted=false。
- Reason: 调用者选择启动位置，初始化函数保证其余初始状态。
- Trade-off: 当前接口不是任意 CPU 快照的恢复接口；PC 地址合法性及映射留待后续约定。

## Chapter 1：x0 写入策略

- Decision: 初始化 x0 为零，并在寄存器写入入口忽略写 x0 的请求。
- Context: 学生最初考虑读取时返回零，经讨论后选择同时保持内部存储为零。
- Options: 读取时特殊处理；或初始化为零并忽略写入。
- Choice: 忽略写入；学生已实现 wreg 方法对编号 0 提前返回。
- Reason: 使内部状态和可观察值一致。
- Trade-off: 已把 Cpu 放入独立 cpu 模块，字段私有；模块外不能直接写数组，模块内部仍必须遵守不变量。

## Milestone 1.2：状态契约与当前保证范围

| 项目 | 契约 | 当前实现 |
|---|---|---|
| x0 | 始终为 0 | new 初始化为零，wreg 忽略对 x0 的写入；rreg 只读数组，不负责修正值。字段私有，模块外无法直接写入。 |
| PC | 第一版仅 RV32I，取指地址须 4 字节对齐，即 pc % 4 == 0 | new 原样保存传入值，尚无对齐检查；目前由调用者提供对齐地址。取指尚未实现。 |
| 寄存器编号 | 合法范围 0..32，即 0 至 31 | 调用者须满足前提；非法数组下标导致 host Rust panic，不会自动进入 guest 异常处理。 |
| halted | false 允许推进执行；true 表示执行循环应停止推进，仍可观察状态 | 初始化为 false；执行循环尚未实现。PC 描述位置，与是否停止是独立状态。 |

- 模块边界：Cpu、new、rreg、wreg 公开，字段私有；测试保留在 cpu 模块内。
- 验证：模块拆分后两个测试通过，格式检查通过，运行的寄存器结果正确；学生报告尝试外部直接写 regs 被编译器拒绝，教师检查了私有字段边界，但未收到该失败诊断。
- 普通运行目前对 pc、halted 有未读取警告；两者已在单元测试中被检查，后续执行逻辑将使用它们。

## Chapter 2：第一版 Memory 契约

- Decision: 固定 16 字节 RAM，以 Vec<u8> 保存，guest 起点 0x80000000；容量取容器长度。
- Context: 学生选择先固定容量，专注字节寻址与边界；由 guest 地址减起点得到 host 偏移，下界须在无符号减法前检查。
- Options: 固定数组或 Vec；错误描述内部失败步骤或原始访问请求。
- Choice: Vec<u8>；8/16/32 位小端访问；Result 的错误为 AddrError::InvalidRange { start_addr, length }，统一报告原始请求。
- Reason: 成功值零与访问失败可明确区分；外部错误含义不依赖内部如何拆分读取。
- Trade-off: 当前只保证固定非空 RAM 配置；普通数据访问允许非对齐地址，取指对齐另行规定。失败写入须保持整块内存不变，? 或 match 提前返回本身不会回滚状态。

## Chapter 2：Fetch 与模块边界

- Decision: fetch 只使用当前 PC 读取原始 32 位值；CPU、Memory 与其测试分文件组织。
- Context: 尚未解码或执行，取指不推进 PC。
- Options: 共享借用或可变借用内存；集中测试或各模块子测试。
- Choice: fetch(&self, &Memory) -> Result<u32, AddrError>；调用 read_word 并保留错误。Memory 的必要跨模块接口使用 pub(crate)，字段保持私有。
- Reason: 共享借用表达只读行为；各模块测试可检查本模块私有状态，CPU 测试通过 Memory 接口准备数据。
- Trade-off: 当前字段无内部可变性，状态不变由类型约束和审查确认，无需在 Fetch 测试重复完整内存验证。PC 四字节对齐仍由调用者保证，运行时检查尚未实现。

## Chapter 3：指令表示与解码

- Decision: 解码独立于 CPU 状态，以 Result 返回内部指令或不支持的机器码。
- Context: 当前只支持 RV32I ADDI，执行阶段尚未实现。
- Options: 解码读取 CPU 或只接收机器码；错误包含地址或由调用者补充地址。
- Choice: decode(raw: u32) -> Result<Inst, DecodeError>；Inst::Addi 保存 rs1/rd: u8、imm: i32；UnsupportedInst 保存原始 raw。opcode/funct3 使用 const 命名常量。
- Reason: 解码保存寄存器编号，执行时才读取寄存器值；同时检查 opcode 和 funct3；错误不依赖 CPU，调用者负责地址上下文。
- Trade-off: u8 本身不保证编号小于 32，解码通过五位掩码保证范围；立即数先将 raw 转为 i32 再右移完成符号扩展，此方法依赖当前 I-type 立即数位于最高十二位的布局。

## Milestone 4.1：执行与单步错误

- Decision: execute 修改 CPU，step 串联取指、解码和执行并增加错误上下文。
- Context: 仅支持 ADDI，尚未实现 guest 停止行为。
- Options: 各层各自更新 PC 或统一由执行阶段更新；错误丢失底层原因或包装保留。
- Choice: execute 使用 wrapping_add 计算和推进 PC，复用 wreg 保护 x0；step 仅在取指与解码成功后调用 execute，失败返回 StepError::Fetch/Decode，保存 PC 和底层错误。
- Reason: 防止重复推进 PC；前两阶段只读，因此失败前没有 CPU 写入，不需要回滚。
- Trade-off: 当前 execute 接收合法解码结果，step 以前置条件要求 CPU 未停止且 PC 对齐；后续扩展指令或错误阶段时需重新审视状态更新顺序。

## Milestone 4.2：R-type 与可配置 RAM 容量

- Decision: 增加 ADD/SUB；RAM 容量由实例构造参数指定。
- Context: R-type 使用两个源寄存器；新增测试需要较大内存，而旧边界测试依赖16字节布局。
- Options: 全局扩容并重写旧预期；或每个实例选择容量。
- Choice: Memory::new(size: usize)，base_addr 仍为 0x80000000；旧测试传16，新 ADD/SUB 测试传256。OP 解码检查完整 funct7/funct3；执行读取两个源后 wrapping_add/sub，再由 wreg 写回并推进 PC。
- Reason: 保持旧测试有效，独立控制新测试容量；寄存器存储32位位模式，SUB 无需将寄存器改为有符号类型。
- Trade-off: 当前约定容量非零且映射不跨地址空间末端；尚未把这些前提转换为构造错误检查。不支持的 OP 编码返回错误，不能保留 todo! 导致 host panic。

## Milestone 4.3：教学停止协议

- Decision: 当前模拟器将完整编码 `0x00100073` 解码为 EBREAK；执行时设置 halted=true，PC 保留在 EBREAK 地址；停止后 step 返回 Ok(()) 且不取指。
- Context: 真实 RISC-V EBREAK 向执行环境请求断点处理，通常进入调试/异常路径，不等同于关闭 CPU。
- Options: 建模断点异常；或第一版直接提供可观察的 host 停止状态。
- Choice: 先采用 halted 简化协议，并明确它不是完整 ISA 异常语义。
- Reason: 当前尚未实现 trap、privilege 或调试环境；协议仍能让算术程序可靠停机并保留触发地址。
- Trade-off: 后续实现异常系统时必须把 EBREAK 从直接停机改为请求 trap，并重新定义 step 与运行循环的接口。

## 后续决策模板

- Decision:
- Context:
- Options:
- Choice:
- Reason:
- Trade-off:
