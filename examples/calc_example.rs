use ratpak_rs::{initialize_engine, AngleType, NumberFormat, Rational, RationalMath, Result};

/// Helper to create a rational number representing a fraction `p/q`.
fn frac(p: i32, q: i32) -> Result<Rational> {
    let numerator = Rational::from_i32(p);
    let denominator = Rational::from_i32(q);

    numerator.checked_div(&denominator)
}

fn main() -> Result<()> {
    // 1. Initialize the Ratpack engine
    println!("Initializing engine...");
    initialize_engine(10, 32);

    // 2. Exact Fraction Arithmetic
    println!("--- Basic Arithmetic ---");
    let tenth = frac(1, 10)?;
    let two_tenths = frac(2, 10)?;
    let three_tenths = frac(3, 10)?;

    let sum = &tenth + &two_tenths;

    println!("0.1        = {}", tenth);
    println!("0.2        = {}", two_tenths);
    println!("0.1 + 0.2  = {}", sum);
    println!("0.3        = {}", three_tenths);
    
    if sum == three_tenths {
        println!("0.1 + 0.2 == 0.3 : TRUE (Exact match!)\n");
    } else {
        println!("0.1 + 0.2 == 0.3 : FALSE\n");
    }

    // 3. Approximating Pi
    println!("--- Transcendental Math ---");
    let a = Rational::from_i32(355);
    let b = Rational::from_i32(113);
    let pi_approx = &a / &b;

    let formatted_pi = pi_approx.to_formatted_string(10, NumberFormat::Float, 32)?;
    println!("355 / 113          = {}", formatted_pi);

    // 4. Trigonometry
    let sin_pi = RationalMath::sin(&pi_approx, AngleType::Radians)?;
    
    let formatted_sin = sin_pi.to_formatted_string(10, NumberFormat::Scientific, 32)?;
    println!("sin(355/113)       = {}", formatted_sin);

    // 5. Exponents
    let one = Rational::from_i32(1);
    let e = RationalMath::exp(&one)?;
    println!("e (exp(1))         = {}", e);
 
    Ok(())
}