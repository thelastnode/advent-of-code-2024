use std::io::stdin;

use day07::{read_input, Equation, Op};

fn process(input: &[Equation]) -> i64 {
    input
        .iter()
        .filter(|eq| eq.is_valid(&[Op::Add, Op::Multiply]))
        .map(|eq| eq.result)
        .sum()
}

fn main() {
    let input = read_input(stdin().lock());
    println!("{}", process(&input));
}
