


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]



    fn test_sin() {
        assert!((0.0_f64.sin() - 0.0).abs() < 0.0001);
    }

    #[test]


    fn test_cos() {
        assert!((0.0_f64.cos() - 1.0).abs() < 0.0001);
    }

    #[test]


    fn test_log() {
        assert!((100.0_f64.log10() - 2.0).abs() < 0.0001);
    }

    #[test]


    fn test_exp() {
        assert!((0.0_f64.exp() - 1.0).abs() < 0.0001);
    }

    #[test]


    fn test_pi() {
        assert!(std::f64::consts::PI > 3.14);
        assert!(std::f64::consts::PI < 3.15);
    }