# decimal-rs 🔢

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Tests](https://img.shields.io/badge/tests-161%20passing-brightgreen.svg)]()
[![Docs](https://img.shields.io/badge/docs-100%25-blue.svg)]()

一个高性能的Rust任意精度十进制运算库。专注于安全性、易用性和性能。

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | **中文**

## ✨ 主要特性

### 🚀 核心运算
- **完整的算术运算**: 无溢出的加、减、乘、除、取模运算
- **复合赋值运算符**: 用于高效就地更新的`+=`、`-=`、`*=`
- **位运算**: 用于十进制乘除法的左/右移位（`<<`、`>>`）
- **高级数学**: `pow`、`gcd`、`lcm`、`factorial`

### 🛡️ 安全性与便利性
- **检查运算**: `checked_add`、`checked_sub`、`checked_mul`、`checked_div`
- **饱和运算**: `saturating_add`、`saturating_sub`、`saturating_mul`
- **引用运算**: 使用`&BigInt`避免不必要的克隆
- **原始类型比较**: 直接与`u64`、`u32`比较，无需转换

### 🎨 显示与格式化
- **多种进制**: 二进制、八进制、十进制、十六进制（2-36进制）
- **格式化trait**: 支持`{:b}`、`{:o}`、`{:x}`、`{:X}`
- **分隔符**: 用于提高可读性的千位分隔符
- **科学记数法**: 自动指数格式化

### 🔧 实用工具
- **类型转换**: 所有标准整数类型（`u8`-`u64`、`i8`-`i64`）之间的转换
- **数字访问**: 基于索引访问单个数字
- **比较运算**: `max`、`min`、`clamp`、`is_even`、`is_odd`
- **Zero/One trait**: 标准数值trait实现

### ⚡ 性能
- **优化的算法**: O(n)运算，平方求幂
- **最小分配**: 通过预分配实现高效的内存使用
- **零成本抽象**: 引用运算避免克隆

## 📦 安装

在`Cargo.toml`中添加：

```toml
[dependencies]
decimal-rs = "0.1.0"
```

## 🚀 快速开始

```rust
use decimal_rs::bigint::BigInt;

fn main() {
    // 从各种类型创建
    let a = BigInt::from(123_u64);
    let b = BigInt::from("999999999999999999999");
    let c = "12345".parse::<BigInt>().unwrap();

    // 算术运算
    let sum = &a + &b;           // 引用运算
    let product = a * b;          // 所有权运算
    let power = c.pow(10);        // 幂运算

    // 以多种格式显示
    println!("十进制: {}", sum);
    println!("二进制: {:b}", sum);
    println!("十六进制: {:x}", sum);
    println!("带分隔符: {}", sum.to_string_with_separator(','));
}
```

## 📚 示例

### 基本算术运算

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(100_u64);
let b = BigInt::from(42_u64);

// 所有基本运算
assert_eq!(a + b, BigInt::from(142_u64));
assert_eq!(a - b, BigInt::from(58_u64));
assert_eq!(a * b, BigInt::from(4200_u64));
assert_eq!(a / b, BigInt::from(2_u64));
assert_eq!(a % b, BigInt::from(16_u64));
```

### 检查运算

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(10_u64);
let b = BigInt::from(20_u64);
let zero = BigInt::from(0_u64);

// 检查减法
assert_eq!(a.checked_sub(&b), None);  // 会变成负数，返回None

// 检查除法
assert_eq!(a.checked_div(&zero), None);  // 除以零
assert_eq!(a.checked_div(&b), Some(BigInt::from(0_u64)));
```

### 数学函数

```rust
use decimal_rs::bigint::BigInt;

// 幂运算
let base = BigInt::from(2_u64);
let result = base.pow(100);  // 2^100

// 最大公约数和最小公倍数
let a = BigInt::from(48_u64);
let b = BigInt::from(18_u64);
assert_eq!(a.gcd(&b), BigInt::from(6_u64));
assert_eq!(a.lcm(&b), BigInt::from(144_u64));

// 阶乘
let n = BigInt::from(20_u64);
let fact = n.factorial();  // 20!
```

### 进制与格式化

```rust
use decimal_rs::bigint::BigInt;

let num = BigInt::from(255_u64);

// 使用格式化trait
assert_eq!(format!("{:b}", num), "11111111");  // 二进制
assert_eq!(format!("{:o}", num), "377");       // 八进制
assert_eq!(format!("{:x}", num), "ff");        // 十六进制（小写）
assert_eq!(format!("{:X}", num), "FF");        // 十六进制（大写）

// 自定义进制转换（2-36）
assert_eq!(num.to_string_radix(2), "11111111");
assert_eq!(num.to_string_radix(16), "ff");
assert_eq!(num.to_string_radix(36), "73");

// 千位分隔符
let big = BigInt::from("1234567890");
assert_eq!(big.to_string_with_separator(','), "1,234,567,890");

// 科学记数法
let huge = BigInt::from(123000_u64);
assert_eq!(huge.to_scientific_notation(), "1.23e5");
```

## 🎯 使用场景

- **金融计算**: 无舍入误差的精确十进制运算
- **密码学**: 密钥生成的大数运算
- **科学计算**: 任意精度数值模拟
- **数论**: GCD、LCM、质因数分解算法
- **组合数学**: 大输入的阶乘、二项式系数

## 🧪 测试

全面的测试覆盖：

```bash
cargo test
```

**测试统计:**
- 92个单元测试
- 69个文档测试
- **总计161个测试** - 全部通过 ✅
- 100%公共API文档化

## 📈 性能

优化亮点：

- **O(n)前导零移除**（从O(n²)改进）
- **最小化向量操作**（从3次反转减少到1次）
- **平方求幂** O(log n)运算
- **预分配缓冲区** 减少堆分配
- **引用运算** 避免不必要的克隆

## 🗺️ 路线图

### 已实现 ✅
- [x] 完整的BigInt算术运算（加、减、乘、除、模）
- [x] 检查和饱和运算
- [x] 数学函数（pow、gcd、lcm、factorial）
- [x] 多种显示格式和进制
- [x] 全面的类型转换
- [x] 用于提高效率的引用运算
- [x] Zero和One trait

### 计划中 🚧
- [ ] 完整的Decimal类型实现
- [ ] Serde序列化支持
- [ ] 高级算法（Karatsuba乘法、FFT）
- [ ] 基准测试套件
- [ ] WASM支持
- [ ] No-std兼容性

## 📖 文档

生成完整文档：

```bash
cargo doc --open
```

所有公共API都有示例和文档测试。

## 🤝 贡献

欢迎贡献！请随时提交issue或pull request。

### 开发

```bash
# 运行测试
cargo test

# 带输出运行测试
cargo test -- --nocapture

# 检查格式化
cargo fmt --check

# 运行clippy
cargo clippy
```

## 📄 许可证

本项目在MIT许可证下开源。

## 🙏 致谢

基于Rust强大的类型系统和零成本抽象构建。

---

**状态:** 任意精度整数运算生产就绪
**当前版本:** 0.1.0
**Rust版本:** 1.70+
