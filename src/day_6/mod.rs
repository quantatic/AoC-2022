use std::collections::{btree_map::Entry, BTreeMap};

use anyhow::{anyhow, Error, Result};

pub const INPUT: &str = include_str!("./input");

fn calculate_solution(input: &str, num_needed: usize) -> Result<u32> {
    let chars = input.chars().collect::<Vec<_>>();
    let mut seen: BTreeMap<char, u32> = BTreeMap::new();
    for c in chars[..num_needed].iter().copied() {
        *seen.entry(c).or_insert(0) += 1;
    }

    for (i, c) in chars[num_needed..].into_iter().copied().enumerate() {
        *seen.entry(c).or_insert(0) += 1;
        match seen.entry(chars[i]) {
            Entry::Occupied(mut occupied) => {
                *occupied.get_mut() -= 1;
                if *occupied.get() == 0 {
                    occupied.remove();
                }
            }
            Entry::Vacant(_) => return Err(anyhow!("expected to find character {:?} at index {} leaving sliding window in map, but found nothing", chars[i], i)),
        };

        if seen.len() == num_needed {
            return u32::try_from(i + num_needed + 1).map_err(Error::from);
        }
    }

    Err(anyhow!(
        "exhaused entire input stream without finding signal"
    ))
}

pub fn part_one(input: &str) -> Result<u32> {
    const NUM_DISTINCT: usize = 4;

    calculate_solution(input, NUM_DISTINCT)
}

pub fn part_two(input: &str) -> Result<u32> {
    const NUM_DISTINCT: usize = 14;

    calculate_solution(input, NUM_DISTINCT)
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
