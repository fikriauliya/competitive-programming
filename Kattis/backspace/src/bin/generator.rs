use std::io;
use std::str;
use std::io::Write;
use rand::Rng;
use rand::seq::SliceRandom;
use std::ops::Range;

pub fn generate() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let n = 4;
    writeln!(out, "{}", n);

    let mut rng = rand::thread_rng();
    for _ in 0..n {
        for _ in 0..(10 as i32).pow(6) {
            let mut chars: Vec<u8>= ('a' as u8..'z' as u8).collect();
            chars.append(&mut vec!['[' as u8, ']' as u8, '<' as u8]);
            let c = chars.choose(&mut rng);
            write!(out, "{}", *c.unwrap() as char);
        }
        writeln!(out, "");
    }
}

pub fn main() {
    generate();
}
