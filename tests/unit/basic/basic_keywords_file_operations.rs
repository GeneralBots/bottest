


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_dynamic_to_json() {
        let dynamic = Dynamic::from("hello");
        let json = dynamic_to_json(&dynamic);
        assert_eq!(json, Value::String("hello".to_string()));
    }

    #[test]


    fn test_dynamic_to_file_data() {
        let dynamic = Dynamic::from("test content");
        let file_data = dynamic_to_file_data(&dynamic);
        assert_eq!(file_data.filename, "file");
        assert!(!file_data.content.is_empty());
    }