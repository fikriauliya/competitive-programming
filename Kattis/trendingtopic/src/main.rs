use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Input,
    Query,
    Idle,
    EndInput
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut state = State::Idle;
    let mut word_count: HashMap<String, usize> = HashMap::new();
    let mut count_words: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut word_queue: VecDeque<Vec<String>>= VecDeque::new();
    let mut accum_words = Vec::new();

    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let line = line.trim();

        state = match (state, &line[..]) {
            (State::Idle, "<text>") => State::Input,
            (State::Idle, line) if line.starts_with("<top ") => State::Query,
            (_, "</text>") => State::EndInput,
            (_, "") => continue,
            _ => state
        };
        if state == State::Input && line == "<text>" { continue; }

        // dbg!(&line);
        match state {
            State::Idle => { continue },
            State::Query => {
                let mut counts: Vec<&usize> = count_words.keys().collect();
                let mut sp = line.split(" "); sp.next();
                let n = sp.next().unwrap().parse::<usize>().unwrap();
                counts.sort();
                counts.reverse();
                // dbg!(&counts);
                println!("<top {}>", n);
                for i in 0..n.min(counts.len()) {
                    let count = counts[i];
                    let words = count_words.get(count).unwrap();
                    let mut words: Vec<&String> = words.iter().collect();
                    words.sort();
                    for word in words {
                        println!("{} {}", word, count);
                    }
                }
                println!("</top>");
            },
            State::Input => {
                let words: Vec<String> = line.split_ascii_whitespace()
                    .filter(|w| w.len() >= 4)
                    .map(|w| w.to_string())
                    .collect();
                for word in &words {
                    accum_words.push(word.to_string());
                }
            }
            State::EndInput => {
                for word in &accum_words {
                    let count = match word_count.get_mut(word) {
                        Some(wc) => {*wc += 1; *wc },
                        None => { word_count.insert(word.to_string(), 1); 1 }
                    };
                    match count_words.get_mut(&count) {
                        Some(wd) => { wd.insert(word.to_string()); }
                        None => { 
                            let mut set = HashSet::new();
                            set.insert(word.to_string());
                            count_words.insert(count, set);
                        }
                    };
                    if let Some(wd) = count_words.get_mut(&(count - 1)) {
                        wd.remove(word);
                        if wd.is_empty() {
                            count_words.remove(&(count - 1));
                        }
                    }
                }
                word_queue.push_back(accum_words.clone());
                accum_words.clear();
                if word_queue.len() > 7 {
                    let words = word_queue.pop_front().unwrap();
                    for word in &words {
                        let count = match word_count.get_mut(word) {
                            Some(wc) => {*wc -= 1; *wc },
                            None => 0,
                        };
                        if count > 0 {
                            match count_words.get_mut(&count) {
                                Some(wd) => { wd.insert(word.to_string()); }
                                None => { 
                                    let mut set = HashSet::new();
                                    set.insert(word.to_string());
                                    count_words.insert(count, set);
                                }
                            };
                        }
                        if let Some(wd) = count_words.get_mut(&(count + 1)) {
                            wd.remove(word);
                            if wd.is_empty() {
                                count_words.remove(&(count + 1));
                            }
                        }
                    }
                }
            }
        }

        if let State::Query = state { state = State::Idle; }
        if let State::EndInput = state { state = State::Idle; }
    }
}
