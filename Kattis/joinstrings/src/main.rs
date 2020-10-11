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
// fn problem() {
//     let mut items = Vec::new();
//     items.push(vec![1,2,3]);
//     items.push(vec![4,5,6]);
//     items[0].append(&mut items[1]);
// }

// fn no_problem_with_option() {
//     let mut items = Vec::new();
//     items.push(Some(vec![1,2,3]));
//     items.push(Some(vec![4,5,6]));
//     let mut second = items[1].take();
//     items[0].as_mut().unwrap().append(second.as_mut().unwrap());
// }

// fn no_problem_with_box() {
//     let mut items = Vec::new();
//     items.push(Box::new(vec![1,2,3]));
//     items.push(Box::new(vec![4,5,6]));
//     let mut second = items[1].clone();
//     items[0].as_mut().append(second.as_mut());
// }

use std::collections::LinkedList;

fn print<W: io::Write>(out: &mut W, v: LinkedList<String>) {
    for i in v {
        write!(out, "{}", i).unwrap();
    }
    writeln!(out, "").unwrap();
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<u32>();
    let mut words = Vec::new();
    for _ in 0..n {
        let mut v = LinkedList::new();
        v.push_back(scan.token::<String>());
        words.push(Some(v));
    }
    if n == 1 {
        print(out, words[0].take().unwrap());
    } else {
        for i in 0..n-1 {
            let a = scan.token::<usize>() - 1;
            let b = scan.token::<usize>() - 1;
            let mut wb = words[b].take();
            words[a].as_mut().unwrap().append(wb.as_mut().unwrap());
            if i == n-2 {
                print(out, words[a].take().unwrap());
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out); 
}
