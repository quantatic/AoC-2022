use anyhow::{anyhow, Result};

use std::collections::BinaryHeap;

use nom::{
    character::complete::{newline, u32},
    combinator::{map, opt},
    multi::{count, separated_list1},
    sequence::terminated,
    IResult,
};

pub const INPUT: &str = include_str!("./input");

fn single_calorie_count(input: &str) -> IResult<&str, u32> {
    map(separated_list1(newline, u32), |list| list.into_iter().sum())(input)
}

fn calories_list(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(
        separated_list1(count(newline, 2), single_calorie_count),
        opt(newline),
    )(input)
}

fn parse_calories_list(input: &str) -> Result<Vec<u32>> {
    let (leftover, result) = calories_list(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

pub fn part_one(input: &str) -> Result<u32> {
    let elf_calories = parse_calories_list(input)?;

    Ok(elf_calories
        .into_iter()
        .max()
        .expect("unexpected empty list of elf calories"))
}

pub fn part_two(input: &str) -> Result<u32> {
    const NUM_ITEMS: usize = 3;

    let elf_calories = parse_calories_list(input)?;

    let mut heap = elf_calories.into_iter().collect::<BinaryHeap<_>>();
    let mut result = 0;
    for _ in 0..NUM_ITEMS {
        result += heap
            .pop()
            .ok_or_else(|| anyhow!("couldn't get enough top elements from calories list"))?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 24_000);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 45_000);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 69_626);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 206_780);
    }
}
