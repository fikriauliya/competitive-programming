use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token::<usize>().unwrap();
    'case: for tt in 0..t {
        let n = scan.token::<usize>().unwrap();
        let m = scan.token::<usize>().unwrap();
        let mut days = Vec::with_capacity(m);
        let mut locs: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut counts = vec![0; n+1];
        for i in 0..m {
            let k = scan.token::<usize>().unwrap();
            days.push(Vec::with_capacity(k));
            for _ in 0..k {
                let f = scan.token::<usize>().unwrap();
                counts[f] += 1;
                days[i].push(f);
                match locs.get_mut(&f) {
                    Some(vals) => {vals.insert(i);}
                    None => {
                        let mut set = HashSet::new();
                        set.insert(i);
                        locs.insert(f, set);
                    },
                }
            }
        }
        let mut tree: BTreeMap<(usize, usize), ()> = BTreeMap::new();
        for i in 1..=n {
            tree.insert((counts[i], i), ());
        }
        let mut ctr = 0;
        let mut res = vec![0; m];
        while ctr < m {
            let mut it = tree.iter();
            let top = it.next().unwrap();
            let count = top.0.clone();
            tree.remove(&count);

            let i = count.1;
            let max = (m+1)/2;
            let mut cur_ctr = 0;

            let loc = locs.get(&i);
            if loc.is_none () {
                writeln!(out, "NO").unwrap();
                continue 'case;
            }
            for d in loc.unwrap().clone() {
                if cur_ctr > max {
                    locs.get_mut(&i).unwrap().remove(&d);
                } else {
                    res[d] = i;
                    ctr += 1;
                    for f in &days[d] {
                        if *f != i {
                            tree.remove(&(locs.get(&f).unwrap().len(), *f));
                            locs.get_mut(&f).unwrap().remove(&d);
                            tree.insert((locs.get(&f).unwrap().len(), *f), ());
                        }
                    }
                    cur_ctr += 1;
                }
            }
        }
        writeln!(out, "YES").unwrap();
        let res:Vec<String> = res.iter().map(|s| s.to_string()).collect();
        writeln!(out, "{}", res.join(" ")).unwrap();
    }
}
