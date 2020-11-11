use std::io; use std::str;
#[allow(unused_imports)] use std::convert::TryFrom;
#[allow(unused_imports)] use std::cmp::{Reverse, Ordering};
#[allow(unused_imports)] use std::collections::{LinkedList, HashSet, HashMap, BinaryHeap, BTreeSet, BTreeMap};

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, } impl<R: io::BufRead> UnsafeScanner<R> { pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } } pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } } pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } } pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } } } fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

struct UnionFind {
    ranks: Vec<usize>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
    childs: HashMap<usize, HashSet<usize>>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let ranks = vec![1; size];
        let mut parents = vec![0; size];
        for i in 0..size { parents[i] = i }
        let sizes = vec![1; size];
        let childs: HashMap<usize, HashSet<usize>> = HashMap::new();
        UnionFind { ranks, parents, sizes, childs }
    }

    fn remove_child(&mut self, parent: usize, child: usize) {
        if parent == child { return; }
        match self.childs.get_mut(&parent) {
            Some(c) => { c.remove(&child); },
            None =>  { }
        };
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        if parent == child { return; }
        match self.childs.get_mut(&parent) {
            Some(c) => { c.insert(child); },
            None =>  {
                self.childs.insert(parent, [child].iter().cloned().collect());
            }
        };
    }

    fn set_parent(&mut self, new_parent: usize, child: usize) -> usize {
        let prev_parent = self.parents[child];
        if prev_parent == new_parent { return prev_parent; }
        self.remove_child(prev_parent, child);
        self.parents[child] = new_parent;
        self.add_child(new_parent, child);
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

    fn moves(&mut self, a: usize, b: usize) {
        if self.same(a, b) { return; }
        let b = self.find(b);
        let pa = self.find(a);
        if let Some(childs) = self.childs.get(&a) {
            let it = childs.clone();
            let mut it = it.iter();
            if let Some(first_child) = it.next() {
                for child in it {
                    self.set_parent(*first_child, *child);
                }
                if pa == a {
                    self.set_parent(*first_child, *first_child);
                    self.sizes[*first_child] = self.sizes[pa] - 1;
                } else {
                    self.set_parent(pa, *first_child);
                }
            }
        }
        self.set_parent(b, a);
        if pa != a {
            self.sizes[pa] -= 1;
        }
        self.sizes[b] += 1;
    }

    fn sum(&mut self, a: usize) -> usize {
        let mut total = a;
        let childs = self.childs.get(&a);
        if let Some(childs) = childs {
            for child in childs.clone().iter() {
                total += self.sum(*child);
            };
        }
        total
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    loop {
        let n = scan.token::<usize>();
        if n.is_none() { break; };
        let n = n.unwrap();
        let m = scan.token::<usize>().unwrap();
        let mut uf = UnionFind::new(n + 1);
        for _ in 0..m {
            let cmd = scan.token::<usize>().unwrap();
            match cmd {
                1 => {
                    let a = scan.token::<usize>().unwrap();
                    let b = scan.token::<usize>().unwrap();
                    uf.union(a, b);
                },
                2 => {
                    let a = scan.token::<usize>().unwrap();
                    let b = scan.token::<usize>().unwrap();
                    uf.moves(a, b);
                },
                3 => {
                    let a = scan.token::<usize>().unwrap();
                    let num = uf.size(a);
                    let root = uf.find(a);
                    let total = uf.sum(root);
                    // dbg!(&uf.parents);
                    // dbg!(&uf.childs);
                    // dbg!(&elems);
                    writeln!(out, "{} {}", num, total);
                },
                _ => panic!()
            }
        }
    }

}
