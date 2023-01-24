use std::io::Cursor;

use wordcount::*;

fn main() {
    count(&mut Cursor::new("Hello"));
}
