


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]



    fn test_round_basic() {
        assert_eq!(3.7_f64.round() as i64, 4);
        assert_eq!(3.2_f64.round() as i64, 3);
        assert_eq!((-3.7_f64).round() as i64, -4);
    }

    #[test]


    fn test_round_decimals() {
        let n = 2.71828_f64;
        let decimals = 2;
        let factor = 10_f64.powi(decimals);
        let result = (n * factor).round() / factor;
        assert!((result - 2.72).abs() < 0.001);
    }