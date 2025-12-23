


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]



    fn test_int() {
        assert_eq!(3.9_f64.trunc() as i64, 3);
        assert_eq!((-3.9_f64).trunc() as i64, -3);
    }

    #[test]


    fn test_floor_ceil() {
        assert_eq!(3.7_f64.floor() as i64, 3);
        assert_eq!(3.2_f64.ceil() as i64, 4);
    }

    #[test]


    fn test_minmax() {
        assert_eq!(10_i64.max(5), 10);
        assert_eq!(10_i64.min(5), 5);
    }

    #[test]


    fn test_mod() {
        assert_eq!(17 % 5, 2);
    }

    #[test]


    fn test_sgn() {
        assert_eq!((-5_i64).signum(), -1);
        assert_eq!(5_i64.signum(), 1);
        assert_eq!(0_i64.signum(), 0);
    }

    #[test]


    fn test_sqrt() {
        assert!((16_f64.sqrt() - 4.0).abs() < 0.0001);
    }

    #[test]


    fn test_pow() {
        assert!((2_f64.powf(8.0) - 256.0).abs() < 0.0001);
    }