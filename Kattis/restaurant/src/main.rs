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
    let mut first_time = true;
    loop {
        let n = scan.token::<usize>().unwrap();
        let mut a = 0;
        let mut b = 0;
        if n == 0 { break; }
        if !first_time { println!(); }
        first_time = false;
        for _ in 0..n {
            let command = scan.token::<String>().unwrap();
            let mut count = scan.token::<usize>().unwrap();
            match &command[..] {
                "DROP" => { 
                    b += count;
                    println!("DROP 2 {}", count);

                },
                "TAKE" => {
                    if a >= count {
                        println!("TAKE 1 {}", count);
                        a -= count;
                    } else {
                        if a > 0 {
                            println!("TAKE 1 {}", a);
                            count -= a;
                            a = 0;
                        }
                        println!("MOVE 2->1 {}", b);
                        a += b;
                        b = 0;
                        println!("TAKE 1 {}", count);
                        a -= count;
                    }
                }
                _ => panic!()
            }
        }
    }
}
