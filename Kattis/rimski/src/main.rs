use std::io;
use std::str;
use std::convert::TryFrom;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    // loop {
        let mut num = scan.line();
        let mut num = if let None = num { return }
        else { num.unwrap() };

        loop {
            let start_num = num.clone();
            //VI -> IV
            if num.ends_with("VI") {
                num = num.replace("VI", "IV");
            }
            //LX -> XL
            if !num.starts_with("LXX") && !num.starts_with("LXXX") {
                num = num.replace("LX", "XL");
            }
            num = match &num[..] {
                "LIX" => "XLI".to_string(),
                "XI" => "IX".to_string(),
                "XXI" => "XIX".to_string(),
                "XXXI" => "XXIX".to_string(),
                // "LXI" => "LIX",
                "LXXI" => "LXIX".to_string(),
                "LXXXI" => "LXXIX".to_string(),
                num => num.to_string()
            };

            dbg!(&start_num, &num);
            if start_num == num { break }
        }
        writeln!(out, "{}", num);
        // out.flush();
    // }
}
