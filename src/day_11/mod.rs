use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, newline},
    character::complete::{u16, u64},
    multi::{count, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct Monkey {
    worries: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
enum OperationModifier {
    Add,
    Mul,
}

#[derive(Debug)]
enum OperationTarget {
    Worry,
    Int(u64),
}

#[derive(Debug)]
struct Operation {
    modifier: OperationModifier,
    target: OperationTarget,
}

impl Operation {
    fn evaluate(&self, old: u64) -> u64 {
        let other = match self.target {
            OperationTarget::Worry => old,
            OperationTarget::Int(val) => val,
        };

        match self.modifier {
            OperationModifier::Add => old + other,
            OperationModifier::Mul => old * other,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    true_target: usize,
    false_target: usize,
}

fn starting_items(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("  Starting items: "), separated_list1(tag(", "), u64))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    fn operation_target_worry(input: &str) -> IResult<&str, OperationTarget> {
        tag("old").map(|_| OperationTarget::Worry).parse(input)
    }

    fn operation_target_int(input: &str) -> IResult<&str, OperationTarget> {
        u64.map(OperationTarget::Int).parse(input)
    }

    fn operation_modifier_add(input: &str) -> IResult<&str, OperationModifier> {
        char('+').map(|_| OperationModifier::Add).parse(input)
    }

    fn operation_modifier_mul(input: &str) -> IResult<&str, OperationModifier> {
        char('*').map(|_| OperationModifier::Mul).parse(input)
    }

    let rest = input;
    let (rest, _) = tag("  Operation: new = old ")(rest)?;
    let (rest, operation_modifier) = alt((operation_modifier_add, operation_modifier_mul))(rest)?;
    let (rest, _) = char(' ')(rest)?;
    let (rest, operation_target) = alt((operation_target_int, operation_target_worry))(rest)?;

    let result = Operation {
        modifier: operation_modifier,
        target: operation_target,
    };

    Ok((rest, result))
}

fn test(input: &str) -> IResult<&str, Test> {
    let rest = input;
    let (rest, divisible_by) = delimited(tag("  Test: divisible by "), u64, newline)(rest)?;
    let (rest, true_target) = delimited(tag("    If true: throw to monkey "), u16, newline)
        .map(usize::from)
        .parse(rest)?;
    let (rest, false_target) = preceded(tag("    If false: throw to monkey "), u16)
        .map(usize::from)
        .parse(rest)?;

    let result = Test {
        divisible_by,
        true_target,
        false_target,
    };

    Ok((rest, result))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let rest = input;
    let (rest, _) = tuple((tag("Monkey "), u64, tag(":"), newline))(rest)?;
    let (rest, starting_items) = terminated(starting_items, newline)(rest)?;
    let (rest, operation) = terminated(operation, newline)(rest)?;
    let (rest, test) = test(rest)?;

    let result = Monkey {
        worries: starting_items,
        operation,
        test,
    };

    Ok((rest, result))
}

fn problem_statement(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(count(newline, 2), monkey)(input)
}

fn parse_problem_statement(input: &str) -> Result<Vec<Monkey>> {
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

pub fn part_one(input: &str) -> Result<u64> {
    const NUM_ROUNDS: usize = 20;

    let mut monkeys = parse_problem_statement(input)?;
    let mut num_inspections: Vec<u64> = monkeys.iter().map(|_| 0).collect();

    for _ in 0..NUM_ROUNDS {
        for idx in 0..monkeys.len() {
            while let Some(inspected) = monkeys[idx].worries.pop() {
                // relief that item wasn't broken causes worry to be divided by 3 after inspection
                let new_worry = monkeys[idx].operation.evaluate(inspected) / 3;
                let new_idx = if new_worry % monkeys[idx].test.divisible_by == 0 {
                    monkeys[idx].test.true_target
                } else {
                    monkeys[idx].test.false_target
                };

                monkeys[new_idx].worries.push(new_worry);
                num_inspections[idx] += 1;
            }
        }
    }

    let mut inspections_heap = BinaryHeap::from(num_inspections);
    if inspections_heap.len() < 2 {
        return Err(anyhow!(
            "expected to have at least 2 monkeys with valid inspection values, but got {:?}",
            inspections_heap
        ));
    }

    // popping twice is safe because we have verified that the heap has at least two elements.
    Ok(inspections_heap.pop().unwrap() * inspections_heap.pop().unwrap())
}

pub fn part_two(input: &str) -> Result<u64> {
    const NUM_ROUNDS: usize = 10_000;

    let mut monkeys = parse_problem_statement(input)?;
    let mut num_inspections: Vec<u64> = monkeys.iter().map(|_| 0).collect();

    //
    let supermodulo: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();

    for _ in 0..NUM_ROUNDS {
        for idx in 0..monkeys.len() {
            while let Some(inspected) = monkeys[idx].worries.pop() {
                // Worry level is no longer divided by 3 after inspection.
                let new_worry = monkeys[idx].operation.evaluate(inspected % supermodulo);
                let new_idx = if new_worry % monkeys[idx].test.divisible_by == 0 {
                    monkeys[idx].test.true_target
                } else {
                    monkeys[idx].test.false_target
                };

                monkeys[new_idx].worries.push(new_worry);
                num_inspections[idx] += 1;
            }
        }
    }

    let mut inspections_heap = BinaryHeap::from(num_inspections);
    if inspections_heap.len() < 2 {
        return Err(anyhow!(
            "expected to have at least 2 monkeys with valid inspection values, but got {:?}",
            inspections_heap
        ));
    }

    // popping twice is safe because we have verified that the heap has at least two elements.
    Ok(inspections_heap.pop().unwrap() * inspections_heap.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 10_605);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 90_294);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 2_713_310_158);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 18_170_818_354);
    }
}
