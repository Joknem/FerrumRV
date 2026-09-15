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
- Trade-off: 若存在绕过该入口的直接字段写入，仍可能破坏不变量；后续需讨论模块与访问边界。

## 记录模板

- Decision:
- Context:
- Options:
- Choice:
- Reason:
- Trade-off:
