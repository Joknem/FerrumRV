# FerrumRV Bug 日记

## Milestone 0.1：输出宏名称拼写错误

- Symptom: cargo test 报 cannot find macro `pritln` in this scope，随后报告 could not compile。
- Wrong hypothesis: 未记录学生的错误假设；学生正确判断为编译阶段失败。
- Observation: 学生截图定位 src/main.rs:2:5；编译器提示相近名称 println。教师解释标准库路径用于展示相近宏定义，无需修改标准库。
- Root cause: 输出宏调用写成 pritln!，名称少了 n。
- Fix: 学生亲手改回 println!；教师读取源码并运行 cargo build、cargo test，均成功退出，测试用例数为 0。
- What I learned: 学生已确认 could not compile 表明编译失败，测试用例尚未执行；在教师讲解后确认测试程序成功运行但零用例不能验证 Hello world 的输出行为。

## Milestone 2.1：多字节写入检查顺序与错误丢弃

- Symptom: 早期实现可能在跨界写入时只修改部分字节，忽略子写入 Err 后还返回 Ok。
- Wrong hypothesis: 先逐字节写入、遇错返回即可满足失败时内存不变；后经讨论纠正。
- Observation: 教师源码推演末尾跨界与起点前跨入 RAM 的例子；未对旧实现运行故障注入。学生随后加入完整错误及每次失败后整块内存比较的自动测试。
- Root cause: 全范围检查晚于第一次写入，且最初丢弃 wbyte 返回的 Result。
- Fix: 学生预先检查地址溢出和上界，利用首个子写入在修改前检查下界；传播或转换错误；最终各宽度失败状态保持测试通过。
- What I learned: 学生解释 ? 只提前返回，不撤销之前的修改；错误元数据应描述外层原始请求。

## Milestone 3.3：负立即数变成正数

- Symptom: decode(0xffd30293) 返回 imm=4093，期望 -3。
- Wrong hypothesis: 未记录学生认为普通类型转换足够的独立假设；先前版本按教学步骤暂只处理正立即数。
- Observation: 最初测试机器码写错导致 UnsupportedInst；学生逐字段修正后，断言显示 rs1/rd 正确，只有 imm 不一致。
- Root cause: 对 u32 先右移补零，再 as i32 保留 4093，未执行十二位符号扩展。
- Fix: 学生将转换放在右移之前，使 i32 右移补符号位；-3、-1、-2048 与已有正数、零及错误测试均通过。
- What I learned: 学生解释右移补位取决于左操作数类型；须先确认测试机器码有效，再定位解码语义错误。

## 后续记录模板

- Symptom:
- Wrong hypothesis:
- Observation:
- Root cause:
- Fix:
- What I learned:
