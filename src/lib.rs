use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    iter::Sum,
};

use rayon::prelude::*;

#[derive(Default, PartialEq, Debug)]
pub struct FileInfo {
    pub bytes: usize,
    pub words: usize,
    pub lines: usize,
    pub chars: usize,
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>8} {:>8} {:>8} {:>8}",
            self.words, self.lines, self.chars, self.bytes
        )
    }
}

impl Sum for FileInfo {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = FileInfo::default();
        for item in iter {
            sum.bytes += item.bytes;
            sum.chars += item.chars;
            sum.lines += item.lines;
            sum.words += item.words;
        }

        sum
    }
}

fn open_and_count_file(path: String) -> Result<FileInfo, std::io::Error> {
    let file = File::open(path)?;
    let mut file = BufReader::new(file);
    count(&mut file)
}

pub fn count_files(list: Vec<String>) -> Result<FileInfo, std::io::Error> {
    let sum: FileInfo = list
        .into_par_iter()
        .map(open_and_count_file)
        .map(|el| el.unwrap_or_default())
        .sum();
    Ok(sum)
}

pub fn count(file: &mut impl BufRead) -> Result<FileInfo, std::io::Error> {
    let mut file_info = FileInfo::default();
    let mut buf = String::new();
    loop {
        let line_bytes = file.read_line(&mut buf)?;

        if line_bytes == 0 {
            break;
        }

        file_info.bytes += line_bytes;
        file_info.lines += 1;
        file_info.words += buf.split_whitespace().count();
        file_info.chars += buf.chars().count();
        buf.clear();
    }
    Ok(file_info)
}
