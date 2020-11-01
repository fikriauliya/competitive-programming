use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

#[derive(Debug)]
enum Operator { Plus, Minus }

enum Operand {
    Variable(String),
    Value(i16)
}

fn to_variable(operand: Operand, val_to_var: &HashMap<i16, String>, var_to_val: &HashMap<String, i16>) -> Option<String> {
    use Operand::*;
    match operand {
        Variable(s) => var_to_val.get(&s).and_then(|val| val_to_var.get(val).cloned()),
        Value(v) => val_to_var.get(&v).cloned()
    }
}

fn to_value(operand: Operand, var_to_val: &HashMap<String, i16>) -> Option<i16> {
    use Operand::*;
    match operand {
        Variable(s) => var_to_val.get(&s).cloned(),
        Value(v) => Some(v)
    }
}

fn eval(operands: &mut Vec<Option<Operand>>, 
        operators: &mut Vec<Operator>,
        var_to_val: &HashMap<String, i16>,
        ) {
    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        let var_b = operands.pop().unwrap();
        let var_a = operands.pop().unwrap();
        let val_b = var_b.and_then(|var| to_value(var, var_to_val));
        let val_a = var_a.and_then(|var| to_value(var, var_to_val));

        let res = val_a.and_then(|a| val_b.map(|b| {
            let val = match op {
                Operator::Plus => a + b,
                Operator::Minus => a - b,
            };
            Operand::Value(val)
        }));
        operands.push(res);
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut var_to_val = HashMap::new();
    let mut val_to_var = HashMap::new();
    loop {
        let line = scan.line();
        if line.is_none() { break };
        let line = line.unwrap();
        let line = line.trim();
        let mut it = line.split_ascii_whitespace();
        match it.next() {
            Some("def") => {
                let var = it.next().unwrap();
                let val = it.next().unwrap().parse::<i16>().unwrap();
                var_to_val.get(&var.to_string()).map(|val| val_to_var.remove(val));
                var_to_val.insert(var.to_string(), val);
                val_to_var.insert(val, var.to_string());
            },
            Some("clear") => { var_to_val.clear(); val_to_var.clear() },
            Some("calc") => {
                let mut operands: Vec<Option<Operand>> = Vec::new();
                let mut operators: Vec<Operator> = Vec::new();

                loop {
                    let var = it.next().unwrap();
                    operands.push(Some(Operand::Variable(var.to_string())));
                    eval(&mut operands, &mut operators, &var_to_val);

                    let op = it.next().unwrap();
                    match op {
                        "=" => { break; },
                        "+" => { operators.push(Operator::Plus); },
                        "-" => { operators.push(Operator::Minus) },
                        _ => panic!()
                    };
                }
                eval(&mut operands, &mut operators, &var_to_val);

                let res = operands.pop().unwrap();
                let res = res.and_then(|res| to_variable(res, &val_to_var, &var_to_val));
                match res {
                    Some(res) => println!("{} {}", &line[5..line.len()], res),
                    _ => println!("{} unknown", &line[5..line.len()])
                }
            },
            _ => panic!()
        }
    }
}
