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

fn time_to_sec(time: &str) -> i32 {
    let mut sp = time.split(":");
    let h = sp.next().unwrap().parse::<usize>().unwrap();
    let m = sp.next().unwrap().parse::<usize>().unwrap();
    let s = sp.next().unwrap().parse::<usize>().unwrap();
    (h * 3600 + m * 60 + s) as i32
}

fn speed_to_km_per_sec(km_per_hour: usize) -> f64 {
    km_per_hour as f64 / 3600f64
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut speed = 0f64;
    let mut prev_time = 0;
    let mut distance = 0f64;
    loop {
        let line = scan.line();
        if let None = line { break }
        let line = line.unwrap();
        let sp: Vec<&str> = line.split(" ").collect();
        match sp.len() {
            1 => {
                let new_time = time_to_sec(sp[0]);
                let delta_time = new_time - prev_time;
                let delta_distance = delta_time as f64 * speed;
                println!("{} {:.2} km", sp[0], distance+delta_distance);
            },
            2 => {
                let new_time = time_to_sec(sp[0]);
                let delta_time = new_time - prev_time;
                let delta_distance = delta_time as f64 * speed;
                distance += delta_distance;
                speed = speed_to_km_per_sec(sp[1].parse::<usize>().unwrap());
                prev_time = new_time;
            }
            _ => ()
        }
    }
}
