


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_degrees_to_compass() {
        assert_eq!(degrees_to_compass(0.0), "N");
        assert_eq!(degrees_to_compass(45.0), "NE");
        assert_eq!(degrees_to_compass(90.0), "E");
        assert_eq!(degrees_to_compass(180.0), "S");
        assert_eq!(degrees_to_compass(270.0), "W");
        assert_eq!(degrees_to_compass(315.0), "NW");
    }

    #[test]


    fn test_format_weather_response() {
        let weather = WeatherData {
            location: "London".to_string(),
            temperature: 15.0,
            temperature_unit: "°C".to_string(),
            description: "Partly cloudy".to_string(),
            humidity: 65,
            wind_speed: 3.5,
            wind_direction: "NE".to_string(),
            feels_like: 14.0,
            pressure: 1013,
            visibility: 10.0,
            uv_index: Some(3.0),
            forecast: Vec::new(),
        };

        let response = format_weather_response(&weather);
        assert!(response.contains("London"));
        assert!(response.contains("15.0"));
        assert!(response.contains("Partly cloudy"));
    }