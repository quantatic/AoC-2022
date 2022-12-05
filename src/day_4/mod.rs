use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct ElfAssignment {
    start: u8,
    end: u8,
}

#[derive(Debug)]
struct ElfAssignmentPair(ElfAssignment, ElfAssignment);

impl ElfAssignmentPair {
    fn fully_overlaps(&self) -> bool {
        let first_in_second = self.0.start >= self.1.start && self.0.end <= self.1.end;
        let second_in_first = self.0.start <= self.1.start && self.0.end >= self.1.end;

        first_in_second || second_in_first
    }

    fn has_overlap(&self) -> bool {
        self.0.start <= self.1.end && self.0.end >= self.1.start
    }
}

fn assignment(input: &str) -> IResult<&str, ElfAssignment> {
    map(separated_pair(u8, tag("-"), u8), |(start, end)| {
        ElfAssignment { start, end }
    })(input)
}

fn assignment_pair(input: &str) -> IResult<&str, ElfAssignmentPair> {
    map(
        separated_pair(assignment, tag(","), assignment),
        |(first, second)| ElfAssignmentPair(first, second),
    )(input)
}

fn full_parser(input: &str) -> IResult<&str, Vec<ElfAssignmentPair>> {
    terminated(separated_list1(newline, assignment_pair), opt(newline))(input)
}

fn parse_assignment_pairs(input: &str) -> Result<Vec<ElfAssignmentPair>> {
    let (leftover, result) = full_parser(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

pub fn part_one(input: &str) -> Result<u32> {
    let assignment_pairs = parse_assignment_pairs(input)?;

    u32::try_from(
        assignment_pairs
            .into_iter()
            .filter(ElfAssignmentPair::fully_overlaps)
            .count(),
    )
    .map_err(anyhow::Error::from)
}

pub fn part_two(input: &str) -> Result<u32> {
    let assignment_pairs = parse_assignment_pairs(input)?;

    u32::try_from(
        assignment_pairs
            .into_iter()
            .filter(ElfAssignmentPair::has_overlap)
            .count(),
    )
    .map_err(anyhow::Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 576);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 905);
    }
}
