use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

struct UnionFind {
    ranks: Vec<usize>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let ranks = vec![1; size];
        let mut parents = vec![0; size];
        for i in 0..size { parents[i] = i }
        let sizes = vec![1; size];
        UnionFind { ranks, parents, sizes }
    }

    fn set_parent(&mut self, new_parent: usize, child: usize) -> usize {
        let prev_parent = self.parents[child];
        if prev_parent == new_parent { return prev_parent; }
        self.parents[child] = new_parent;
        new_parent
    }

    fn find(&mut self, val: usize) -> usize {
        if self.parents[val] == val { return val }
        let new_parent = self.find(self.parents[val]);
        self.set_parent(new_parent, val)
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn union(&mut self, a: usize, b: usize) -> usize {
        if self.same(a, b) { return self.find(a); }
        let a = self.find(a); let b = self.find(b);
        let (parent, child) = 
            if self.ranks[a] > self.ranks[b] { (a, b) }
            else if self.ranks[a] < self.ranks[b] { (b, a) }
            else { self.ranks[a] += 1; (a, b) };
        
        self.set_parent(parent, child);
        self.sizes[parent] += self.sizes[child]; 
        parent
    }

    fn size(&mut self, a: usize) -> usize {
        let a = self.find(a);
        self.sizes[a]
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let mut uf = UnionFind::new(500_001);
    let mut res = 0;
    for _ in 0..n {
        let m = scan.token::<usize>().unwrap();
        let mut to_be_parents: HashMap<usize, usize> = HashMap::new();
        for _ in 0..m {
            let item = scan.token::<usize>().unwrap();
            let parent = uf.find(item);
            match to_be_parents.get_mut(&parent) {
                Some(count) => { *count += 1 }
                None => {to_be_parents.insert(parent, 1);},
            }
        }
        let mut can = true;
        for (k, v) in &to_be_parents {
            if *v != uf.size(*k) { can = false; break; }
        }
        // dbg!(&to_be_parents);
        // dbg!(&can);
        if !can { continue; }
        res += 1;
        let sets: Vec<&usize> = to_be_parents.keys().collect();
        for i in 1..sets.len() {
            uf.union(*sets[i], *sets[i-1]);
        }
    }
    writeln!(out, "{}", res);
}
