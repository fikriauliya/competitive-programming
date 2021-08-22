#[allow(unused_imports)]
use std::cmp::{Ordering, Reverse};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList};
#[allow(unused_imports)]
use std::convert::TryFrom;
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
            let len = self
                .reader
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct CellDef {
    val: char,
    horizontal_start: (usize, usize),
    horizontal_end: (usize, usize),
    vertical_start: (usize, usize),
    vertical_end: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Cell {
    Blocked,
    Space(CellDef),
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let t = scan.token().unwrap();
    for tt in 0..t {
        let n = scan.token().unwrap();
        let m = scan.token().unwrap();
        let mut v = vec![vec![' '; m]; n];
        for i in 0..n {
            let line = scan.line().unwrap();
            let lines = line.chars().collect::<Vec<char>>();
            for j in 0..m {
                v[i][j] = lines[j];
            }
        }

        let mut board: Vec<Vec<Cell>> = vec![vec![Cell::Blocked; m]; n];
        for i in 0..n {
            for j in 0..m {
                if v[i][j] == '#' {
                    board[i][j] = Cell::Blocked;
                } else {
                    let cell = CellDef {
                        val: v[i][j],
                        horizontal_start: (i, j),
                        horizontal_end: (i, m - 1),
                        vertical_start: (i, j),
                        vertical_end: (n - 1, j),
                    };
                    board[i][j] = Cell::Space(cell);
                }
            }
        }
        for i in 0..n {
            let mut horizontal_start = None;
            for j in 0..m {
                match (horizontal_start, board[i][j]) {
                    (None, Cell::Space(_)) => {
                        horizontal_start = Some((i, j));
                    }
                    (Some(_), Cell::Blocked) => {
                        for k in (0..j).rev() {
                            match board[i][k] {
                                Cell::Space(mut cell) => {
                                    cell.horizontal_end = (i, j - 1);
                                    board[i][k] = Cell::Space(cell);
                                }
                                Cell::Blocked => {
                                    break;
                                }
                            }
                        }
                        horizontal_start = None;
                    }
                    (_, _) => {}
                }
                match board[i][j] {
                    Cell::Blocked => {}
                    Cell::Space(mut cell) => {
                        cell.horizontal_start = horizontal_start.unwrap();
                        board[i][j] = Cell::Space(cell);
                    }
                }
            }
        }
        for j in 0..m {
            let mut vertical_start = None;
            for i in 0..n {
                match (vertical_start, board[i][j]) {
                    (None, Cell::Space(_)) => {
                        vertical_start = Some((i, j));
                    }
                    (Some(_), Cell::Blocked) => {
                        for k in (0..i).rev() {
                            match board[k][j] {
                                Cell::Space(mut cell) => {
                                    cell.vertical_end = (i - 1, j);
                                    board[k][j] = Cell::Space(cell);
                                }
                                Cell::Blocked => {
                                    break;
                                }
                            }
                        }
                        vertical_start = None;
                    }
                    (_, _) => {}
                }
                match board.get_mut(i).unwrap().get_mut(j).unwrap() {
                    Cell::Blocked => {}
                    Cell::Space(&mut cell) => {
                        cell.vertical_start = vertical_start.unwrap();
                        // board[i][j] = Cell::Space(cell);
                    }
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                res += rec(&mut board, i, j);
            }
        }
        write!(out, "Case #{}: {}\n", tt + 1, res).ok();
        for i in 0..n {
            for j in 0..m {
                match board[i][j] {
                    Cell::Space(cell) => write!(out, "{}", cell.val).unwrap(),
                    Cell::Blocked => write!(out, "#").unwrap(),
                }
            }
            write!(out, "\n");
        }
    }
}

fn rec(board: &mut Vec<Vec<Cell>>, i: usize, j: usize) -> usize {
    match board[i][j] {
        Cell::Blocked => 0,
        Cell::Space(cell) => {
            let mut res = 0;
            if cell.val != '.' {
                {
                    let len = cell.horizontal_end.1 - cell.horizontal_start.1 + 1;
                    if len > 1 {
                        let pos = j - cell.horizontal_start.1;
                        let pair_pos = cell.horizontal_start.1 + len - pos - 1;
                        if let Cell::Space(mut pair_cell) = board[i][pair_pos] {
                            if pair_cell.val == '.' {
                                pair_cell.val = cell.val;
                                board[i][pair_pos] = Cell::Space(pair_cell);
                                res += rec(board, i, pair_pos);
                                res += 1;
                            }
                        }
                    }
                }
                {
                    let len = cell.vertical_end.0 - cell.vertical_start.0 + 1;
                    if len > 1 {
                        let pos = i - cell.vertical_start.0;
                        let pair_pos = cell.vertical_start.0 + len - pos - 1;
                        if let Cell::Space(mut pair_cell) = board[pair_pos][j] {
                            if pair_cell.val == '.' {
                                pair_cell.val = cell.val;
                                board[pair_pos][j] = Cell::Space(pair_cell);
                                res += rec(board, pair_pos, j);
                                res += 1;
                            }
                        }
                    }
                }
            }
            res
        }
    }
}
