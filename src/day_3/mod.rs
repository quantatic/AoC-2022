use std::collections::HashSet;

use anyhow::{anyhow, Result};

pub const INPUT: &str = include_str!("./input");

struct Rucksack(HashSet<char>, HashSet<char>);
struct ElfGroup(HashSet<char>, HashSet<char>, HashSet<char>);

fn get_priority(val: char) -> Result<u8> {
    let priority = match val {
        'a'..='z' => (val as u8) - ('a' as u8) + 1,
        'A'..='Z' => (val as u8) - ('A' as u8) + 27,
        _ => return Err(anyhow!("expected alphabetic character, but got {}", val)),
    };

    Ok(priority)
}

fn parse_part_one(input: &str) -> Result<Vec<Rucksack>> {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            if chars.len() % 2 != 0 {
                return Err(anyhow!(
                    "expected line {} to have length divisible by two",
                    line
                ));
            }

            let (first_half, second_half) = chars.split_at(chars.len() / 2);

            let compartment_one = first_half.iter().cloned().collect();
            let compartment_two = second_half.iter().cloned().collect();

            Ok(Rucksack(compartment_one, compartment_two))
        })
        .collect::<Result<Vec<_>>>()
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

fn parse_part_two(input: &str) -> Result<Vec<ElfGroup>> {
    let mut elves = input
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    if elves.len() % 3 != 0 {
        return Err(anyhow!(
            "expected number of elves to be divisible by 3, but got {} elves",
            elves.len()
        ));
    }

    let mut groups = Vec::new();
    for _ in 0..(elves.len() / 3) {
        groups.push(ElfGroup(
            elves.pop().unwrap(),
            elves.pop().unwrap(),
            elves.pop().unwrap(),
        ))
    }

    Ok(groups)
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
