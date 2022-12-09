use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct ProblemStatement {
    forest: Vec<Vec<u8>>, // map[y][x]
}

impl ProblemStatement {
    fn width(&self) -> usize {
        self.forest[0].len()
    }

    fn height(&self) -> usize {
        self.forest.len()
    }

    #[allow(dead_code)]
    fn saturating_up(&self, y: usize) -> usize {
        y.saturating_sub(1)
    }

    #[allow(dead_code)]
    fn saturating_left(&self, x: usize) -> usize {
        x.saturating_sub(1)
    }

    fn saturating_down(&self, y: usize) -> usize {
        usize::min(y + 1, self.height())
    }

    fn saturating_right(&self, x: usize) -> usize {
        usize::min(x + 1, self.width())
    }
}

fn map_row(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, row) = digit1(input)?;

    let result = row
        .chars()
        .map(|c| {
            // we are guaranteed that these characters will always be valid u8 digits, so parsing should always be possible
            c.to_digit(10).unwrap() as u8
        })
        .collect::<Vec<_>>();

    Ok((rest, result))
}

fn forest(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(newline, map_row)(input)
}

fn problem_statement(input: &str) -> IResult<&str, ProblemStatement> {
    let (rest, forest) = forest(input)?;

    let result = ProblemStatement { forest };

    Ok((rest, result))
}

fn parse_problem_statement(input: &str) -> Result<ProblemStatement> {
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
    #![allow(clippy::needless_range_loop)]

    let problem_statement = parse_problem_statement(input)?;
    let mut seen_map = vec![vec![false; problem_statement.width()]; problem_statement.height()];

    // left to right
    for y in 0..problem_statement.height() {
        let mut tallest = problem_statement.forest[y][0];
        seen_map[y][0] = true;
        for x in 0..problem_statement.width() {
            let current_height = problem_statement.forest[y][x];
            if current_height > tallest {
                seen_map[y][x] = true;
                tallest = current_height;
            }
        }
    }

    // right to left
    for y in 0..problem_statement.height() {
        let mut tallest = problem_statement.forest[y][problem_statement.width() - 1];
        seen_map[y][problem_statement.width() - 1] = true;
        for x in (0..problem_statement.width()).rev() {
            let current_height = problem_statement.forest[y][x];
            if current_height > tallest {
                seen_map[y][x] = true;
                tallest = current_height;
            }
        }
    }

    // top to bottom
    for x in 0..problem_statement.width() {
        let mut tallest = problem_statement.forest[0][x];
        seen_map[0][x] = true;
        for y in 0..problem_statement.height() {
            let current_height = problem_statement.forest[y][x];
            if current_height > tallest {
                seen_map[y][x] = true;
                tallest = current_height;
            }
        }
    }

    // bottom to top
    for x in 0..problem_statement.width() {
        let mut tallest = problem_statement.forest[problem_statement.height() - 1][x];
        seen_map[problem_statement.height() - 1][x] = true;
        for y in (0..problem_statement.height()).rev() {
            let current_height = problem_statement.forest[y][x];
            if current_height > tallest {
                seen_map[y][x] = true;
                tallest = current_height;
            }
        }
    }

    // count number of boolean "true" values in this 2d array.
    let num_seen = seen_map
        .into_iter()
        .map(|row| row.into_iter().filter(|val| *val).count())
        .sum::<usize>()
        .try_into()?;

    Ok(num_seen)
}

pub fn part_two(input: &str) -> Result<u32> {
    let problem_statement = parse_problem_statement(input)?;
    let mut best_score = 0;
    for y in 0..problem_statement.height() {
        for x in 0..problem_statement.width() {
            // tree to top
            let mut up_score = 0;
            let current_height = problem_statement.forest[y][x];
            for tmp_y in (0..y).rev() {
                up_score += 1;

                if current_height <= problem_statement.forest[tmp_y][x] {
                    break;
                }
            }

            // tree to bottom
            let mut down_score = 0;
            let current_height = problem_statement.forest[y][x];
            for tmp_y in problem_statement.saturating_down(y)..problem_statement.height() {
                down_score += 1;

                if current_height <= problem_statement.forest[tmp_y][x] {
                    break;
                }
            }

            // tree to left
            let mut left_score = 0;
            let current_height = problem_statement.forest[y][x];
            for tmp_x in (0..x).rev() {
                left_score += 1;

                if current_height <= problem_statement.forest[y][tmp_x] {
                    break;
                }
            }

            // tree to right
            let mut right_score = 0;
            let current_height = problem_statement.forest[y][x];
            for tmp_x in problem_statement.saturating_right(x)..problem_statement.width() {
                right_score += 1;

                if current_height <= problem_statement.forest[y][tmp_x] {
                    break;
                }
            }

            let this_score = left_score * right_score * up_score * down_score;

            if this_score > best_score {
                best_score = this_score;
            }
        }
    }

    Ok(best_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 1_669);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 331_344);
    }
}
