use std::io::Cursor;
use wordcount::{count, FileInfo};

#[test]
fn test_count_fn() {
    let text = "The minstrel in the gallery looked down upon the smiling faces";
    let mut text = Cursor::new(text);
    let result = count(&mut text);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        FileInfo {
            words: 11,
            lines: 1,
            bytes: 62,
            chars: 62
        }
    );
}

#[test]
fn test_multi_line() {
    let text = r#"The minstrel in the gallery
looked down upon the smiling faces
He met the gazes, observed the spaces
Between the old men's cackle

He brewed a song of love and hatred
Oblique suggestions and he waited
He polarized the pumpkin-eaters
Static-humming, panel-beaters
Freshly day, glowed factory cheaters
Salaried and collar-scrubbing"#;
    let mut text = Cursor::new(text);
    let result = count(&mut text);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        FileInfo {
            words: 50,
            lines: 11,
            bytes: 329,
            chars: 329
        }
    );
}

#[test]
fn test_chars_vs_bytes() {
    let text = "y̆";
    let mut text = Cursor::new(text);
    let result = count(&mut text);
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        FileInfo {
            words: 1,
            lines: 1,
            bytes: 3,
            chars: 2
        }
    );
}
