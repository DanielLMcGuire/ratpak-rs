#[cfg(test)]
mod tests {
    use ratpak_rs::*; 
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            initialize_engine(RATIONAL_BASE, RATIONAL_PRECISION);
        });
    }

    #[test]
    fn test_creation_and_display() {
        setup();
        let r = Rational::from_i32(-123);
        assert_eq!(r.to_string(), "-123");

        let r2 = Rational::from_u32(456);
        assert_eq!(r2.to_string(), "456");
    }

    #[test]
    fn test_conversions() {
        setup();
        let r = Rational::from_u32(100);
        assert_eq!(r.to_u64().unwrap(), 100u64);

        let neg = Rational::from_i32(-1);
        assert!(neg.to_u64().is_err(), "Negative number should not convert to u64");
    }

    #[test]
    fn test_arithmetic_operators() {
        setup();
        let a = Rational::from_i32(10);
        let b = Rational::from_i32(3);

        assert_eq!(&a + &b, Rational::from_i32(13));
        assert_eq!(&a - &b, Rational::from_i32(7));
        assert_eq!(&a * &b, Rational::from_i32(30));

        let div = &a / &b;
        assert!(div.to_string().starts_with("3.33333333"));
    }

    #[test]
    fn test_checked_arithmetic_errors() {
        setup();
        let a = Rational::from_i32(10);
        let zero = Rational::from_i32(0);

        let result = a.checked_div(&zero);
        assert!(result.is_err(), "Division by zero should return an error");
    }

    #[test]
    fn test_comparison() {
        setup();
        let a = Rational::from_i32(5);
        let b = Rational::from_i32(10);
        let a_clone = a.clone();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, a_clone);
        assert_ne!(a, b);
    }

    #[test]
    fn test_formatting_modes() {
        setup();
        let r = Rational::from_i32(123456);
        
        let sci = r.to_formatted_string(10, NumberFormat::Scientific, 5).unwrap();

        assert!(sci.contains('e') || sci.contains('E'));

        let hex = Rational::from_i32(255).to_formatted_string(16, NumberFormat::Float, 5).unwrap();
        assert_eq!(hex.to_uppercase(), "FF");
    }

    #[test]
    fn test_math_functions() {
        setup();
        
        let zero = Rational::from_i32(0);
        let sin_zero = RationalMath::sin(&zero, AngleType::Radians).unwrap();
        assert_eq!(sin_zero, Rational::from_i32(0));

        let exp_zero = RationalMath::exp(&zero).unwrap();
        assert_eq!(exp_zero, Rational::from_i32(1));

        let one = Rational::from_i32(1);
        let log_one = RationalMath::log(&one).unwrap();
        assert_eq!(log_one, Rational::from_i32(0));

        let neg = Rational::from_i32(-1);
        assert!(RationalMath::log(&neg).is_err());

        let zero = Rational::from_i32(0);
        let cos_zero = RationalMath::cos(&zero, AngleType::Radians).unwrap();
        assert_eq!(cos_zero, Rational::from_i32(1));
    }

    #[test]
    fn test_cloning_and_memory() {
        setup();
        let original = Rational::from_i32(42);
        {
            let cloned = original.clone();
            assert_eq!(original, cloned);
        }
        assert_eq!(original.to_string(), "42");
    }
}