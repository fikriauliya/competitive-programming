use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn anchestors(name: &str, 
              level: usize,
              mothers: &HashMap<String, String>,
              fathers: &HashMap<String, String>,
              dobs: &HashMap<String, String>,
              deaths: &HashMap<String, String>,
              children: &HashMap<String, HashSet<String>>) {
    if level == 0 {
        println!("ANCESTORS of {}", name);
    } else {
        for i in 0..level*2 {
            print!(" ");
        }
        match (dobs.get(name), deaths.get(name)) {
            (Some(_), Some(_)) => 
                println!("{} {} - {}", name,
                       dobs.get(name).unwrap_or(&"".to_string()),
                       deaths.get(name).unwrap_or(&"".to_string())),
            (Some(_), None) => 
                println!("{} {} -", name,
                       dobs.get(name).unwrap_or(&"".to_string())),
            _ => println!("{}", name),
        }
    }
    match (mothers.get(name), fathers.get(name)) {
        (Some(m), Some(f)) => {
            if f > m {
                anchestors(m, level + 1, mothers, fathers, dobs, deaths, children);
                anchestors(f, level + 1, mothers, fathers, dobs, deaths, children);
            } else {
                anchestors(f, level + 1, mothers, fathers, dobs, deaths, children);
                anchestors(m, level + 1, mothers, fathers, dobs, deaths, children);
            }
        }
        _ => {}
    }
}

fn descendants(name: &str, 
              level: usize,
              mothers: &HashMap<String, String>,
              fathers: &HashMap<String, String>,
              dobs: &HashMap<String, String>,
              deaths: &HashMap<String, String>,
              children: &HashMap<String, HashSet<String>>) {
    if level == 0 {
        println!("DESCENDANTS of {}", name);
    } else {
        for i in 0..level*2 {
            print!(" ");
        }
        match (dobs.get(name), deaths.get(name)) {
            (Some(_), Some(_)) => 
                println!("{} {} - {}", name,
                       dobs.get(name).unwrap_or(&"".to_string()),
                       deaths.get(name).unwrap_or(&"".to_string())),
            (Some(_), None) => 
                println!("{} {} -", name,
                       dobs.get(name).unwrap_or(&"".to_string())),
            _ => println!("{}", name),
        }
    }

    if let Some(c) = children.get(name) {
        let mut c: Vec<&String> = c.iter().collect();
        c.sort();
        for cc in c {
            descendants(cc, level + 1, mothers, fathers, dobs, deaths, children);
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut mothers = HashMap::new();
    let mut fathers = HashMap::new();
    let mut children: HashMap<String, HashSet<String>> = HashMap::new();
    let mut dobs = HashMap::new();
    let mut deaths = HashMap::new();

    let mut first_output = true;
    loop {
        let line = scan.line().unwrap();

        if line.starts_with("BIRTH") {
            let mut sp = line[6..line.len()].split(" : ");
            let child = sp.next().unwrap().to_string();
            let date = sp.next().unwrap().to_string();
            let mother = sp.next().unwrap().to_string();
            let father = sp.next().unwrap().to_string();

            mothers.insert(child.clone(), mother.clone());
            fathers.insert(child.clone(), father.clone());

            if let Some(c) = children.get_mut(&father) {
                c.insert(child.clone());
            } else {
                children.insert(father.clone(), 
                                [child.clone()].iter().cloned().collect());
            }

            if let Some(c) = children.get_mut(&mother) {
                c.insert(child.clone());
            } else {
                children.insert(mother.clone(), 
                                [child.clone()].iter().cloned().collect());
            }

            dobs.insert(child.clone(), date);
        } else if line.starts_with("DEATH") {
            let mut sp = line[6..line.len()].split(" : ");
            let person = sp.next().unwrap().to_string();
            let date = sp.next().unwrap().to_string();

            deaths.insert(person, date);
        } else if line.starts_with("ANCESTORS") {
            let person = &line[10..line.len()];
            if !first_output { println!()}
            first_output = false;
            anchestors(person, 0, &mothers, &fathers, &dobs, &deaths, &children);
        } else if line.starts_with("DESCENDANTS") {
            let person = &line[12..line.len()];
            if !first_output { println!()}
            first_output = false;
            descendants(person, 0, &mothers, &fathers, &dobs, &deaths, &children);
        } else if line.starts_with("QUIT") {
            break;
        }
    }
}
