# FerrumRV Bug 日记

## Milestone 0.1：输出宏名称拼写错误

- Symptom: cargo test 报 cannot find macro `pritln` in this scope，随后报告 could not compile。
- Wrong hypothesis: 未记录学生的错误假设；学生正确判断为编译阶段失败。
- Observation: 学生截图定位 src/main.rs:2:5；编译器提示相近名称 println。教师解释标准库路径用于展示相近宏定义，无需修改标准库。
- Root cause: 输出宏调用写成 pritln!，名称少了 n。
- Fix: 学生亲手改回 println!；教师读取源码并运行 cargo build、cargo test，均成功退出，测试用例数为 0。
- What I learned: 学生已确认 could not compile 表明编译失败，测试用例尚未执行；在教师讲解后确认测试程序成功运行但零用例不能验证 Hello world 的输出行为。

## 记录模板

- Symptom:
- Wrong hypothesis:
- Observation:
- Root cause:
- Fix:
- What I learned:
