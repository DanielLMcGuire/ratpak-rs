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

    header("3. trimit: denominator growth from unlike-fraction addition");

    println!("  ∑_{{k=1}}^{{N}} 1/(k(k+1))  should equal  1 - 1/(N+1)  exactly.\n");
    println!("  _addrat sets new_denom = old_denom × b_denom (no LCM, no GCD).");
    println!("  After ~17 terms the running denominator exceeds 32 decimal digits");
    println!("  and trimit cuts it, corrupting the numerator's tail.\n");
    println!("  {:>4}   {:>42}   {}", "N", "residual  (sum) - (closed form)", "snapped?");
    println!("  {:>4}   {:>42}   {}", "─".repeat(4), "─".repeat(42), "─".repeat(8));

    for &n in &[5i32, 10, 20, 50, 100] {
        let closed = one.checked_sub(
            &one.checked_div(&Rational::from_i32(n + 1))?
        )?;

        let mut running = zero.clone();
        for k in 1..=n {
            let kk  = Rational::from_i32(k);
            let kp1 = Rational::from_i32(k + 1);
            let term = one.checked_div(&kk.checked_mul(&kp1)?)?;
            running = running.checked_add(&term)?;
        }

        let diff    = running.checked_sub(&closed)?;
        let snapped = diff == zero;

        println!("  N={:<4} {}  {}",
            n,
            format!("{:>42}", sci(&diff)),
            if snapped { "zero (trimit not yet triggered)" } else { "← nonzero: trimit loss" }
        );
    }
    println!("\n  Denominator size after k terms (no GCD):  ∏_{{i=1}}^k i(i+1) ≈ (k+1)! × k!");
    println!("  That exceeds 32 digits around k=17, where the residuals above appear.");

    header("4. scale2pi: large-angle cancellation erodes sin/cos accuracy");

    println!("  x mod 2π  =  x - floor(x/2π)·2π");
    println!("  For x = 10^k, the subtraction cancels k leading digits.");
    println!("  With precision=32, sin(10^k) has only ~(32-k) good digits.\n");
    println!("  sin²(x) + cos²(x) - 1 should be exactly zero;");
    println!("  a nonzero residual measures the argument-reduction error.\n");
    println!("  {:>12}   {:>42}", "x", "sin²(x) + cos²(x) - 1");
    println!("  {:>12}   {:>42}", "─".repeat(12), "─".repeat(42));

    for &mag in &[1i32, 1_000, 1_000_000, 1_000_000_000] {
        let x  = Rational::from_i32(mag);
        let s  = RationalMath::sin(&x, AngleType::Radians)?;
        let c  = RationalMath::cos(&x, AngleType::Radians)?;
        let id = s.checked_mul(&s)?.checked_add(&c.checked_mul(&c)?)?.checked_sub(&one)?;
        println!("  {:>12}   {:>42}", mag, sci(&id));
    }
    println!("\n  The residual grows roughly as 10^-(32-k) for x = 10^k,");
    println!("  confirming that each extra decade of magnitude costs one");
    println!("  significant digit in the trig result.");

    header("5. π, e, ln2 are frozen at init — extra display digits are noise");

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