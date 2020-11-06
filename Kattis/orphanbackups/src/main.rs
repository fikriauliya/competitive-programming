use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashSet;
use std::collections::HashMap;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut images = HashSet::new();
    let mut files: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let line = scan.line().unwrap();
        if line.len() == 0 { break; }
        images.insert(line);
    }
    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let mut file = "".to_string();
        let mut ctr = 0;
        for c in line.chars().rev() {
            if ctr >= 2 {
                file.push(c);
            }
            if c == '_' { ctr += 1 }
        }
        let file: String = file.chars().rev().collect();
        match files.get_mut(&file) {
            Some(l) => l.push(line),
            None => { files.insert(file, vec![line]); }
        }
    }
    let file_keys: HashSet<String> = files.keys().cloned().collect();
    let mut files_images: Vec<&String> = file_keys.difference(&images).collect();
    let mut images_files: Vec<&String> = images.difference(&file_keys).collect();
    files_images.sort();
    images_files.sort();
    if files_images.is_empty() && images_files.is_empty() {
        writeln!(out, "{}", "No mismatches.");
    } else {
        for file in files_images {
            let mut file_names = files.get_mut(file).unwrap();
            file_names.sort();
            for file_name in file_names {
                writeln!(out, "F {}", file_name);
            }
        }
        for image in images_files {
            writeln!(out, "I {}", images.get(image).unwrap());
        }
    }
}
