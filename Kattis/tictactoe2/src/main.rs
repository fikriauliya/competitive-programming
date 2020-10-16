use std::io;
use std::str;
use std::convert::TryFrom;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
    DiagonalInvert
}

fn calc_cumm(orientation: Orientation, 
             m: usize, r: usize, c: usize,
             y: i32, x: i32,
             cumm: &mut Vec<usize>,
             board: &mut Vec<Vec<char>>,
             scored_board: &mut Vec<Vec<Vec<usize>>>){
    let mut fill_scored_board = |index:usize, count: usize| {
        dbg!(x, y, count);
        for j in 1..count+1 {
            match orientation {
                Orientation::Horizontal => scored_board[index][y as usize][(x-j as i32) as usize] += 1,
                Orientation::Vertical => scored_board[index][(y-j as i32) as usize][x as usize] += 1,
                Orientation::Diagonal => scored_board[index][(y-j as i32) as usize][(x-j as i32) as usize] += 1,
                Orientation::DiagonalInvert => scored_board[index][(y-j as i32) as usize][(x+j as i32) as usize] += 1,
            }
        };
    };
    if !((0..r as i32).contains(&y) && (0..c as i32).contains(&x)) {
        if cumm[1] >= m { fill_scored_board(1, cumm[1]); }
        if cumm[0] >= m { fill_scored_board(0, cumm[0]); }
        cumm[0] = 0; cumm[1] = 0; 
    } else {
        match board[y as usize][x as usize] {
            'X' => { 
                if cumm[1] >= m { fill_scored_board(1, cumm[1]); }
                cumm[0] += 1; cumm[1] = 0; 
            },
            'O' => { 
                if cumm[0] >= m { fill_scored_board(0, cumm[0]); }
                cumm[1] += 1; cumm[0] = 0; 
            },
            _ => { 
                if cumm[1] >= m { fill_scored_board(1, cumm[1]); }
                if cumm[0] >= m { fill_scored_board(0, cumm[0]); }
                cumm[0] = 0; cumm[1] = 0; 
            }
        };
    }
}


fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let n = scan.token::<usize>().unwrap();
    let m = 3;
    let r = 3;
    let c = 3;
    let mut board = vec![vec![' ';c];r];
    'cases: for nn in 0..n {
        for i in 0..r {
            let line: Vec<char> = scan.line().unwrap().chars().collect();
            for j in 0..c {
                board[i][j] = line[j];
            }
        }
        scan.line();
        dbg!(&board);

        let total_x:usize = board.iter().map(|r| r.iter().filter(|rr| rr == &&'X').count()).sum();
        let total_o:usize = board.iter().map(|r| r.iter().filter(|rr| rr == &&'O').count()).sum();
        if total_o > total_x {
            writeln!(out, "no");
            continue 'cases;
        }

        let mut x = 0i32;
        let mut y = 0i32;

        let mut wins_hor = vec![0; 2];
        let mut wins_ver = vec![0; 2];
        let mut wins_diag = vec![0; 2];
        let mut wins_diag_invert = vec![0; 2];

        let mut cumm_hor = vec![0; 2];
        let mut cumm_ver = vec![0; 2];
        let mut cumm_diag = vec![0; 2];
        let mut cumm_diag_invert = vec![0; 2];

        let mut scored_board = vec![vec![vec![0;c];r];2];

        x = 0; y = 0;
        let cumm = &mut cumm_hor;
        let wins = &mut wins_hor;
        let orientation = Orientation::Horizontal;
        while (y, x) != (r as i32, 0) {
            calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
            dbg!(&y, &x, &cumm);
            for i in 0..2 {
                if cumm[i] == m { wins[i] += 1; } 
                if cumm[i] == 2 * m {
                    dbg!(&scored_board);
                    writeln!(out, "no");
                    continue 'cases;
                }
            }
            x += 1;
            if y == r as i32 || x == c as i32 { 
                calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
                x = 0; y += 1;
            };
        }
        dbg!(&scored_board);

        x = 0; y = 0;
        let cumm = &mut cumm_ver;
        let wins = &mut wins_ver;
        let orientation = Orientation::Vertical;
        while (y, x) != (0, c as i32) {
            calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
            dbg!(&y, &x, &cumm);
            for i in 0..2 {
                if cumm[i] == m { wins[i] += 1; } 
                if cumm[i] == 2 * m {
                    dbg!(&scored_board);
                    writeln!(out, "no");
                    continue 'cases;
                }
            }
            y += 1;
            if y == r as i32 || x == c as i32 { 
                calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
                x += 1; y = 0;
            };
        }
        dbg!(&scored_board);

        x = 0; y = r as i32-1;
        let cumm = &mut cumm_diag;
        let wins = &mut wins_diag;
        let orientation = Orientation::Diagonal;
        let mut start_x = 0 as i32;
        let mut start_y = r as i32-1;

        while start_x != c as i32 {
            calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
            dbg!(&y, &x, &cumm);
            for i in 0..2 {
                if cumm[i] == m { wins[i] += 1; } 
                if cumm[i] == 2 * m {
                    dbg!(&scored_board);
                    writeln!(out, "no");
                    continue 'cases;
                }
            }
            x += 1; y += 1;
            if y == r as i32 || x == c as i32  { 
                calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
                if start_y > 0 { start_y -= 1; }
                else if start_y == 0 { start_x += 1; }
                x = start_x; y = start_y;
            };
        }
        dbg!(&scored_board);

        x = 0; y = 0;
        let cumm = &mut cumm_diag_invert;
        let wins = &mut wins_diag_invert;
        let orientation = Orientation::DiagonalInvert;
        let mut start_x = 0i32;
        let mut start_y = 0i32;

        while start_y != r as i32 {
            calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
            dbg!(&y, &x, &cumm);
            for i in 0..2 {
                if cumm[i] == m { wins[i] += 1; } 
                if cumm[i] == 2 * m {
                    dbg!(&scored_board);
                    writeln!(out, "no");
                    continue 'cases;
                }
            }
            x -= 1; y += 1;
            if y == r as i32 || x == -1 { 
                calc_cumm(orientation, m, r, c, y, x, cumm, &mut board, &mut scored_board);
                if start_x <= c as i32 - 2 { start_x += 1; }
                else if start_x == c as i32 - 1 { start_y += 1; }
                x = start_x; y = start_y;
            };
        }
        dbg!(&scored_board);

        writeln!(out, "yes");
        // let total_x = board.iter().map(|r| r.iter().filter(|rr| rr == 'X')).sum();
    }
}
