use std::collections::HashSet;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{alpha1, char, newline},
    combinator::{map, opt, verify},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

pub const INPUT: &str = include_str!("./input");

struct Rucksack(HashSet<char>, HashSet<char>);
struct ElfGroup(HashSet<char>, HashSet<char>, HashSet<char>);

fn get_priority(val: char) -> Result<u8> {
    let priority = match val {
        'a'..='z' => (val as u8) - b'a' + 1,
        'A'..='Z' => (val as u8) - b'A' + 27,
        _ => return Err(anyhow!("expected alphabetic character, but got {}", val)),
    };

    Ok(priority)
}

fn rucksack(input: &str) -> IResult<&str, Rucksack> {
    map(
        verify(alpha1, |s: &str| s.chars().count() % 2 == 0),
        |s: &str| {
            let chars = s.chars().collect::<Vec<_>>();
            let (first_half, second_half) = chars.split_at(chars.len() / 2);

            let compartment_one = first_half.iter().copied().collect();
            let compartment_two = second_half.iter().copied().collect();

            Rucksack(compartment_one, compartment_two)
        },
    )(input)
}

fn full_parser_part_one(input: &str) -> IResult<&str, Vec<Rucksack>> {
    terminated(separated_list1(newline, rucksack), opt(newline))(input)
}

fn parse_part_one(input: &str) -> Result<Vec<Rucksack>> {
    let (leftover, result) =
        full_parser_part_one(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

pub fn part_one(input: &str) -> Result<u32> {
    let line_values = parse_part_one(input)?
        .into_iter()
        .map(|line| {
            let in_common = (&line.0) & (&line.1);

            if in_common.len() != 1 {
                return Err(anyhow!(
                    "expected to have exactly one character in common, but got {:?}",
                    in_common
                ));
            }

            let common_chars = in_common.into_iter().collect::<Vec<_>>();
            let common_char = common_chars[0];

            get_priority(common_char)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(line_values.into_iter().map(u32::from).sum())
}

fn elf_group(input: &str) -> IResult<&str, ElfGroup> {
    map(
        tuple((alpha1, newline, alpha1, newline, alpha1)),
        |(elf_one, _, elf_two, _, elf_three): (&str, _, &str, _, &str)| {
            ElfGroup(
                elf_one.chars().collect(),
                elf_two.chars().collect(),
                elf_three.chars().collect(),
            )
        },
    )(input)
}

fn full_parser_part_two(input: &str) -> IResult<&str, Vec<ElfGroup>> {
    separated_list1(char('\n'), elf_group)(input)
}

fn parse_part_two(input: &str) -> Result<Vec<ElfGroup>> {
    let (leftover, result) =
        full_parser_part_two(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

pub fn part_two(input: &str) -> Result<u32> {
    let group_values = parse_part_two(input)?
        .into_iter()
        .map(|group| {
            let in_common = (&group.0) & (&group.1);
            let in_common = (&in_common) & (&group.2);

            if in_common.len() != 1 {
                return Err(anyhow!(
                    "expected to have exactly one character in common, but got {:?}",
                    in_common
                ));
            }

            let common_chars = in_common.into_iter().collect::<Vec<_>>();
            let common_char = common_chars[0];

            get_priority(common_char)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(group_values.into_iter().map(u32::from).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 157);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 7_889);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 70);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 2_825);
    }
}
