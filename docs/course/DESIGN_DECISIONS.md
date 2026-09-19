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

## 后续决策模板

- Decision:
- Context:
- Options:
- Choice:
- Reason:
- Trade-off:
