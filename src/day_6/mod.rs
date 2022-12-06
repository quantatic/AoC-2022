use std::collections::{BTreeSet, HashSet, VecDeque};

use anyhow::{anyhow, Error, Result};


pub const INPUT: &str = include_str!("./input");

pub fn part_one(input: &str) -> Result<u32> {
    const NUM_DISTINCT: usize = 4;

    let mut chars = input.chars().collect::<Vec<_>>();
    let mut seen = chars.drain(0..(NUM_DISTINCT - 1)).collect::<VecDeque<_>>();

    for (i, c) in chars.into_iter().enumerate() {
        seen.push_back(c);

        if seen.iter().collect::<BTreeSet<_>>().len() == NUM_DISTINCT {
            return u32::try_from(i + NUM_DISTINCT).map_err(Error::from);
        }

        seen.pop_front();
    }

    Err(anyhow!(
        "exhaused entire input stream without finding signal"
    ))
}

pub fn part_two(input: &str) -> Result<u32> {
    const NUM_DISTINCT: usize = 14;

    let mut chars = input.chars().collect::<Vec<_>>();
    let mut seen = chars.drain(0..(NUM_DISTINCT - 1)).collect::<VecDeque<_>>();

    for (i, c) in chars.into_iter().enumerate() {
        seen.push_back(c);

        if seen.iter().collect::<HashSet<_>>().len() == NUM_DISTINCT {
            return u32::try_from(i + NUM_DISTINCT).map_err(Error::from);
        }

        seen.pop_front();
    }

    Err(anyhow!(
        "exhaused entire input stream without finding signal"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase<'a> {
        input: &'a str,
        expected_result: u32,
    }

    #[test]
    fn test_part_one() {
        const TEST_CASES: &[TestCase] = &[
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected_result: 7,
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected_result: 5,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected_result: 6,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected_result: 10,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected_result: 11,
            },
        ];

        for case in TEST_CASES {
            let result = part_one(case.input).unwrap();
            assert_eq!(result, case.expected_result, "{case:?}");
        }
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 1_658);
    }

    #[test]
    fn test_part_two() {
        const TEST_CASES: &[TestCase] = &[
            TestCase {
                input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
                expected_result: 19,
            },
            TestCase {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected_result: 23,
            },
            TestCase {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected_result: 23,
            },
            TestCase {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected_result: 29,
            },
            TestCase {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected_result: 26,
            },
        ];

        for case in TEST_CASES {
            let result = part_two(case.input).unwrap();
            assert_eq!(result, case.expected_result, "{case:?}");
        }
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 2_260);
    }
}
