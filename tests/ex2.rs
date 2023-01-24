/*
use std::io::Cursor;
use wordcount::{count, FileInfo};


#[test]
fn test_sum() {
    let one = r#"The minstrel in the gallery
looked down upon the smiling faces
He met the gazes, observed the spaces
Between the old men's cackle"#;
    let two = r#"
He brewed a song of love and hatred
Oblique suggestions and he waited
He polarized the pumpkin-eaters
Static-humming, panel-beaters
Freshly day, glowed factory cheaters
Salaried and collar-scrubbing"#;
    let mut verses = vec![Cursor::new(one), Cursor::new(two)];

    let result: FileInfo = verses
        .iter_mut()
        .map(|cursor| count(cursor))
        .map(|el| el.unwrap_or_default())
        .sum();

    assert_eq!(
        result,
        FileInfo {
            words: 50,
            lines: 11,
            bytes: 328,
            chars: 328
        }
    );
}

#[test]
fn test_display_trait() {
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
        result.unwrap().to_string(),
        "      50       11      329      329".to_string()
    );
}
*/
