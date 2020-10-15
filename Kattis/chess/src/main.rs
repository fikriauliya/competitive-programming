use std::io;
use std::str;
use std::convert::TryFrom;
use std::char;
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

fn from_char(c: char) -> i32 {
    c as i32 - 'A' as i32 + 1
}

fn to_char(i: i32) -> char {
    char::from_u32(i as u32 + 'A' as u32 - 1).unwrap()
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<i32>().unwrap();
    let movements = vec![(-1, -1), (1, 1), (1, -1), (-1, 1)];
    for i in 0..n {
        let sx = from_char(scan.token::<char>().unwrap());
        let sy = scan.token::<i32>().unwrap();
        let dx = from_char(scan.token::<char>().unwrap());
        let dy = scan.token::<i32>().unwrap();

        let mut found = false;
        if (sx, sy) == (dx, dy) {
            write!(out, "0 {} ", to_char(sx));
            writeln!(out, "{}", sy);
            found = true;
        } else {
            'outer: for movement in &movements {
                let (deltax, deltay) = (dx - sx, dy - sy);
                if deltax.abs() == deltay.abs() {
                    write!(out, "1 {} ", to_char(sx));
                    write!(out, "{} ", sy);
                    write!(out, "{} ", to_char(dx));
                    writeln!(out, "{}", dy);
                    found = true;
                    break;
                }

                for j in 1i32..8i32 {
                    let (tx, ty) = (sx + movement.0 * j, sy + movement.1 * j);
                    if tx < 1 || tx > 8 { continue; };
                    if ty < 1 || ty > 8 { continue; };

                    let (deltax, deltay) = (dx - tx, dy - ty);
                    if deltax.abs() == deltay.abs() {
                        write!(out, "2 {} ", to_char(sx));
                        write!(out, "{} ", sy);
                        write!(out, "{} ", to_char(tx));
                        write!(out, "{} ", ty);
                        write!(out, "{} ", to_char(dx));
                        writeln!(out, "{}", dy);
                        found = true;
                        break 'outer;
                    }
                }
            }
        }
        if !found {
            writeln!(out, "Impossible");
        }
    }

}
