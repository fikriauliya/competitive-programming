use std::io;
use std::str;
use std::convert::TryFrom;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Eq, PartialEq, Copy, Clone)]
enum Class {
    Upper = 2,
    Middle = 1,
    Lower = 0
}

#[derive(PartialEq, Eq)]
struct Person {
    name: String, 
    class: Vec<Class>
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = Ordering::Equal;
        let mut s_iter = self.class.iter().rev();
        let mut o_iter = other.class.iter().rev();
        loop {
            let s = s_iter.next();
            let o = o_iter.next();
            match (s, o) {
                (Some(s), Some(o)) => {
                    let cur_ord = (*s as i32).cmp(&(*o as i32));
                    if let Ordering::Equal = cur_ord { continue; }
                    ord = cur_ord; break;
                }
                (None, None) => break,
                (None, Some(Class::Middle)) | (Some(Class::Middle), None) => { continue; }
                (Some(s), None) => { ord = (*s as i32).cmp(&(Class::Middle as i32)); break },
                (None, Some(o)) => { ord = (Class::Middle as i32).cmp(&(*o as i32)); break },
            }
        };
        ord.then_with(|| other.name.cmp(&self.name))
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let mut persons = Vec::new();
        for i in 0..n {
            let mut name = scan.token::<String>().unwrap();
            name.pop();

            let class = scan.token::<String>().unwrap();
            scan.token::<String>().unwrap();

            let class = class.split("-");
            let class = class.map(|it| match it {
                "upper" => Class::Upper,
                "middle" => Class::Middle,
                "lower" => Class::Lower,
                _ => panic!("Invalid class")
            }).collect();

            let p = Person { name, class };
            persons.push(p);
        }
        persons.sort();
        persons.reverse();
        for person in persons {
            println!("{}", person.name);
        }
        println!("==============================");
    }
}
