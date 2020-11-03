use std::io;
use std::str;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;

pub struct UnsafeScanner<R> { reader: R, buf_str: Vec<u8>, buf_iter: str::SplitAsciiWhitespace<'static>, }
impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self { Self { reader, buf_str: vec![], buf_iter: "".split_ascii_whitespace(), } }
    pub fn trim_newline(s: &mut String) { if s.ends_with('\n') { s.pop(); if s.ends_with('\r') { s.pop(); } } }
    pub fn token<T: str::FromStr>(&mut self) -> Option<T> { loop { if let Some(token) = self.buf_iter.next() { return Some(token.parse().ok().expect("Failed parse")); } self.buf_str.clear(); let len = self.reader .read_until(b'\n', &mut self.buf_str) .expect("Failed read"); if len == 0 { return None; } self.buf_iter = unsafe { let slice = str::from_utf8_unchecked(&self.buf_str); std::mem::transmute(slice.split_ascii_whitespace()) } } }
    pub fn line(&mut self) -> Option<String> { let mut input = String::new(); let len = self.reader.read_line(&mut input).expect("Failed read"); match len { 0 => None, _ => { Self::trim_newline(&mut input); Some(input) } } }
}
fn main() { let (stdin, stdout) = (io::stdin(), io::stdout()); let mut scan = UnsafeScanner::new(stdin.lock()); let mut out = io::BufWriter::new(stdout.lock()); solve(&mut scan, &mut out); }

type Label = i32;
type VariableName = String;
type Integer = i32;
type Statements = HashMap<Label, Statement>;

#[derive(Debug)]
enum VariableOrIntegerLiteral {
    Variable(VariableName),
    IntegerLiteral(Integer),
}

impl VariableOrIntegerLiteral {
    fn parse(token: &mut SplitAsciiWhitespace) -> VariableOrIntegerLiteral {
        use VariableOrIntegerLiteral::*;
        let t = token.next().unwrap().to_string(); 
        if let Ok(integer) = t.parse::<Integer>() {
            IntegerLiteral(integer)
        } else {
            Variable(t)
        }
    }

    fn eval(&self, variables: &Variables) -> Integer {
        use VariableOrIntegerLiteral::*;
        match self {
            Variable(var) => *variables.get(var).unwrap(),
            IntegerLiteral(int) => *int,
        }
    }
}

#[derive(Debug)]
enum ArithmeticStatement {
    Value(VariableOrIntegerLiteral),
    Addition(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    Substraction(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    Multiplication(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    Division(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
}

impl ArithmeticStatement {
    fn parse(token: &mut SplitAsciiWhitespace) -> ArithmeticStatement {
        let var_a = VariableOrIntegerLiteral::parse(token);
        let op = token.next();
        if let Some(op) = op {
            let var_b = VariableOrIntegerLiteral::parse(token);
            match op {
                "+" => ArithmeticStatement::Addition(var_a, var_b),
                "-" => ArithmeticStatement::Substraction(var_a, var_b),
                "*" => ArithmeticStatement::Multiplication(var_a, var_b),
                "/" => ArithmeticStatement::Division(var_a, var_b),
                _ => panic!()
            }
        } else {
            ArithmeticStatement::Value(var_a)
        }
    }
    fn eval(&self, variables: &Variables) -> Integer {
        use ArithmeticStatement::*;
        match self {
            Value(val) => val.eval(variables),
            Addition(var_a, var_b) => var_a.eval(variables) + var_b.eval(variables),
            Substraction(var_a, var_b) => var_a.eval(variables) - var_b.eval(variables),
            Division(var_a, var_b) => var_a.eval(variables) / var_b.eval(variables),
            Multiplication(var_a, var_b) => var_a.eval(variables) * var_b.eval(variables),
        }
    }
}


#[derive(Debug)]
enum Condition {
    Equal(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    NotEqual(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    GreaterThan(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    GreaterThanOrEqual(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    LowerThan(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
    LowerThanOrEqual(VariableOrIntegerLiteral, VariableOrIntegerLiteral),
}

impl Condition {
    fn parse(token: &mut SplitAsciiWhitespace) -> Condition {
        use Condition::*;

        let var_a = VariableOrIntegerLiteral::parse(token);
        let op = token.next().unwrap();
        let var_b = VariableOrIntegerLiteral::parse(token);
        match op {
            "=" => Equal(var_a, var_b),
            ">" => GreaterThan(var_a, var_b),
            ">=" => GreaterThanOrEqual(var_a, var_b),
            "<" => LowerThan(var_a, var_b),
            "<=" => LowerThanOrEqual(var_a, var_b),
            "<>" => NotEqual(var_a, var_b),
            _ => panic!()
        }
    }
    fn eval(&self, variables: &Variables) -> bool {
        use Condition::*;
        match self {
            Equal(var_a, var_b) => var_a.eval(variables) == var_b.eval(variables),
            NotEqual(var_a, var_b) => var_a.eval(variables) != var_b.eval(variables),
            GreaterThan(var_a, var_b) => var_a.eval(variables) > var_b.eval(variables),
            GreaterThanOrEqual(var_a, var_b) => var_a.eval(variables) >= var_b.eval(variables),
            LowerThan(var_a, var_b) => var_a.eval(variables) < var_b.eval(variables),
            LowerThanOrEqual(var_a, var_b) => var_a.eval(variables) <= var_b.eval(variables),
        }
    }
}

#[derive(Debug)]
enum VariableOrStringLiteral {
    Variable(VariableName),
    StringLiteral(String),
}
impl VariableOrStringLiteral {
    fn parse(line: &String, token: &mut SplitAsciiWhitespace) -> VariableOrStringLiteral {
        use VariableOrStringLiteral::*;
        let t = token.next().unwrap().to_string(); 
        if t.starts_with("\"") {
            let st = line.find("\"").unwrap();
            let ed = line.rfind("\"").unwrap();
            let res = &line[st+1..ed];
            StringLiteral(res.to_string())
        } else {
            Variable(t)
        }
    }
    fn eval(&self, variables: &Variables) -> String {
        use VariableOrStringLiteral::*;
        match self {
            Variable(var) => variables.get(var).unwrap().to_string(),
            StringLiteral(s) => s.clone()
        }
    }
}

#[derive(Debug)]
enum Statement {
    Assignment(VariableName, ArithmeticStatement),
    Branching(Condition, Label),
    Print(VariableOrStringLiteral),
    Println(VariableOrStringLiteral),
}

impl Statement {
    fn parse_assignment(token: &mut SplitAsciiWhitespace) -> Statement {
        let variable = token.next().unwrap();
        let _ = token.next().unwrap(); // =
        let arithmetic = ArithmeticStatement::parse(token);
        Statement::Assignment(variable.to_string(), arithmetic)
    }

    fn parse_branching(token: &mut SplitAsciiWhitespace) -> Statement {
        let condition = Condition::parse(token);
        token.next();
        token.next();
        let label = parse_label(token);
        Statement::Branching(condition, label)
    }

    fn parse_print(line: &String, token: &mut SplitAsciiWhitespace) -> Statement {
        Statement::Print(VariableOrStringLiteral::parse(line, token))
    }

    fn parse_println(line: &String, token: &mut SplitAsciiWhitespace) -> Statement {
        Statement::Println(VariableOrStringLiteral::parse(line, token))
    }

    fn parse(line: &String, token: &mut SplitAsciiWhitespace) -> Statement {
        match token.next().unwrap() {
            "LET" => Statement::parse_assignment(token),
            "IF" => Statement::parse_branching(token),
            "PRINT" => Statement::parse_print(line, token),
            "PRINTLN" => Statement::parse_println(line, token),
            _ => panic!()
        }
    }

    fn eval<W: io::Write>(&self, out: &mut W, variables: &mut Variables) -> Option<Label> {
        use Statement::*;
        match self {
            Assignment(var_a, st) => {
                variables.insert(var_a.clone(), st.eval(&variables));
                None
            },
            Branching(cond, goto) => {
                if cond.eval(&variables) { Some(*goto) }
                else { None }
            },
            Print(var) => {
                write!(out, "{}", var.eval(&variables)).ok();
                None
            }
            Println(var) => {
                writeln!(out, "{}", var.eval(&variables)).ok();
                None
            }
        }
    }

}

fn parse_label(token: &mut SplitAsciiWhitespace) -> Label {
    token.next().unwrap().parse::<i32>().unwrap()
}

type Variables = HashMap<VariableName, Integer>;

fn run<W: io::Write>(out: &mut W, statements: &Statements, next_labels: &HashMap<Label, Label>, cur_label: Label, last_label: Label) {
    let mut cur_label = cur_label;
    let mut variables: Variables = HashMap::new();
    loop {
        let cur_statement = statements.get(&cur_label).unwrap();
        let next_label = cur_statement.eval(out, &mut variables);
        if cur_label == last_label { break; }
        cur_label = match next_label {
            None => *next_labels.get(&cur_label).unwrap(),
            Some(next) => next
        }
    }
}

fn solve<R: io::BufRead, W: io::Write>(scan: &mut UnsafeScanner<R>, out: &mut W) {
    let mut statements: Statements = HashMap::new();
    let mut next_labels: HashMap<Label, Label> = HashMap::new();
    let mut labels: Vec<Label> = Vec::new();
    loop {
        let line = scan.line();
        if line.is_none() { break; }
        let line = line.unwrap();
        let mut token = line.split_ascii_whitespace().into_iter();
        let label = parse_label(&mut token);
        let statement = Statement::parse(&line, &mut token);
        statements.insert(label, statement);
        labels.push(label);
    }
    labels.sort();
    for i in 0..labels.len() - 1 {
        next_labels.insert(labels[i], labels[i+1]);
    }
    run(out, &statements, &next_labels, labels[0], labels[labels.len() - 1]);
}
