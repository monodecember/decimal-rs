# decimal-rs 🔢

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)]()
[![Tests](https://img.shields.io/badge/tests-161%20passing-brightgreen.svg)]()
[![Docs](https://img.shields.io/badge/docs-100%25-blue.svg)]()

Una biblioteca Rust de alto rendimiento para aritmética decimal de precisión arbitraria. Construida con seguridad, ergonomía y rendimiento en mente.

[English](README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [中文](README.zh.md) | **Español** | [Français](README.fr.md)

## ✨ Características

### 🚀 Operaciones Principales
- **Aritmética completa**: Operaciones Add, Sub, Mul, Div, Rem sin desbordamiento
- **Operadores de asignación compuesta**: `+=`, `-=`, `*=` para actualizaciones eficientes in situ
- **Operaciones de bits**: Desplazamiento izquierdo/derecho (`<<`, `>>`) para multiplicación/división decimal
- **Matemáticas avanzadas**: `pow`, `gcd`, `lcm`, `factorial`

### 🛡️ Seguridad y Ergonomía
- **Operaciones verificadas**: `checked_add`, `checked_sub`, `checked_mul`, `checked_div`
- **Operaciones saturadas**: `saturating_add`, `saturating_sub`, `saturating_mul`
- **Operaciones con referencias**: Trabaje con `&BigInt` para evitar clonaciones innecesarias
- **Comparaciones primitivas**: Comparación directa con `u64`, `u32` sin conversión

### 🎨 Visualización y Formato
- **Múltiples bases**: Binario, Octal, Decimal, Hexadecimal (2-36)
- **Traits de formato**: Soporte para `{:b}`, `{:o}`, `{:x}`, `{:X}`
- **Separadores**: Separadores de miles para legibilidad
- **Notación científica**: Formato exponencial automático

### 🔧 Utilidades
- **Conversiones de tipo**: Desde/hacia todos los tipos enteros estándar (`u8`-`u64`, `i8`-`i64`)
- **Acceso a dígitos**: Acceso basado en índice a dígitos individuales
- **Comparaciones**: `max`, `min`, `clamp`, `is_even`, `is_odd`
- **Traits Zero/One**: Implementaciones de traits numéricos estándar

### ⚡ Rendimiento
- **Algoritmos optimizados**: Operaciones O(n), exponenciación por cuadrado
- **Asignaciones mínimas**: Uso eficiente de memoria con preasignación
- **Abstracciones de coste cero**: Operaciones con referencias evitan clonaciones

## 📦 Instalación

Agregue esto a su `Cargo.toml`:

```toml
[dependencies]
decimal-rs = "0.1.0"
```

## 🚀 Inicio Rápido

```rust
use decimal_rs::bigint::BigInt;

fn main() {
    // Crear desde varios tipos
    let a = BigInt::from(123_u64);
    let b = BigInt::from("999999999999999999999");
    let c = "12345".parse::<BigInt>().unwrap();

    // Operaciones aritméticas
    let sum = &a + &b;           // Operaciones con referencias
    let product = a * b;          // Operaciones con propiedad
    let power = c.pow(10);        // Exponenciación

    // Mostrar en múltiples formatos
    println!("Decimal: {}", sum);
    println!("Binario: {:b}", sum);
    println!("Hex:     {:x}", sum);
    println!("Con separadores: {}", sum.to_string_with_separator(','));
}
```

## 📚 Ejemplos

### Aritmética Básica

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(100_u64);
let b = BigInt::from(42_u64);

// Todas las operaciones básicas
assert_eq!(a + b, BigInt::from(142_u64));
assert_eq!(a - b, BigInt::from(58_u64));
assert_eq!(a * b, BigInt::from(4200_u64));
assert_eq!(a / b, BigInt::from(2_u64));
assert_eq!(a % b, BigInt::from(16_u64));
```

### Operaciones Verificadas

```rust
use decimal_rs::bigint::BigInt;

let a = BigInt::from(10_u64);
let b = BigInt::from(20_u64);
let zero = BigInt::from(0_u64);

// Resta verificada
assert_eq!(a.checked_sub(&b), None);  // Sería negativo

// División verificada
assert_eq!(a.checked_div(&zero), None);  // División por cero
assert_eq!(a.checked_div(&b), Some(BigInt::from(0_u64)));
```

### Funciones Matemáticas

```rust
use decimal_rs::bigint::BigInt;

// Exponenciación
let base = BigInt::from(2_u64);
let result = base.pow(100);  // 2^100

// MCD y MCM
let a = BigInt::from(48_u64);
let b = BigInt::from(18_u64);
assert_eq!(a.gcd(&b), BigInt::from(6_u64));
assert_eq!(a.lcm(&b), BigInt::from(144_u64));

// Factorial
let n = BigInt::from(20_u64);
let fact = n.factorial();  // 20!
```

### Bases Numéricas y Formato

```rust
use decimal_rs::bigint::BigInt;

let num = BigInt::from(255_u64);

// Usando traits de formato
assert_eq!(format!("{:b}", num), "11111111");  // Binario
assert_eq!(format!("{:o}", num), "377");       // Octal
assert_eq!(format!("{:x}", num), "ff");        // Hex (minúsculas)
assert_eq!(format!("{:X}", num), "FF");        // Hex (mayúsculas)

// Conversión de base personalizada (2-36)
assert_eq!(num.to_string_radix(2), "11111111");
assert_eq!(num.to_string_radix(16), "ff");
assert_eq!(num.to_string_radix(36), "73");

// Separadores de miles
let big = BigInt::from("1234567890");
assert_eq!(big.to_string_with_separator(','), "1,234,567,890");

// Notación científica
let huge = BigInt::from(123000_u64);
assert_eq!(huge.to_scientific_notation(), "1.23e5");
```

## 🎯 Casos de Uso

- **Cálculos financieros**: Aritmética decimal exacta sin errores de redondeo
- **Criptografía**: Operaciones con números grandes para generación de claves
- **Computación científica**: Simulaciones numéricas de precisión arbitraria
- **Teoría de números**: Algoritmos de MCD, MCM, factorización prima
- **Combinatoria**: Factorial, coeficientes binomiales para entradas grandes

## 🧪 Pruebas

Cobertura de pruebas completa:

```bash
cargo test
```

**Estadísticas de Pruebas:**
- 92 pruebas unitarias
- 69 pruebas de documentación
- **161 pruebas totales** - todas pasan ✅
- 100% de documentación de API pública

## 📈 Rendimiento

Optimizaciones incluidas:

- **Eliminación de ceros iniciales O(n)** (mejorado desde O(n²))
- **Operaciones vectoriales minimizadas** (1 inversión en lugar de 3)
- **Exponenciación por cuadrado** para operaciones O(log n)
- **Búferes preasignados** para reducir asignaciones de heap
- **Operaciones con referencias** para evitar clonaciones innecesarias

## 🗺️ Hoja de Ruta

### Implementado ✅
- [x] Aritmética completa de BigInt (Add, Sub, Mul, Div, Rem)
- [x] Operaciones verificadas y saturadas
- [x] Funciones matemáticas (pow, gcd, lcm, factorial)
- [x] Múltiples formatos de visualización y bases
- [x] Conversiones de tipo completas
- [x] Operaciones con referencias para eficiencia
- [x] Traits Zero y One

### Planificado 🚧
- [ ] Implementación completa del tipo Decimal
- [ ] Soporte de serialización Serde
- [ ] Algoritmos avanzados (multiplicación Karatsuba, FFT)
- [ ] Suite de benchmarks
- [ ] Soporte WASM
- [ ] Compatibilidad No-std

## 📖 Documentación

Generar la documentación completa:

```bash
cargo doc --open
```

Todas las APIs públicas están documentadas con ejemplos y pruebas de documentación.

## 🤝 Contribución

¡Las contribuciones son bienvenidas! Por favor, siéntase libre de enviar issues o pull requests.

### Desarrollo

```bash
# Ejecutar pruebas
cargo test

# Ejecutar pruebas con salida
cargo test -- --nocapture

# Verificar formato
cargo fmt --check

# Ejecutar clippy
cargo clippy
```

## 📄 Licencia

Este proyecto es de código abierto y está disponible bajo la Licencia MIT.

## 🙏 Agradecimientos

Construido con el poderoso sistema de tipos de Rust y abstracciones de coste cero.

---

**Estado:** Listo para producción para aritmética de enteros de precisión arbitraria
**Versión Actual:** 0.1.0
**Versión de Rust:** 1.70+
