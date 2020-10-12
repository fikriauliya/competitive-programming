use std::io;
use std::str;

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            self.buf_str.clear();
            let len = self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            if len == 0 { 
                return None;
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }

    pub fn line<T: str::FromStr>(&mut self) -> Option<String> {
        let mut input = String::new();
        let len = self.reader.read_line(&mut input).expect("Failed read");
        match len {
            0 => None,
            _ => Some(input)
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());
        if let (Some(a), Some(b)) = (a, b) {
            let diff = (a - b).abs();
            writeln!(out, "{}", diff).unwrap();
        } else {
            return;
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out); 
}
