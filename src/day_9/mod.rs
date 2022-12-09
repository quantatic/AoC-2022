use std::collections::HashSet;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::u8,
    character::complete::{newline, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: u8,
}

#[derive(Debug)]
struct WorldState {
    nodes: Vec<(i32, i32)>,
    seen_tail: HashSet<(i32, i32)>,
}

impl WorldState {
    fn new(num_nodes: usize) -> Result<Self> {
        if num_nodes == 0 {
            return Err(anyhow!("cannot have a world state with no nodes"));
        }

        let result = Self {
            nodes: vec![(0, 0); num_nodes],
            seen_tail: HashSet::new(),
        };

        Ok(result)
    }

    fn head_up(&mut self) {
        self.nodes[0].1 += 1;

        for (head_idx, tail_idx) in (0..).zip(1..self.nodes.len()) {
            self.nodes[tail_idx] = Self::catch_up(self.nodes[head_idx], self.nodes[tail_idx]);
        }

        // empty nodes is an invalid world state
        self.seen_tail.insert(*self.nodes.last().unwrap());
    }

    fn head_down(&mut self) {
        self.nodes[0].1 -= 1;

        for (head_idx, tail_idx) in (0..).zip(1..self.nodes.len()) {
            self.nodes[tail_idx] = Self::catch_up(self.nodes[head_idx], self.nodes[tail_idx]);
        }

        // empty nodes is an invalid world state
        self.seen_tail.insert(*self.nodes.last().unwrap());
    }

    fn head_left(&mut self) {
        self.nodes[0].0 -= 1;

        for (head_idx, tail_idx) in (0..).zip(1..self.nodes.len()) {
            self.nodes[tail_idx] = Self::catch_up(self.nodes[head_idx], self.nodes[tail_idx]);
        }

        // empty nodes is an invalid world state
        self.seen_tail.insert(*self.nodes.last().unwrap());
    }

    fn head_right(&mut self) {
        self.nodes[0].0 += 1;

        for (head_idx, tail_idx) in (0..).zip(1..self.nodes.len()) {
            self.nodes[tail_idx] = Self::catch_up(self.nodes[head_idx], self.nodes[tail_idx]);
        }

        // empty nodes is an invalid world state
        self.seen_tail.insert(*self.nodes.last().unwrap());
    }

    fn catch_up(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        let diff_x = head.0 - tail.0;
        let diff_y = head.1 - tail.1;

        let mut result_x = tail.0;
        let mut result_y = tail.1;

        // if moved far enough away, have tail catch up to head
        if diff_x.abs() > 1 || diff_y.abs() > 1 {
            result_x += diff_x.signum();
            result_y += diff_y.signum();
        }

        (result_x, result_y)
    }

    fn num_seen_tail_positions(&self) -> usize {
        self.seen_tail.len()
    }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (rest, dir_char) = one_of("RULD")(input)?;

    let result = match dir_char {
        'R' => Direction::Right,
        'U' => Direction::Up,
        'L' => Direction::Left,
        'D' => Direction::Down,
        _ => unreachable!("unexpected direction character {dir_char:?}"),
    };

    Ok((rest, result))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, (direction, amount)) = separated_pair(direction, tag(" "), u8)(input)?;

    let result = Move { direction, amount };

    Ok((rest, result))
}

fn problem_statement(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(newline, parse_move)(input)
}

fn parse_problem_statement(input: &str) -> Result<Vec<Move>> {
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

pub fn part_one(input: &str) -> Result<u32> {
    const NUM_NODES: usize = 2;

    let moves = parse_problem_statement(input)?;
    let mut world_state = WorldState::new(NUM_NODES)?;

    for this_move in moves {
        let move_function = match this_move.direction {
            Direction::Down => WorldState::head_down,
            Direction::Left => WorldState::head_left,
            Direction::Right => WorldState::head_right,
            Direction::Up => WorldState::head_up,
        };

        (0..this_move.amount).for_each(|_| move_function(&mut world_state));
    }

    let result = world_state.num_seen_tail_positions().try_into()?;

    Ok(result)
}

pub fn part_two(input: &str) -> Result<u32> {
    const NUM_NODES: usize = 10;

    let moves = parse_problem_statement(input)?;
    let mut world_state = WorldState::new(NUM_NODES)?;

    for this_move in moves {
        let move_function = match this_move.direction {
            Direction::Down => WorldState::head_down,
            Direction::Left => WorldState::head_left,
            Direction::Right => WorldState::head_right,
            Direction::Up => WorldState::head_up,
        };

        (0..this_move.amount).for_each(|_| move_function(&mut world_state));
    }

    let result = world_state.num_seen_tail_positions().try_into()?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PART_1_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_PART_2_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_PART_1_INPUT).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 5_878);
    }

    #[test]
    fn test_part_two_small() {
        let result = part_two(TEST_PART_1_INPUT).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_large() {
        let result = part_two(TEST_PART_2_INPUT).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 2_405);
    }
}
