use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{anychar, char, line_ending, newline, u8},
    combinator::value,
    multi::{count, separated_list1},
    sequence::{delimited, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Move {
    start_idx: usize,
    end_idx: usize,
    count: usize,
}

#[derive(Debug)]
struct ProblemStatement {
    stacks: Vec<Vec<char>>, // [stack_idx][height], with 0-indexed height
    moves: Vec<Move>,
}

pub const INPUT: &str = include_str!("./input");

fn parse_box(input: &str) -> IResult<&str, Option<char>> {
    alt((
        delimited(char('['), anychar, char(']')).map(Option::from),
        value(None, tag("   ")),
    ))(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (rest, lines) = separated_list1(newline, separated_list1(char(' '), parse_box))(input)?;

    let mut stacks = vec![Vec::new(); lines[0].len()];

    for line in lines {
        if line.len() != stacks.len() {
            panic!();
        }

        for (i, val) in line.into_iter().enumerate() {
            if let Some(c) = val {
                stacks[i].insert(0, c);
            }
        }
    }

    let (rest, _) = newline(rest)?;

    Ok((rest, stacks))
}

fn parse_stacks_labels(input: &str) -> IResult<&str, ()> {
    let rest = input;
    let (rest, _) = is_not("\n\r")(rest)?;
    let (rest, _) = count(line_ending, 2)(rest)?;

    Ok((rest, ()))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, (_, count, _, start_idx, _, end_idx)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;

    let result_move = Move {
        count: usize::from(count),
        start_idx: usize::from(start_idx),
        end_idx: usize::from(end_idx),
    };

    Ok((rest, result_move))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, parse_move)(input)
}

fn parse_problem_statement(input: &str) -> Result<ProblemStatement> {
    let rest = input;
    let (rest, stacks) = parse_stacks(rest).map_err(|err| err.map_input(str::to_string))?;
    let (rest, _) = parse_stacks_labels(rest).map_err(|err| err.map_input(str::to_string))?;
    let (rest, moves) = parse_moves(rest).map_err(|err| err.map_input(str::to_string))?;

    if !rest.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            rest
        ));
    }

    Ok(ProblemStatement { moves, stacks })
}

pub fn part_one(input: &str) -> Result<String> {
    let problem = parse_problem_statement(input)?;

    let mut stacks = problem.stacks;
    for problem_move in problem.moves {
        for _ in 0..problem_move.count {
            let moved = stacks[problem_move.start_idx - 1].pop().ok_or_else(|| {
                anyhow!(
                    "attempted to move from empty stack {}: {:?}",
                    problem_move.start_idx,
                    stacks
                )
            })?;
            stacks[problem_move.end_idx - 1].push(moved);
        }
    }

    let solution = stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>();

    Ok(solution)
}

pub fn part_two(input: &str) -> Result<String> {
    let problem = parse_problem_statement(input)?;

    let mut stacks = problem.stacks;
    for problem_move in problem.moves {
        let mut scratch = Vec::new();
        for _ in 0..problem_move.count {
            let moved = stacks[problem_move.start_idx - 1].pop().ok_or_else(|| {
                anyhow!(
                    "attempted to move from empty stack {}: {:?}",
                    problem_move.start_idx,
                    stacks
                )
            })?;
            scratch.push(moved);
        }

        while let Some(moved) = scratch.pop() {
            stacks[problem_move.end_idx - 1].push(moved);
        }
    }

    let solution = stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect::<String>();

    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "BWNCQRMDB");
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, "MCD");
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "NHWZCBNBF");
    }
}
