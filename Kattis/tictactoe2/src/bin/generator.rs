use std::io;
use std::io::Write;

pub fn rec<W: Write>(out: &mut W, level: usize, s: &mut String) {
    let seq = vec!['.','X','O'];

    if level == 9 { 
        writeln!(out, "{}\n", s);
        return; 
    }
    for i in 0..3 {
        match level {
            3 | 6 | 9 => {s.push('\n'); ()},
            _ => ()
        }
        s.push(seq[i]);
        rec(out, level + 1, s);
        s.pop();
        match level {
            3 | 6 | 9 => {s.pop(); ()},
            _ => ()
        }
    }
}

pub fn generate() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let n = 3i32.pow(9);
    writeln!(out, "{}", n);

    rec(&mut out, 0, &mut "".to_string());
}

pub fn main() {
    generate();
}

