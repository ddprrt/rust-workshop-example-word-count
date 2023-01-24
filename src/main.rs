use std::env;

use wordcount::count_files;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{}", count_files(args).expect("Error loading file"));
}
