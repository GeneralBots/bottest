


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_chunk_creation() {
        let processor = DocumentProcessor::default();
        let text = "This is a test document with some content that needs to be chunked properly. "
            .repeat(20);
        let chunks = processor.create_chunks(&text, Path::new("test.txt"));


        assert!(!chunks.is_empty());


        for chunk in &chunks {
            assert!(chunk.content.len() <= processor.chunk_size);
        }


        if chunks.len() > 1 {
            let first_end = &chunks[0].content[chunks[0].content.len().saturating_sub(100)..];
            let second_start = &chunks[1].content[..100.min(chunks[1].content.len())];


            assert!(first_end.chars().any(|c| second_start.contains(c)));
        }
    }

    #[test]


    fn test_format_detection() {
        assert_eq!(
            DocumentFormat::from_extension(Path::new("test.pdf")),
            Some(DocumentFormat::PDF)
        );
        assert_eq!(
            DocumentFormat::from_extension(Path::new("test.docx")),
            Some(DocumentFormat::DOCX)
        );
        assert_eq!(
            DocumentFormat::from_extension(Path::new("test.txt")),
            Some(DocumentFormat::TXT)
        );
        assert_eq!(
            DocumentFormat::from_extension(Path::new("test.md")),
            Some(DocumentFormat::MD)
        );
        assert_eq!(
            DocumentFormat::from_extension(Path::new("test.unknown")),
            None
        );
    }

    #[test]


    fn test_text_cleaning() {
        let processor = DocumentProcessor::default();
        let dirty_text = "  This   is\n\n\na    test\r\nwith  multiple    spaces  ";
        let cleaned = processor.clean_text(dirty_text);
        assert_eq!(cleaned, "This is a test with multiple spaces");
    }