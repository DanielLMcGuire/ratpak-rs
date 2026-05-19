# ratpak-rs

Safe Rust wrapper for Microsoft Ratpack arbitrary-precision rational arithmetic engine.

## Building

Requires a C++ compiler (for the underlying engine) and the git submodules initialized.

```bash
git submodule update --init --recursive
cargo build
```

To run the example binaries:

```bash
cargo run --example calc_example
cargo run --example rough_edges
```

To integrate into your own project:

```toml
[dependencies]
ratpak-rs = { git = "https://github.com/DanielLMcGuire/ratpak-rs.git" }
```

## Initialization

Call this at startup to set constants before doing any math:

```rust
use ratpak_rs::initialize_engine;

initialize_engine(10, 32);
```

| Parameter   | Meaning                                                               |
|-------------|-----------------------------------------------------------------------|
| `radix`     | Display base. `10` = decimal, `16` = hex, `2` = binary.               |
| `precision` | Significant digits to maintain. `32` is what Windows Calculator uses. |

This pre-computes all internal constants (π, e, ln 2, ln 10, angle conversion factors) at the requested precision. It is cheap to call again if you need to switch radix.

## Usage

```rust
use ratpak_rs::{Rational, RationalMath, AngleType, NumberFormat};
```

### Constructing a Rational

```rust
// From an i32 or u32
let a = Rational::from_i32(42);
let b = Rational::from_u32(100);

// For decimals, use division to maintain exactness
let tenth = Rational::from_i32(1).checked_div(&Rational::from_i32(10))?;

// Standard cloning
let c = a.clone();
```

### Arithmetic operators

Standard operators are implemented for `&Rational`. These will panic on errors (like division by zero). For safe handling, use the `checked_*` methods.

```rust
let x = &frac(1, 10)? + &frac(2, 10)?; // 0.3, exactly
let y = &frac(1, 3)?  * &Rational::from_i32(3); // 1, exactly

assert_eq!(x, frac(3, 10)?); // true
```

### Math functions (RationalMath)

The following high-precision transcendental functions are available:

```rust
RationalMath::exp(&x)?;
RationalMath::log(&x)?; // Natural log

// Trig - AngleType::Radians / Degrees / Gradians
RationalMath::sin(&x, AngleType::Radians)?;
RationalMath::cos(&x, AngleType::Radians)?;
```

### Converting to a string

```rust
// to_formatted_string(radix, format, precision)
let s = x.to_formatted_string(10, NumberFormat::Float, 32)?;

// The Display trait uses default RATIONAL_BASE and RATIONAL_PRECISION:
println!("{}", x); 

// NumberFormat options:
//   NumberFormat::Float       (decimal or exponential as needed)
//   NumberFormat::Scientific  (always exponential)
//   NumberFormat::Engineering (exponent always a multiple of 3)
```

### Comparison

`Rational` implements `PartialEq`, `Eq`, and `PartialOrd`.

```rust
if x == y { /* ... */ }
if x <  y { /* ... */ }
```

## Example

```rust
use ratpak_rs::{initialize_engine, Rational, RationalMath, NumberFormat, Result};

fn frac(p: i32, q: i32) -> Result<Rational> {
    Rational::from_i32(p).checked_div(&Rational::from_i32(q))
}

fn main() -> Result<()> {
    initialize_engine(10, 32);

    let a = frac(1, 10)?;
    let b = frac(2, 10)?;
    let c = frac(3, 10)?;

    println!("0.1 + 0.2 == 0.3: {}", &a + &b == c);

    let pi_approx = frac(355, 113)?;
    println!("Pi approx: {}", pi_approx.to_formatted_string(10, NumberFormat::Float, 32)?);

    let e = RationalMath::exp(&Rational::from_i32(1))?;
    println!("e: {}", e);

    Ok(())
}
```

## Licence

MIT, © Daniel McGuire [See license](LICENSE).

Code used: CEngine, ratpack/ratpak - MIT, © Microsoft Corporation.
