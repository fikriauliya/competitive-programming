use std::io;
use std::str;
use std::convert::TryFrom;

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

    pub fn trim_newline(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
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

    pub fn line(&mut self) -> Option<String> {
        let mut input = String::new();
        let len = self.reader.read_line(&mut input).expect("Failed read");
        match len {
            0 => None,
            _ => {
                Self::trim_newline(&mut input);
                Some(input)
            }
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let (r, c) = (scan.token::<usize>().unwrap(), scan.token::<usize>().unwrap());
    let mut m = vec![vec![' '; c]; r];
    for i in 0..r {
        let line = scan.token::<String>().unwrap();
        for (j, ch) in line.char_indices() {
            m[i][j] = ch;
        }
    } 
    let (mut x, mut y) = (0i32, 0i32);
    let mut res = 0;
    loop {
        let (next_y, next_x) = match m[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] {
            'N' => (y - 1, x),
            'S' => (y + 1, x),
            'W' => (y, x - 1),
            'E' => (y, x + 1),
            'T' => {
                writeln!(out, "{}", res).unwrap();
                break;
            }
            _ => {
                writeln!(out, "Lost").unwrap();
                break;
            }
        };
        m[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()] = ' ';

        y = next_y;
        x = next_x;

        if match (y, x) {
            (-1, _) | (_, -1) => true, 
            (y, _) if y == r as i32 => true,
            (_, x) if x == c as i32 => true,
            _ => false
        } {
            writeln!(out, "Out").unwrap();
            break;
        }

        res += 1
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out); 
}
