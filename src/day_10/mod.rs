use anyhow::{anyhow, Result};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i8, character::complete::newline,
    multi::separated_list1, sequence::preceded, IResult, Parser,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Nop;
#[derive(Debug)]
struct Addx(i8);

#[derive(Debug)]
enum Instruction {
    Nop(Nop),
    Addx(Addx),
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    fn nop(input: &str) -> IResult<&str, Nop> {
        tag("noop").map(|_| Nop).parse(input)
    }

    fn addx(input: &str) -> IResult<&str, Addx> {
        preceded(tag("addx "), i8).map(Addx).parse(input)
    }

    alt((nop.map(Instruction::Nop), addx.map(Instruction::Addx)))(input)
}

fn problem_statement(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, instruction)(input)
}

fn parse_problem_statement(input: &str) -> Result<Vec<Instruction>> {
    let (rest, problem_statement) =
        problem_statement(input).map_err(|err| err.map_input(str::to_string))?;

    if !rest.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            rest
        ));
    }

    Ok(problem_statement)
}

pub fn part_one(input: &str) -> Result<i32> {
    let problem_statement = parse_problem_statement(input)?;

    let mut x = 1;
    let mut cycle_values = Vec::new();
    cycle_values.push(x);
    for instruction in problem_statement {
        match instruction {
            Instruction::Addx(Addx(val)) => {
                cycle_values.push(x);
                cycle_values.push(x);
                x += val;
            }
            Instruction::Nop(_) => {
                cycle_values.push(x);
            }
        }
    }

    let mut result = 0;
    for (i, val) in cycle_values.into_iter().enumerate() {
        if i >= 20 && ((i - 20) % 40) == 0 {
            result += i32::try_from(i)? * i32::from(val);
        }
    }

    Ok(result)
}

pub const CRT_WIDTH: usize = 40;
pub const CRT_HEIGHT: usize = 6;

pub fn part_two(input: &str) -> Result<[[bool; CRT_WIDTH]; CRT_HEIGHT]> {
    let problem_statement = parse_problem_statement(input)?;
    let mut crt = [[false; CRT_WIDTH]; CRT_HEIGHT];

    let mut x = 1;
    let mut cycle_values = Vec::new();
    for instruction in problem_statement {
        match instruction {
            Instruction::Addx(Addx(val)) => {
                cycle_values.push(x);
                cycle_values.push(x);
                x += val;
            }
            Instruction::Nop(_) => {
                cycle_values.push(x);
            }
        }
    }

    for (i, val) in cycle_values.into_iter().enumerate() {
        if val < 0 {
            continue;
        }

        let x_val = i % CRT_WIDTH;
        let y_val = i / CRT_WIDTH;

        if usize::abs_diff(x_val, usize::try_from(val)?) <= 1 {
            crt[y_val][x_val] = true;
        }
    }

    Ok(crt)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 13_140);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 16_060);
    }

    // ##..##..##..##..##..##..##..##..##..##..
    // ###...###...###...###...###...###...###.
    // ####....####....####....####....####....
    // #####.....#####.....#####.....#####.....
    // ######......######......######......####
    // #######.......#######.......#######.....
    #[test]
    fn test_part_two() {
        const EXPECTED_RESULT: [[bool; CRT_WIDTH]; CRT_HEIGHT] = [
            [
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, false, false, true, true, false, false, true, true,
                false, false,
            ],
            [
                true, true, true, false, false, false, true, true, true, false, false, false, true,
                true, true, false, false, false, true, true, true, false, false, false, true, true,
                true, false, false, false, true, true, true, false, false, false, true, true, true,
                false,
            ],
            [
                true, true, true, true, false, false, false, false, true, true, true, true, false,
                false, false, false, true, true, true, true, false, false, false, false, true,
                true, true, true, false, false, false, false, true, true, true, true, false, false,
                false, false,
            ],
            [
                true, true, true, true, true, false, false, false, false, false, true, true, true,
                true, true, false, false, false, false, false, true, true, true, true, true, false,
                false, false, false, false, true, true, true, true, true, false, false, false,
                false, false,
            ],
            [
                true, true, true, true, true, true, false, false, false, false, false, false, true,
                true, true, true, true, true, false, false, false, false, false, false, true, true,
                true, true, true, true, false, false, false, false, false, false, true, true, true,
                true,
            ],
            [
                true, true, true, true, true, true, true, false, false, false, false, false, false,
                false, true, true, true, true, true, true, true, false, false, false, false, false,
                false, false, true, true, true, true, true, true, true, false, false, false, false,
                false,
            ],
        ];
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, EXPECTED_RESULT);
    }

    // BACEKLHF
    #[test]
    fn solution_part_two() {
        const EXPECTED_RESULT: [[bool; CRT_WIDTH]; CRT_HEIGHT] = [
            [
                true, true, true, false, false, false, true, true, false, false, false, true, true,
                false, false, true, true, true, true, false, true, false, false, true, false, true,
                false, false, false, false, true, false, false, true, false, true, true, true,
                true, false,
            ],
            [
                false, false, false, true, false, true, false, false, true, false, true, false,
                false, true, false, true, false, false, false, false, true, false, true, false,
                false, true, false, false, false, false, true, false, false, true, false, true,
                false, false, false, false,
            ],
            [
                true, true, true, false, false, true, false, false, true, false, true, false,
                false, false, false, true, true, true, false, false, true, true, false, false,
                false, true, false, false, false, false, true, true, true, true, false, true, true,
                true, false, false,
            ],
            [
                false, false, false, true, false, true, true, true, true, false, true, false,
                false, false, false, true, false, false, false, false, true, false, true, false,
                false, true, false, false, false, false, true, false, false, true, false, true,
                false, false, false, false,
            ],
            [
                true, false, false, true, false, true, false, false, true, false, true, false,
                false, true, false, true, false, false, false, false, true, false, true, false,
                false, true, false, false, false, false, true, false, false, true, false, true,
                false, false, false, false,
            ],
            [
                true, true, true, false, false, true, false, false, true, false, false, true, true,
                false, false, true, true, true, true, false, true, false, false, true, false, true,
                true, true, true, false, true, false, false, true, false, true, false, false,
                false, false,
            ],
        ];
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, EXPECTED_RESULT);
    }
}
