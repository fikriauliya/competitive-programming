use std::io;
use std::str;

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            input.pop();
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.pop();
        input
    }
}

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

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

use std::collections::LinkedList;
fn solve<R: io::BufRead, W: io::Write>(scan: &mut Scanner<R>, out: &mut W) {
    let n = scan.token::<u32>();
    for i in 0..n {
        let mut left:LinkedList<char> = LinkedList::new();
        let mut right:LinkedList<char> = LinkedList::new();
        let mut cur = &mut right;

        let inp = scan.line();
        let inp: Vec<char>= inp.chars().collect();

        // let mut prevs = Vec::with_capacity(inp.len());
        // prevs.resize(inp.len(), 0);
        // let mut last_char = 0;

        // for i in 0..inp.len() {
        //     let c = inp[i];
        //     match c {
        //         '<' =>  {
        //             inp[last_char] = 'X';
        //             inp[i] = 'X';
        //             last_char = prevs[last_char];
        //         },
        //         ']'|'[' => (),
        //         _ => {
        //             prevs[i] = last_char;
        //             last_char = i;
        //         }
        //     };
        // };
        // dbg!(&inp);

        for c in inp {
            match c {
                '[' => {
                    left.append(&mut right);
                    right = left;
                    left = LinkedList::new();
                    cur = &mut left;
                },
                ']' => {
                    left.append(&mut right);
                    right = left;
                    left = LinkedList::new();
                    cur = &mut right;
                },
                // 'X' => (),
                '<' => {
                    cur.pop_back();
                },
                _ => {
                    cur.push_back(c)
                }
            }
        }
        left.append(&mut right);

        if left.len() == 0 {
            writeln!(out, "").unwrap();
        } else {
            for item in left {
                write!(out, "{}", item).unwrap();
            }
            writeln!(out, "").unwrap();
        }
        out.flush().unwrap();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out); 
}
