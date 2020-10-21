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

enum Direction {
    Left,
    Right
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let l = scan.token::<i32>().unwrap();
    let w = scan.token::<i32>().unwrap();
    let mut offsets = vec![0i32;l as usize];
    let mut intervals = vec![0i32;l as usize];
    let mut speeds = vec![0i32;l as usize];

    // let m = vec![vec![
    for ll in 0..l {
        let o = scan.token::<i32>().unwrap();
        let i = scan.token::<i32>().unwrap();
        let s = scan.token::<i32>().unwrap();
        intervals[ll as usize] = i;
        speeds[ll as usize] = s;
        offsets[ll as usize] = o;

    }
    let mut x = scan.token::<i32>().unwrap();
    let mut y = l as i32;
    let movements = scan.token::<String>().unwrap();
    let movements = movements.trim().chars();
    dbg!(&l, &w, &y, &x, &offsets, &intervals, &speeds);

    let mut result = "squish";
    for m in movements {
        let (new_x, new_y) = match m {
            'U' => (x, y-1),
            'L' => (x-1, y),
            'R' => (x+1, y),
            'D' => (x, y+1),
            _ => (x, y)
        };
        x = new_x; y = new_y;
        let direction = if y % 2 == 1 { Direction::Left } else { Direction::Right };

        if y == -1 { result = "safe"; break; }
        if y < l { 
            let yi = y as usize;
            if intervals[yi] <= speeds[yi] {
                break;
            }
            let lane_x = if let Direction::Right = direction { x } else { w - x - 1 };
            dbg!(&m, &y, &x, &lane_x, &speeds[yi], &offsets[yi], &intervals[yi]);
            let x_from_nearest_offset = {
                let mut prev_offset = offsets[yi];
                while prev_offset + intervals[yi] <= lane_x {
                    prev_offset += intervals[yi];
                };

                if prev_offset <= lane_x {
                    lane_x - prev_offset
                } else {
                    lane_x + (intervals[yi] - offsets[yi])
                }
            };
            dbg!(&x_from_nearest_offset);
            if speeds[yi] != 0 {
                let start = match (m, direction) {
                    ('L', Direction::Right) | ('R', Direction::Left) => 0,
                    _ => 1,
                };
                if (start..speeds[yi]+1).contains(&x_from_nearest_offset) {
                    break;
                }
            } else {
                if x_from_nearest_offset == 0 { break; }
            }
        }

        dbg!(&offsets);
        offsets = offsets.iter().enumerate()
            .map(|(i, o)| (*o + speeds[i]) % intervals[i])
            .collect();
        dbg!(&offsets);
    }
    
    println!("{}", result);
}
