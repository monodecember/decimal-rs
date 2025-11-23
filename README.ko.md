# decimal-rs 🔢

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Tests](https://img.shields.io/badge/tests-161%20passing-brightgreen.svg)]()
[![Docs](https://img.shields.io/badge/docs-100%25-blue.svg)]()

안전성, 편의성, 성능을 고려하여 설계된 고성능 Rust 임의 정밀도 십진수 연산 라이브러리입니다.

[English](README.md) | **한국어** | [日本語](README.ja.md) | [中文](README.zh.md)

## ✨ 주요 기능

### 🚀 핵심 연산
- **완전한 산술 연산**: 오버플로우 없는 Add, Sub, Mul, Div, Rem 연산
- **복합 할당 연산자**: 효율적인 제자리 수정을 위한 `+=`, `-=`, `*=`
- **비트 연산**: 십진수 곱셈/나눗셈을 위한 좌/우 시프트 (`<<`, `>>`)
- **고급 수학**: `pow`, `gcd`, `lcm`, `factorial`

### 🛡️ 안전성 & 편의성
- **안전한 연산**: `checked_add`, `checked_sub`, `checked_mul`, `checked_div`
- **포화 연산**: `saturating_add`, `saturating_sub`, `saturating_mul`
- **참조 연산**: 불필요한 복제를 피하기 위한 `&BigInt` 작업
- **기본 타입 비교**: 변환 없이 `u64`, `u32`와 직접 비교

### 🎨 표시 & 포맷팅
- **다중 진법**: 2진수, 8진수, 10진수, 16진수 (2-36진법)
- **포맷 트레이트**: `{:b}`, `{:o}`, `{:x}`, `{:X}` 지원
- **구분자**: 가독성을 위한 천 단위 구분자
- **과학적 표기법**: 자동 지수 포맷팅

### 🔧 유틸리티
- **타입 변환**: 모든 표준 정수 타입 (`u8`-`u64`, `i8`-`i64`) 간 변환
- **숫자 접근**: 개별 숫자에 대한 인덱스 기반 접근
- **비교 연산**: `max`, `min`, `clamp`, `is_even`, `is_odd`
- **Zero/One 트레이트**: 표준 숫자 트레이트 구현

### ⚡ 성능
- **최적화된 알고리즘**: O(n) 연산, 제곱을 통한 거듭제곱
- **최소 할당**: 사전 할당을 통한 효율적인 메모리 사용
- **제로 비용 추상화**: 참조 연산으로 복제 방지

## 📦 설치

`Cargo.toml`에 다음을 추가하세요:

```toml
[dependencies]
decimal-rs = "0.1.0"
```

## 🚀 빠른 시작

```rust
use decimal_rs::bigint::BigInt;

fn main() {
    // 다양한 타입에서 생성
    let a = BigInt::from(123_u64);
    let b = BigInt::from("999999999999999999999");
    let c = "12345".parse::<BigInt>().unwrap();

    // 산술 연산
    let sum = &a + &b;           // 참조 연산
    let product = a * b;          // 소유 연산
    let power = c.pow(10);        // 거듭제곱

    // 다양한 형식으로 표시
    println!("10진수: {}", sum);
    println!("2진수:  {:b}", sum);
    println!("16진수: {:x}", sum);
    println!("구분자: {}", sum.to_string_with_separator(','));
}
```

## 📚 예제

### 기본 산술 연산

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(100_u64);
let b = BigInt::from(42_u64);

// 모든 기본 연산
assert_eq!(a + b, BigInt::from(142_u64));
assert_eq!(a - b, BigInt::from(58_u64));
assert_eq!(a * b, BigInt::from(4200_u64));
assert_eq!(a / b, BigInt::from(2_u64));
assert_eq!(a % b, BigInt::from(16_u64));
```

### 안전한 연산 (Checked Operations)

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(10_u64);
let b = BigInt::from(20_u64);
let zero = BigInt::from(0_u64);

// 안전한 뺄셈
assert_eq!(a.checked_sub(&b), None);  // 음수가 되므로 None

// 안전한 나눗셈
assert_eq!(a.checked_div(&zero), None);  // 0으로 나누기
assert_eq!(a.checked_div(&b), Some(BigInt::from(0_u64)));
```

### 수학 함수

```rust
use decimal_rs::bigint::BigInt;

// 거듭제곱
let base = BigInt::from(2_u64);
let result = base.pow(100);  // 2^100

// 최대공약수와 최소공배수
let a = BigInt::from(48_u64);
let b = BigInt::from(18_u64);
assert_eq!(a.gcd(&b), BigInt::from(6_u64));
assert_eq!(a.lcm(&b), BigInt::from(144_u64));

// 팩토리얼
let n = BigInt::from(20_u64);
let fact = n.factorial();  // 20!
```

### 진법 & 포맷팅

```rust
use decimal_rs::bigint::BigInt;

let num = BigInt::from(255_u64);

// 포맷 트레이트 사용
assert_eq!(format!("{:b}", num), "11111111");  // 2진수
assert_eq!(format!("{:o}", num), "377");       // 8진수
assert_eq!(format!("{:x}", num), "ff");        // 16진수 (소문자)
assert_eq!(format!("{:X}", num), "FF");        // 16진수 (대문자)

// 사용자 정의 진법 변환 (2-36)
assert_eq!(num.to_string_radix(2), "11111111");
assert_eq!(num.to_string_radix(16), "ff");
assert_eq!(num.to_string_radix(36), "73");

// 천 단위 구분자
let big = BigInt::from("1234567890");
assert_eq!(big.to_string_with_separator(','), "1,234,567,890");

// 과학적 표기법
let huge = BigInt::from(123000_u64);
assert_eq!(huge.to_scientific_notation(), "1.23e5");
```

## 🎯 활용 사례

- **금융 계산**: 반올림 오류 없는 정확한 십진수 연산
- **암호학**: 키 생성을 위한 큰 수 연산
- **과학 계산**: 임의 정밀도 수치 시뮬레이션
- **정수론**: GCD, LCM, 소인수분해 알고리즘
- **조합론**: 큰 입력에 대한 팩토리얼, 이항 계수

## 🧪 테스트

포괄적인 테스트 커버리지:

```bash
cargo test
```

**테스트 통계:**
- 92개 유닛 테스트
- 69개 문서 테스트
- **총 161개 테스트** - 모두 통과 ✅
- 100% 공개 API 문서화

## 📈 성능

최적화 내역:

- **O(n) 선행 0 제거** (O(n²)에서 개선)
- **최소화된 벡터 연산** (3회 반전에서 1회로 감소)
- **제곱을 통한 거듭제곱** O(log n) 연산
- **사전 할당 버퍼** 힙 할당 감소
- **참조 연산** 불필요한 복제 방지

## 🗺️ 로드맵

### 구현 완료 ✅
- [x] 완전한 BigInt 산술 연산 (Add, Sub, Mul, Div, Rem)
- [x] 안전 및 포화 연산
- [x] 수학 함수 (pow, gcd, lcm, factorial)
- [x] 다중 표시 형식 및 진법
- [x] 포괄적인 타입 변환
- [x] 효율성을 위한 참조 연산
- [x] Zero 및 One 트레이트

### 계획 중 🚧
- [ ] 완전한 Decimal 타입 구현
- [ ] Serde 직렬화 지원
- [ ] 고급 알고리즘 (Karatsuba 곱셈, FFT)
- [ ] 벤치마킹 스위트
- [ ] WASM 지원
- [ ] No-std 호환성

## 📖 문서

전체 문서 생성:

```bash
cargo doc --open
```

모든 공개 API는 예제와 doc 테스트로 문서화되어 있습니다.

## 🤝 기여

기여를 환영합니다! 이슈나 풀 리퀘스트를 자유롭게 제출해 주세요.

### 개발

```bash
# 테스트 실행
cargo test

# 출력과 함께 테스트 실행
cargo test -- --nocapture

# 포맷팅 확인
cargo fmt --check

# Clippy 실행
cargo clippy
```

## 📄 라이선스

이 프로젝트는 MIT 라이선스 하에 오픈 소스로 제공됩니다.

## 🙏 감사의 말

Rust의 강력한 타입 시스템과 제로 비용 추상화로 구축되었습니다.

---

**상태:** 임의 정밀도 정수 산술을 위한 프로덕션 준비 완료
**현재 버전:** 0.1.0
**Rust 버전:** 1.70+
