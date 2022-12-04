use anyhow::{anyhow, Result};
use nom::{
    character::{
        complete::{newline, one_of},
        streaming::char,
    },
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug)]
struct GameRound {
    my_choice: Choice,
    opponent_choice: Choice,
}

struct RoundPartOne(GameRound);

#[derive(Clone, Copy, Debug)]
struct RoundPartTwo {
    opponent_choice: Choice,
    needed_result: GameRoundResult,
}

impl RoundPartTwo {
    fn calculate_my_choice(&self) -> Choice {
        match (self.opponent_choice, self.needed_result) {
            (Choice::Rock, GameRoundResult::Winner(Player::Opponent)) => Choice::Scissors,
            (Choice::Paper, GameRoundResult::Winner(Player::Opponent)) => Choice::Rock,
            (Choice::Scissors, GameRoundResult::Winner(Player::Opponent)) => Choice::Paper,
            (Choice::Rock, GameRoundResult::Tie) => Choice::Rock,
            (Choice::Paper, GameRoundResult::Tie) => Choice::Paper,
            (Choice::Scissors, GameRoundResult::Tie) => Choice::Scissors,
            (Choice::Rock, GameRoundResult::Winner(Player::Me)) => Choice::Paper,
            (Choice::Paper, GameRoundResult::Winner(Player::Me)) => Choice::Scissors,
            (Choice::Scissors, GameRoundResult::Winner(Player::Me)) => Choice::Rock,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Player {
    Me,
    Opponent,
}

#[derive(Clone, Copy, Debug)]
enum GameRoundResult {
    Winner(Player),
    Tie,
}

impl GameRound {
    fn calculate_score(&self) -> u32 {
        const LOSE_SCORE: u32 = 0;
        const TIE_SCORE: u32 = 3;
        const WIN_SCORE: u32 = 6;

        const ROCK_SCORE: u32 = 1;
        const PAPER_SCORE: u32 = 2;
        const SCISSORS_SCORE: u32 = 3;

        let result_score = match self.get_result() {
            GameRoundResult::Winner(Player::Opponent) => LOSE_SCORE,
            GameRoundResult::Tie => TIE_SCORE,
            GameRoundResult::Winner(Player::Me) => WIN_SCORE,
        };

        let choice_score = match self.my_choice {
            Choice::Rock => ROCK_SCORE,
            Choice::Paper => PAPER_SCORE,
            Choice::Scissors => SCISSORS_SCORE,
        };

        result_score + choice_score
    }

    fn get_result(&self) -> GameRoundResult {
        match (self.my_choice, self.opponent_choice) {
            (Choice::Rock, Choice::Rock) => GameRoundResult::Tie,
            (Choice::Paper, Choice::Paper) => GameRoundResult::Tie,
            (Choice::Scissors, Choice::Scissors) => GameRoundResult::Tie,

            (Choice::Rock, Choice::Scissors) => GameRoundResult::Winner(Player::Me),
            (Choice::Scissors, Choice::Paper) => GameRoundResult::Winner(Player::Me),
            (Choice::Paper, Choice::Rock) => GameRoundResult::Winner(Player::Me),

            (Choice::Scissors, Choice::Rock) => GameRoundResult::Winner(Player::Opponent),
            (Choice::Paper, Choice::Scissors) => GameRoundResult::Winner(Player::Opponent),
            (Choice::Rock, Choice::Paper) => GameRoundResult::Winner(Player::Opponent),
        }
    }
}

fn line_part_one(input: &str) -> IResult<&str, RoundPartOne> {
    map(
        separated_pair(one_of("ABC"), char(' '), one_of("XYZ")),
        |(opponent_char, my_char)| {
            let opponent_choice = match opponent_char {
                'A' => Choice::Rock,
                'B' => Choice::Paper,
                'C' => Choice::Scissors,
                other => unreachable!("unexpected opponent choice {other}"),
            };

            let my_choice = match my_char {
                'X' => Choice::Rock,
                'Y' => Choice::Paper,
                'Z' => Choice::Scissors,
                other => unreachable!("unexpected self choice {other}"),
            };

            RoundPartOne(GameRound {
                my_choice,
                opponent_choice,
            })
        },
    )(input)
}

fn game_part_one(input: &str) -> IResult<&str, Vec<RoundPartOne>> {
    terminated(separated_list1(newline, line_part_one), opt(newline))(input)
}
fn parse_game_part_one(input: &str) -> Result<Vec<RoundPartOne>> {
    let (leftover, result) = game_part_one(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

fn line_part_two(input: &str) -> IResult<&str, RoundPartTwo> {
    map(
        separated_pair(one_of("ABC"), char(' '), one_of("XYZ")),
        |(opponent_char, my_char)| {
            let opponent_choice = match opponent_char {
                'A' => Choice::Rock,
                'B' => Choice::Paper,
                'C' => Choice::Scissors,
                other => unreachable!("unexpected opponent choice {other}"),
            };

            let needed_result = match my_char {
                'X' => GameRoundResult::Winner(Player::Opponent),
                'Y' => GameRoundResult::Tie,
                'Z' => GameRoundResult::Winner(Player::Me),
                other => unreachable!("unexpected self choice {other}"),
            };

            RoundPartTwo {
                needed_result,
                opponent_choice,
            }
        },
    )(input)
}

fn game_part_two(input: &str) -> IResult<&str, Vec<RoundPartTwo>> {
    terminated(separated_list1(newline, line_part_two), opt(newline))(input)
}

fn parse_game_part_two(input: &str) -> Result<Vec<RoundPartTwo>> {
    let (leftover, result) = game_part_two(input).map_err(|err| err.map_input(str::to_string))?;

    if !leftover.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            leftover
        ));
    }

    Ok(result)
}

pub fn part_one(input: &str) -> Result<u32> {
    let game = parse_game_part_one(input)?;
    Ok(game
        .into_iter()
        .map(|round| round.0.calculate_score())
        .sum())
}

pub fn part_two(input: &str) -> Result<u32> {
    let game = parse_game_part_two(input)?;
    Ok(game
        .into_iter()
        .map(|round| {
            let my_choice = round.calculate_my_choice();
            let round = GameRound {
                my_choice,
                opponent_choice: round.opponent_choice,
            };
            round.calculate_score()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 11_150);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 8_295);
    }
}
