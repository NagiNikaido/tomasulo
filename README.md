# Tomasulo simulator

### 计64 2016011322 陶东来

## 设计思路

- 将硬件平台拆分成指令原型、指令、通用数据总线（CDB）、保留站和寄存器5个部件。
- 前四者分别实现，寄存器由于比较简单直接实现在了硬件平台（platform)中。
- 在硬件平台中对诸部件进行组装，实现流水线串联和单周期执行功能。

## 运行方式

首先需要安装`rust`。

Windows 平台直接访问![官方网站](https://www.rust-lang.org/learn/get-started)即可。

\*nix平台执行
```bash
curl https://sh.rustup.rs -sSf | sh
```
即可一键安装，版本选择默认的`stable`。

接下来进行编译。在项目根目录执行
```bash
cargo build --release
```

目标文件将会生成在`target/release/tomasulo{.exe}`。模拟器默认使用标准输入输出，因此需要用管道将测试文件输送给模拟器。
```bash
cat test.nel | ./target/release/tomasulo
```

开始执行之后屏幕会飞速滚动（当然也可能`panic`，因为没有对输入进行鲁棒性测试）。截取一段输出信息如下
```
...

Cycle #17
Regs: [
    F0: 0, F1: 3, F2: 0, F3: -1,
    F4: 0, F5: 0, F6: 0, F7: 0,
    F8: 0, F9: 0, F10: 0, F11: 0,
    F12: 0, F13: 0, F14: 0, F15: 0,
    F16: 0, F17: 0, F18: 0, F19: 0,
    F20: 0, F21: 0, F22: 0, F23: 0,
    F24: 0, F25: 0, F26: 0, F27: 0,
    F28: 0, F29: 0, F30: 0, F31: 0,
    F32: 8,
]
Stations: [
    Station  0 / Ars 0: IDLE
    Station  1 / Ars 1: ISSUE #6   SUB   Station  7  0x00000003
    Station  2 / Ars 2: IDLE
    Station  3 / Ars 3: IDLE
    Station  4 / Ars 4: IDLE
    Station  5 / Ars 5: IDLE
    Station  6 / Mrs 0: EXEC  #4   MUL   0x00000003  0xffffffff
    Station  7 / Mrs 1: EXEC  #5   DIV   0xffffffff  0x00000003
    Station  8 / Mrs 2: IDLE
]
Load Buffers: [
    Station  9 / LB  0: IDLE
    Station 10 / LB  1: IDLE
    Station 11 / LB  2: IDLE
]

...

Cycle #52
inst #6 write back.
Regs: [
    F0: 0, F1: 3, F2: 0, F3: -1,
    F4: -3, F5: 0, F6: 0, F7: 0,
    F8: 0, F9: 0, F10: 0, F11: 0,
    F12: 0, F13: 0, F14: 0, F15: 0,
    F16: 0, F17: 0, F18: 0, F19: 0,
    F20: 0, F21: 0, F22: 0, F23: 0,
    F24: 0, F25: 0, F26: 0, F27: 0,
    F28: 0, F29: 0, F30: 0, F31: 0,
    F32: 8,
]
Stations: [
    Station  0 / Ars 0: IDLE
    Station  1 / Ars 1: IDLE
    Station  2 / Ars 2: IDLE
    Station  3 / Ars 3: IDLE
    Station  4 / Ars 4: IDLE
    Station  5 / Ars 5: IDLE
    Station  6 / Mrs 0: IDLE
    Station  7 / Mrs 1: IDLE
    Station  8 / Mrs 2: IDLE
]
Load Buffers: [
    Station  9 / LB  0: IDLE
    Station 10 / LB  1: IDLE
    Station 11 / LB  2: IDLE
]

=============================
No.     Issue  Exec  WriteBack
#  0  LD    0  3  4
#  1  LD    1  4  5
#  2  LD    2  6  7
#  3 ADD    3  8  9
#  4 MUL    4  19  20
#  5 DIV    5  47  48
#  6 SUB    6  51  52
#  7 JUMP   7  8  9
```

模拟器中为实现方便，将`F32`用作pc，`Cycle`计数从0开始。`Regs`显示的为实际寄存器的值，而寄存器状态值由于重命名之后已被替换成了`Station`编号未记录。

## 测试用例

`test[0-2].nel`为既有测例。`test3.nel`为计算1-100和的测试用例，输出寄存器为`F2`和`F5`。`test4.nel`为前者的简化版，用于测试功能部件少于保留站时FIFO的特性。

## 如有问题？

可邮件联系tdl16@mails.tsinghua.edu.cn。