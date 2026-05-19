use ratpak_rs::{initialize_engine, AngleType, NumberFormat, Rational, RationalMath, Result};

fn frac(p: i32, q: i32) -> Result<Rational> {
    Rational::from_i32(p).checked_div(&Rational::from_i32(q))
}

fn sci(r: &Rational) -> String {
    r.to_formatted_string(10, NumberFormat::Scientific, 32)
        .unwrap_or_else(|_| "<fmt error>".into())
}

fn header(title: &str) {
    println!("\n{}", "─".repeat(64));
    println!("  {}", title);
    println!("{}", "─".repeat(64));
}

fn main() -> Result<()> {
    initialize_engine(10, 32);
    let one  = Rational::from_i32(1);
    let ten  = Rational::from_i32(10);
    let zero = Rational::from_i32(0);

    header("1. _snaprat zeros real values below radix^(-32) — no guard band k");

    let mut tiny = one.clone();
    for _ in 0..33 { tiny = tiny.checked_div(&ten)?; }   // 1e-33

    let mut almost_one = one.clone();
    almost_one = almost_one.checked_add(&tiny)?;
    let residual = almost_one.checked_sub(&one)?;
    println!("(1 + 1e-33) - 1  = {}", sci(&residual));
    println!("  operands are O(1) -> threshold = 1×10^-32");
    println!("  result 1e-33 < threshold -> _snaprat zeroes it");
    println!("  a real value is silently discarded");

    let sin_pi = RationalMath::sin(&frac(355, 113)?, AngleType::Radians)?;
    println!("\nsin(355/113)     = {}", sci(&sin_pi));
    println!("  operand ≈ 3.14  →  threshold ≈ 3.14e-32");
    println!("  result -2.67e-7 >> threshold  →  survives");

    header("2. 1 - cos(x) cancellation: snapped for x ≤ ~1.4e-16");

    println!("  Manual: cos(x) ≈ 1 - x²/2 (first correction term)\n");
    println!("  {:>6}   {:>38}   {}", "x", "1 - cos(x)  [= x²/2 ideally]", "snapped?");
    println!("  {:>6}   {:>38}   {}", "─".repeat(6), "─".repeat(38), "─".repeat(8));

    for exp in [4u32, 8, 12, 16, 18, 20] {
        let mut angle = one.clone();
        for _ in 0..exp { angle = angle.checked_div(&ten)?; }

        let x2         = angle.checked_mul(&angle)?;
        let correction = x2.checked_div(&Rational::from_i32(2))?;
        let cos_approx = one.checked_sub(&correction)?;
        let result     = one.checked_sub(&cos_approx)?;
        let snapped    = result == zero;

        println!("  1e-{:<4}  {}  {}",
            exp, format!("{:>38}", sci(&result)), if snapped { "YES ←" } else { "no" });
    }
    println!("\n  Below x ≈ 1.4e-16 the result x²/2 < 1e-32 and is zeroed.");
    println!("  No numerically stable alternative (e.g. 2sin²(x/2)) exists in ratpak.");

    header("3. π, e, ln2 are frozen at init — extra display digits are noise");

    let e = RationalMath::exp(&one)?;
    println!("e at 32 sig digits: {}",
        e.to_formatted_string(10, NumberFormat::Float, 32)?);
    println!("e at 36 sig digits: {}",
        e.to_formatted_string(10, NumberFormat::Float, 36)?);
    println!("  Known value:  2.71828182845904523536028747135266...");
    println!("  Digits beyond 32 are whatever the stored PRAT's low mantissa limbs");
    println!("  happen to contain, they are not recomputed from a longer series.");

    Ok(())
}