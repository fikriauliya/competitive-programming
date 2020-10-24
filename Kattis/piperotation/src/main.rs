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

#[derive(Debug, Clone)]
enum Connection { Nothing, Top, Left, TopLeft }

fn set(m: &mut Vec<Vec<Connection>>, y: usize, z: usize, conn: Connection) {
    match (&m[y][z], conn) {
        (Connection::Nothing, conn) => m[y][z] = conn,
        (Connection::Top, Connection::Left) => m[y][z] = Connection::TopLeft,
        _ => ()
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, _: &mut W) {
    let r = scan.token::<usize>().unwrap();
    let c = scan.token::<usize>().unwrap();
    let mut conn = vec![vec![Connection::Nothing; c+2]; r+2];
    let mut raw = vec![vec![' '; c+2]; r+2];

    let mut res = true;
    'case: for i in 1..(r+1) {
        let line = scan.line().unwrap();
        let line = line.chars();
        for (j, c) in line.enumerate() {
            let j = j + 1;
            raw[i][j] = c;
            let possible = match (&conn[i][j], c) {
                (Connection::Nothing, 'A') => true,
                (Connection::TopLeft, 'C') => true,
                (Connection::TopLeft, 'D') | (Connection::Nothing, 'C') => {
                    set(&mut conn, i + 1, j, Connection::Top);
                    set(&mut conn, i, j + 1, Connection::Left);
                    true
                },
                (Connection::Left, 'C') => { set(&mut conn, i + 1, j, Connection::Top); true },
                (Connection::Top, 'C') => { set(&mut conn, i, j + 1, Connection::Left); true },
                (Connection::Top, 'B') => { set(&mut conn, i + 1, j, Connection::Top); true }
                (Connection::Left, 'B') => { set(&mut conn, i, j + 1, Connection::Left); true }
                _ => false,
            };
            if !possible { res = false; break 'case; }
        }
    }
    for i in 1..(r + 1) {
        match conn[i][c+1] {
            Connection::Nothing => continue,
            _ => { res = false; break }
        }
    }
    for j in 1..(c + 1) {
        match conn[r+1][j] {
            Connection::Nothing => continue,
            _ => { res = false; break }
        }
    }
    for i in 1..(r + 1) {
        for j in 1..(c + 1) {
            match (raw[i][j], (i+j)%4) {
                ('A', _) => print!(" "),
                ('C', 0) => print!("╔"),
                ('C', 1) => print!("╝"),
                ('C', 2) => print!("╗"),
                ('C', 3) => print!("╚"),
                ('B', 0) | ('B', 1) => print!("═"),
                ('B', 2) | ('B', 3) => print!("║"),
                ('D', _) => print!("╬"),
                _ => print!("❌"),
            }
        }
        println!();
    }

    println!();
    println!();
    for i in 1..(r + 1) {
        for j in 1..(c + 1) {
            match (&conn[i][j], raw[i][j]) {
                (Connection::Nothing, 'A') => print!(" "),
                (Connection::Nothing, 'C') => print!("╔"),
                (Connection::TopLeft, 'C') => print!("╝"),
                (Connection::Left, 'C') => print!("╗"),
                (Connection::Top, 'C') => print!("╚"),
                (Connection::Left, 'B') => print!("═"),
                (Connection::Top, 'B') => print!("║"),
                (Connection::TopLeft, 'D') => print!("╬"),
                (_, _) => print!("❌")
            }
        }
        println!();
    }
    if res { println!("Possible"); }
    else { println!("Impossible"); }
}
