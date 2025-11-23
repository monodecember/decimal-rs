# decimal-rs

A Rust library for arbitrary-precision decimal arithmetic.

## Overview

`decimal-rs` provides two main types for working with numbers of arbitrary size:

- **`BigInt`**: Arbitrary-precision integers
- **`Decimal`**: Arbitrary-precision decimal numbers with configurable precision

## Features

- ✅ Arbitrary-precision integer arithmetic
- ✅ Addition operations with automatic carry handling
- ✅ Robust error handling with detailed error types
- ✅ Comprehensive test coverage
- ✅ Full documentation with examples

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
decimal-rs = "0.1.0"
```

### Basic Examples

#### BigInt Addition

```rust
use decimal_rs::bigint::BigInt;

fn main() {
    let a = BigInt::from("123456789012345678901234567890");
    let b = BigInt::from("987654321098765432109876543210");
    let c = a + b;

    println!("{:?}", c); // BigInt with sum
}
```

#### Safe Parsing with Error Handling

```rust
use decimal_rs::bigint::BigInt;
use std::str::FromStr;

fn main() {
    match BigInt::from_str("12345") {
        Ok(num) => println!("Parsed successfully: {:?}", num),
        Err(e) => println!("Parse error: {}", e),
    }

    // Or using parse()
    let result: Result<BigInt, _> = "67890".parse();
}
```

#### Decimal Numbers

```rust
use decimal_rs::decimal::Decimal;

fn main() {
    let dec = Decimal::new(28); // 28 significant digits
    dec.debug_display();
}
```

## Error Handling

The library uses the `DecimalError` enum for all error cases:

- `InvalidDigit(char)`: An invalid character was found during parsing
- `EmptyInput`: Attempted to parse an empty string
- `InvalidFormat(String)`: The input format is invalid

## Development Status

This library is in early development (v0.1.0). Current features:

- ✅ BigInt addition
- ✅ Error handling with Result types
- ✅ Comprehensive documentation
- ⏳ Subtraction, multiplication, division (planned)
- ⏳ Full Decimal implementation (planned)
- ⏳ Comparison operations (planned)

## Testing

Run the test suite:

```bash
cargo test
```

Current test coverage: 24 tests covering edge cases, error handling, and core functionality.

## Documentation

Generate and view the documentation:

```bash
cargo doc --open
```

## License

This project is open source.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
