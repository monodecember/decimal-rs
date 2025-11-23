# decimal-rs 🔢

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Tests](https://img.shields.io/badge/tests-161%20passing-brightgreen.svg)]()
[![Docs](https://img.shields.io/badge/docs-100%25-blue.svg)]()

安全性、使いやすさ、パフォーマンスを考慮して設計された高性能な任意精度十進演算Rustライブラリです。

[English](README.md) | [한국어](README.ko.md) | **日本語** | [中文](README.zh.md)

## ✨ 主な機能

### 🚀 コア演算
- **完全な算術演算**: オーバーフローのないAdd、Sub、Mul、Div、Rem演算
- **複合代入演算子**: 効率的なインプレース更新のための`+=`、`-=`、`*=`
- **ビット演算**: 十進乗除算のための左/右シフト（`<<`、`>>`）
- **高度な数学**: `pow`、`gcd`、`lcm`、`factorial`

### 🛡️ 安全性と使いやすさ
- **チェック済み演算**: `checked_add`、`checked_sub`、`checked_mul`、`checked_div`
- **飽和演算**: `saturating_add`、`saturating_sub`、`saturating_mul`
- **参照演算**: 不要なクローンを避けるための`&BigInt`操作
- **プリミティブ型比較**: 変換なしで`u64`、`u32`と直接比較

### 🎨 表示とフォーマット
- **複数の基数**: 2進数、8進数、10進数、16進数（2-36進数）
- **フォーマットトレイト**: `{:b}`、`{:o}`、`{:x}`、`{:X}`サポート
- **区切り文字**: 可読性のための桁区切り
- **科学的記数法**: 自動指数フォーマット

### 🔧 ユーティリティ
- **型変換**: すべての標準整数型（`u8`-`u64`、`i8`-`i64`）間の変換
- **数字アクセス**: 個別の数字へのインデックスベースのアクセス
- **比較演算**: `max`、`min`、`clamp`、`is_even`、`is_odd`
- **Zero/Oneトレイト**: 標準的な数値トレイト実装

### ⚡ パフォーマンス
- **最適化されたアルゴリズム**: O(n)演算、二乗法による累乗
- **最小限の割り当て**: 事前割り当てによる効率的なメモリ使用
- **ゼロコスト抽象化**: 参照演算によるクローン回避

## 📦 インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
decimal-rs = "0.1.0"
```

## 🚀 クイックスタート

```rust
use decimal_rs::bigint::BigInt;

fn main() {
    // さまざまな型から生成
    let a = BigInt::from(123_u64);
    let b = BigInt::from("999999999999999999999");
    let c = "12345".parse::<BigInt>().unwrap();

    // 算術演算
    let sum = &a + &b;           // 参照演算
    let product = a * b;          // 所有権演算
    let power = c.pow(10);        // 累乗

    // さまざまな形式で表示
    println!("10進数: {}", sum);
    println!("2進数:  {:b}", sum);
    println!("16進数: {:x}", sum);
    println!("区切り: {}", sum.to_string_with_separator(','));
}
```

## 📚 例

### 基本的な算術演算

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(100_u64);
let b = BigInt::from(42_u64);

// すべての基本演算
assert_eq!(a + b, BigInt::from(142_u64));
assert_eq!(a - b, BigInt::from(58_u64));
assert_eq!(a * b, BigInt::from(4200_u64));
assert_eq!(a / b, BigInt::from(2_u64));
assert_eq!(a % b, BigInt::from(16_u64));
```

### チェック済み演算

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(10_u64);
let b = BigInt::from(20_u64);
let zero = BigInt::from(0_u64);

// チェック済み減算
assert_eq!(a.checked_sub(&b), None);  // 負の数になるのでNone

// チェック済み除算
assert_eq!(a.checked_div(&zero), None);  // ゼロ除算
assert_eq!(a.checked_div(&b), Some(BigInt::from(0_u64)));
```

### 数学関数

```rust
use decimal_rs::bigint::BigInt;

// 累乗
let base = BigInt::from(2_u64);
let result = base.pow(100);  // 2^100

// 最大公約数と最小公倍数
let a = BigInt::from(48_u64);
let b = BigInt::from(18_u64);
assert_eq!(a.gcd(&b), BigInt::from(6_u64));
assert_eq!(a.lcm(&b), BigInt::from(144_u64));

// 階乗
let n = BigInt::from(20_u64);
let fact = n.factorial();  // 20!
```

### 基数とフォーマット

```rust
use decimal_rs::bigint::BigInt;

let num = BigInt::from(255_u64);

// フォーマットトレイトの使用
assert_eq!(format!("{:b}", num), "11111111");  // 2進数
assert_eq!(format!("{:o}", num), "377");       // 8進数
assert_eq!(format!("{:x}", num), "ff");        // 16進数（小文字）
assert_eq!(format!("{:X}", num), "FF");        // 16進数（大文字）

// カスタム基数変換（2-36）
assert_eq!(num.to_string_radix(2), "11111111");
assert_eq!(num.to_string_radix(16), "ff");
assert_eq!(num.to_string_radix(36), "73");

// 桁区切り
let big = BigInt::from("1234567890");
assert_eq!(big.to_string_with_separator(','), "1,234,567,890");

// 科学的記数法
let huge = BigInt::from(123000_u64);
assert_eq!(huge.to_scientific_notation(), "1.23e5");
```

## 🎯 使用例

- **金融計算**: 丸め誤差のない正確な十進演算
- **暗号学**: 鍵生成のための大きな数の演算
- **科学計算**: 任意精度の数値シミュレーション
- **整数論**: GCD、LCM、素因数分解アルゴリズム
- **組合せ論**: 大きな入力に対する階乗、二項係数

## 🧪 テスト

包括的なテストカバレッジ：

```bash
cargo test
```

**テスト統計:**
- 92個のユニットテスト
- 69個のドキュメントテスト
- **合計161個のテスト** - すべて合格 ✅
- 100%公開APIドキュメント化

## 📈 パフォーマンス

最適化の内訳：

- **O(n)先行ゼロ削除**（O(n²)から改善）
- **最小化されたベクトル演算**（3回の反転から1回に削減）
- **二乗法による累乗** O(log n)演算
- **事前割り当てバッファ** ヒープ割り当ての削減
- **参照演算** 不要なクローンの回避

## 🗺️ ロードマップ

### 実装済み ✅
- [x] 完全なBigInt算術演算（Add、Sub、Mul、Div、Rem）
- [x] チェック済みおよび飽和演算
- [x] 数学関数（pow、gcd、lcm、factorial）
- [x] 複数の表示形式と基数
- [x] 包括的な型変換
- [x] 効率性のための参照演算
- [x] ZeroおよびOneトレイト

### 計画中 🚧
- [ ] 完全なDecimal型実装
- [ ] Serdeシリアライゼーションサポート
- [ ] 高度なアルゴリズム（Karatsuba乗算、FFT）
- [ ] ベンチマークスイート
- [ ] WASMサポート
- [ ] No-std互換性

## 📖 ドキュメント

完全なドキュメントの生成：

```bash
cargo doc --open
```

すべての公開APIは例とdocテストでドキュメント化されています。

## 🤝 貢献

貢献を歓迎します！イシューやプルリクエストをお気軽に提出してください。

### 開発

```bash
# テスト実行
cargo test

# 出力付きでテスト実行
cargo test -- --nocapture

# フォーマットチェック
cargo fmt --check

# Clippy実行
cargo clippy
```

## 📄 ライセンス

このプロジェクトはMITライセンスの下でオープンソースとして提供されています。

## 🙏 謝辞

Rustの強力な型システムとゼロコスト抽象化で構築されています。

---

**ステータス:** 任意精度整数演算のためのプロダクション準備完了
**現在のバージョン:** 0.1.0
**Rustバージョン:** 1.70+
