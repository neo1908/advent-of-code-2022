use crate::day2::RoundResult::{Draw, Loss, Win};
use crate::utils::file_to_vec;

enum RoundResult {
    Win,
    Draw,
    Loss
}

impl RoundResult {
    pub fn of(needed: &str) -> RoundResult
    {
        match needed {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!()
        }
    }

    pub fn needed<'a>(&'a self, play: &'a str) -> &str
    {
        match (self, play)
        {
            (Win, "A") => "B",
            (Win, "B") => "C",
            (Win, "C") => "A",
            (Loss, "A") => "C",
            (Loss, "B") => "A",
            (Loss, "C") => "B",
            (Draw, _) => play,
            _ => unreachable!()
        }
    }
}

pub fn part1_calculate_expected_score(input: &str) -> i32
{
    let lines = file_to_vec(input);

    let mut score: i32 = 0;

    for line in lines {
        match line {
            Ok(x) => {
                score += match x.as_ref() {
                    "A X" => 4,
                    "A Y" => 8,
                    "A Z" => 3,
                    "B X" => 1,
                    "B Y" => 5,
                    "B Z" => 9,
                    "C X" => 7,
                    "C Y" => 2,
                    "C Z" => 6,
                    _ => unreachable!()
                };
            },
            Err(e) => println!("Error reading file {:?}", e)
        }
    }

    score
}

pub fn part2_calculate_correct_strategy_score(input: &str) -> i32
{
    let lines = file_to_vec(input);

    let mut score = 0;

    for line in lines {
        match line {
            Ok(x) => {
                let round_parts = x.split_whitespace().collect::<Vec<&str>>();
                let round_result = RoundResult::of(round_parts[1]);
                let needed_play = round_result.needed(round_parts[0]);

                let play = round_parts[0].to_owned() + " " + needed_play;

                score += match play.as_ref() {
                    "A A" => 4,
                    "A B" => 8,
                    "A C" => 3,
                    "B A" => 1,
                    "B B" => 5,
                    "B C" => 9,
                    "C A" => 7,
                    "C B" => 2,
                    "C C" => 6,
                    _ => unreachable!()
                };
            }
            Err(e) => println!("Error reading file {:?}", e)
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "src/test_inputs/day2_example.txt";

    #[test]
    fn test_example_input()
    {
        let expected = 15;
        let result = part1_calculate_expected_score(INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_needed_draw()
    {
        let expected = "A";
        let result = Draw.needed("A");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_needed_win()
    {
        let expected = "A";
        let result = Win.needed("C");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_needed_loss()
    {
        let expected = "A";
        let result = Loss.needed("B");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_part2()
    {
        let expected = 12;
        let result = part2_calculate_correct_strategy_score(INPUT );

        assert_eq!(expected, result);
    }
}