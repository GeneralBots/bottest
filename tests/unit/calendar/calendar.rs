


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_event_to_ical_roundtrip() {
        let event = CalendarEvent {
            id: Uuid::new_v4(),
            title: "Test Meeting".to_string(),
            description: Some("A test meeting".to_string()),
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(1),
            location: Some("Room 101".to_string()),
            attendees: vec!["user@example.com".to_string()],
            organizer: "organizer@example.com".to_string(),
            reminder_minutes: Some(15),
            recurrence: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let ical = event.to_ical();
        assert_eq!(ical.get_summary(), Some("Test Meeting"));
        assert_eq!(ical.get_location(), Some("Room 101"));
    }

    #[test]


    fn test_export_import_ical() {
        let mut engine = CalendarEngine::new();
        engine.create_event(CalendarEventInput {
            title: "Event 1".to_string(),
            description: None,
            start_time: Utc::now(),
            end_time: Utc::now() + chrono::Duration::hours(1),
            location: None,
            attendees: vec![],
            organizer: "test@example.com".to_string(),
            reminder_minutes: None,
            recurrence: None,
        });

        let ical = engine.export_ical("Test Calendar");
        assert!(ical.contains("BEGIN:VCALENDAR"));
        assert!(ical.contains("Event 1"));
    }