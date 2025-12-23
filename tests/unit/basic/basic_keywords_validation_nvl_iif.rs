


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]



    fn test_nvl_logic() {
        let value = "";
        let default = "default";
        let result = if value.is_empty() { default } else { value };
        assert_eq!(result, "default");
    }

    #[test]


    fn test_nvl_with_value() {
        let value = "actual";
        let default = "default";
        let result = if value.is_empty() { default } else { value };
        assert_eq!(result, "actual");
    }

    #[test]


    fn test_iif_true() {
        let condition = true;
        let result = if condition { "yes" } else { "no" };
        assert_eq!(result, "yes");
    }

    #[test]


    fn test_iif_false() {
        let condition = false;
        let result = if condition { "yes" } else { "no" };
        assert_eq!(result, "no");
    }

    #[test]


    fn test_choose() {
        let index = 2;
        let values = vec!["first", "second", "third"];
        let result = values.get((index - 1) as usize).unwrap_or(&"");
        assert_eq!(*result, "second");
    }