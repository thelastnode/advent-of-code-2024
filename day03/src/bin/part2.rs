use std::io::{stdin, Read};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::anychar},
    multi::many0,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum AstNode {
    Multiply(i64, i64),
    Do,
    Dont,
    Junk,
}

fn parse_multiply(input: &str) -> IResult<&str, AstNode> {
    let parser = delimited(
        tag("mul("),
        separated_pair(character::complete::i64, tag(","), character::complete::i64),
        tag(")"),
    );

    parser
        .map(|(left, right)| AstNode::Multiply(left, right))
        .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<AstNode>> {
    let dos = tag("do()").map(|_| AstNode::Do);
    let donts = tag("don't()").map(|_| AstNode::Dont);
    let junk = anychar.map(|_| AstNode::Junk);

    let ast_node = alt((dos, donts, parse_multiply, junk));
    let mut parser = many0(ast_node);

    parser(input)
}

fn process(input: &str) -> Result<i64> {
    let (input, nodes) = parse(input).map_err(|e| e.to_owned())?;

    if !input.is_empty() {
        return Err(anyhow!("Failed to parse: {:?}", input));
    }

    let (result, _) = nodes
        .into_iter()
        .fold((0, true), |(acc, enabled), node| match node {
            AstNode::Multiply(left, right) if enabled => (acc + left * right, enabled),
            AstNode::Multiply(_, _) => (acc, enabled),
            AstNode::Do => (acc, true),
            AstNode::Dont => (acc, false),
            AstNode::Junk => (acc, enabled),
        });

    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    println!("{}", process(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AstNode::*;
    use super::*;

    #[test]
    fn test_parse() {
        let (input, nodes) = parse("xmul(2,3)do()xdon't()mul(4,5)").unwrap();
        assert!(input.is_empty());
        assert_eq!(
            nodes,
            vec![Junk, Multiply(2, 3), Do, Junk, Dont, Multiply(4, 5)]
        );
    }

    #[test]
    fn test_example() {
        let output =
            process("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                .unwrap();
        assert_eq!(output, 48);
    }
}
