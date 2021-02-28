#![allow(warnings)]

use std::{
    fmt::{
        Display
    },
    io::{
        self,
        BufRead
    }
};

trait CodeforcesSolution {
    type Parsed: Clone;
    type Answer: Display + Clone;

    fn new() -> Self;

    fn parse(&self, input: String) -> Self::Parsed;

    fn solve(&self, parsed: Self::Parsed) -> Self::Answer;
}

struct CodeforcesProblem1A;

impl CodeforcesSolution for CodeforcesProblem1A {
    type Parsed = Vec<u64>;
    type Answer = u64;

    fn new() -> Self {
        CodeforcesProblem1A
    }

    fn parse(&self, input: String) -> Self::Parsed {
        input.split(' ').map(|string| string.parse::<u64>().unwrap()).collect()
    }

    fn solve(&self, parsed: Self::Parsed) -> Self::Answer {
        let new_iter = parsed.iter().map(|unsigned| *unsigned as f64).collect::<Vec<_>>();

        ((new_iter[0] / new_iter[2]).ceil() * (new_iter[1] / new_iter[2]).ceil()) as u64
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.lock().read_line(&mut buffer)?;

    let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
    buffer.truncate(len);

    let runner = CodeforcesProblem1A::new();

    let parsed = runner.parse(buffer);
    let answer = runner.solve(parsed);

    println!("{}", answer);

    Ok(())
}
