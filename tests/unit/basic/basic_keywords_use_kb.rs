


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;
use rhai::Engine;



    #[test]


    fn test_use_kb_syntax() {
        let mut engine = Engine::new();


        assert!(engine
            .register_custom_syntax(&["USE_KB", "$expr$"], true, |_, _| Ok(Dynamic::UNIT))
            .is_ok());
    }