use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::{HashSet, HashMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut project = "".to_string();
    let mut project_users: HashMap<String, HashSet<String>> = HashMap::new();
    let mut user_reg: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let line = scan.line().unwrap();
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        if first == '0' { return; }
        else if first == '1' { 
            let mut project_users_count: HashMap<String, usize> = HashMap::new();
            for (k, v) in project_users.iter() {
                project_users_count.insert(k.to_string(), v.len());
            }
            for (_, projects) in user_reg.iter() {
                if projects.len() > 1 {
                    for project in projects {
                        if let Some(count) = project_users_count.get_mut(project) {
                            *count -= 1;
                        }
                    }
                }
            }
            let mut pc: Vec<(&String, &usize)> = project_users_count.iter().collect();
            pc.sort_by(|x, y| {
                y.1.cmp(&x.1).then_with(|| {
                    x.0.cmp(&y.0)
                })
            });
            for p in pc {
                writeln!(out, "{} {}", p.0, p.1);
            }
            project_users.clear();
            user_reg.clear();
        }
        else if line.to_uppercase() == line {
            project = line;
            project_users.insert(project.clone(), HashSet::new());
        } else {
            let username = line;
            let users = project_users.get_mut(&project).unwrap();
            let inserted = users.insert(username.clone());
            if inserted {
                match user_reg.get_mut(&username) {
                    Some(projects) => {
                        projects.push(project.clone());
                    },
                    None => { 
                        user_reg.insert(username, vec![project.clone()]); 
                    },
                }
            }
        }
    }
}
