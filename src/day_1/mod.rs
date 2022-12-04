use anyhow::{anyhow, Result};
use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BinaryHeap;

pub const INPUT: &str = include_str!("./input");

fn get_calories_list(input: &str) -> Result<Vec<u32>> {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .split(|line| line.is_empty())
        .map(|grouping| {
            let values = grouping
                .iter()
                .map(|line| line.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok(values.into_iter().sum())
        })
        .collect::<Result<Vec<_>>>()
}

pub fn part_one(input: &str) -> Result<u32> {
    let elf_calories = get_calories_list(input)?;

    Ok(elf_calories
        .into_iter()
        .max()
        .expect("unexpected empty list of elf calories"))
}

pub fn part_two(input: &str) -> Result<u32> {
    const NUM_ITEMS: usize = 3;

    let elf_calories = get_calories_list(input)?;

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
