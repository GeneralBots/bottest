//! Unit tests migrated from src/basic/keywords/play.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_content_type_from_extension() {
        assert_eq!(ContentType::from_extension("mp4"), ContentType::Video);
        assert_eq!(ContentType::from_extension("MP3"), ContentType::Audio);
        assert_eq!(ContentType::from_extension("png"), ContentType::Image);
        assert_eq!(ContentType::from_extension("pdf"), ContentType::Pdf);
        assert_eq!(ContentType::from_extension("rs"), ContentType::Code);
        assert_eq!(
            ContentType::from_extension("pptx"),
            ContentType::Presentation
        );
        assert_eq!(
            ContentType::from_extension("xlsx"),
            ContentType::Spreadsheet
        );
        assert_eq!(ContentType::from_extension("md"), ContentType::Markdown);
    }

    #[test]

    
    fn test_content_type_from_mime() {
        assert_eq!(ContentType::from_mime("video/mp4"), ContentType::Video);
        assert_eq!(ContentType::from_mime("audio/mpeg"), ContentType::Audio);
        assert_eq!(ContentType::from_mime("image/png"), ContentType::Image);
        assert_eq!(ContentType::from_mime("application/pdf"), ContentType::Pdf);
    }

    #[test]

    
    fn test_play_options_from_string() {
        let opts = PlayOptions::from_string("autoplay,loop,muted");
        assert!(opts.autoplay);
        assert!(opts.loop_content);
        assert!(opts.muted);
        assert!(!opts.fullscreen);
        assert!(opts.controls);

        let opts = PlayOptions::from_string("fullscreen,nocontrols,start=10,end=60");
        assert!(opts.fullscreen);
        assert!(!opts.controls);
        assert_eq!(opts.start_time, Some(10.0));
        assert_eq!(opts.end_time, Some(60.0));

        let opts = PlayOptions::from_string("theme=dark,zoom=1.5,page=3");
        assert_eq!(opts.theme, Some("dark".to_string()));
        assert_eq!(opts.zoom, Some(1.5));
        assert_eq!(opts.page, Some(3));
    }

    #[test]

    
    fn test_detect_content_type() {
        assert_eq!(
            detect_content_type("https://youtube.com/watch?v=123"),
            ContentType::Video
        );
        assert_eq!(
            detect_content_type("https://example.com/video.mp4"),
            ContentType::Video
        );
        assert_eq!(
            detect_content_type("https://imgur.com/abc123"),
            ContentType::Image
        );
        assert_eq!(
            detect_content_type("presentation.pptx"),
            ContentType::Presentation
        );
        assert_eq!(detect_content_type("report.pdf"), ContentType::Pdf);
        assert_eq!(detect_content_type("main.rs"), ContentType::Code);
    }

    #[test]

    
    fn test_extract_title_from_source() {
        assert_eq!(extract_title_from_source("documents/report.pdf"), "report");
        assert_eq!(
            extract_title_from_source("https://example.com/video.mp4?token=abc"),
            "video"
        );
        assert_eq!(
            extract_title_from_source("presentation.pptx"),
            "presentation"
        );
    }

    #[test]

    
    fn test_player_component() {
        assert_eq!(ContentType::Video.player_component(), "video-player");
        assert_eq!(ContentType::Audio.player_component(), "audio-player");
        assert_eq!(ContentType::Image.player_component(), "image-viewer");
        assert_eq!(ContentType::Pdf.player_component(), "pdf-viewer");
        assert_eq!(ContentType::Code.player_component(), "code-viewer");
    }