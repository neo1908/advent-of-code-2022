use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Elf
{
    id: u32,
    calories: u32,
}

// Implement "reverse" ordering - largest first
impl Ord for Elf
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        other.calories.cmp(&self.calories)
    }
}

impl PartialOrd for Elf
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(other.calories.cmp(&self.calories))
    }
}

pub fn compute_calories(path: &str) -> Result<Vec<Elf>, Box<dyn std::error::Error>>
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut computed_calories = vec![];

    let mut elf_id = 0;
    let mut elf_calories = 0;

    for line in reader.lines()
    {
        let parsed_line = line?;

        if parsed_line.is_empty()
        {
            computed_calories.push(Elf{id: elf_id, calories: elf_calories});
            elf_id += 1;
            elf_calories = 0;
        }
        else
        {
            elf_calories += parsed_line.parse::<u32>()?;
        }
    }

    // It'd be nice to do this in one go - detect the last line as part of the loop
    computed_calories.push(Elf{id: elf_id, calories: elf_calories });

    return Ok(computed_calories);
}

pub fn find_elf_with_largest_calories(computed_calories: &Vec<Elf>) -> Option<&Elf>
{
    computed_calories.into_iter().reduce(|a,b| if a.calories > b.calories {a} else { b })
}

pub fn compute_total_calories_of_slice(elves: &Vec<Elf>, slice_size: usize, order: bool) -> Option<u32>
{
    let mut local_vec = elves.to_vec();
    if order
    {
        local_vec.sort();
    }

    local_vec.as_slice()[0..slice_size].into_iter().map(|e| e.calories).reduce(|a,b| a + b )
}

#[cfg(test)]
mod tests
{
    use super::*;

    const INPUT: &str = "/data/code/rust/advent-of-code-2022/src/test_inputs";

    #[test]
    fn test_example_input()
    {
        let mut expected = vec![];
        expected.push(Elf{id: 0, calories: 6000});
        expected.push(Elf{id: 1, calories: 4000});
        expected.push(Elf{id: 2, calories: 11000});
        expected.push(Elf{id: 3, calories: 24000});
        expected.push(Elf{id: 4, calories: 10000});

        assert_eq!(expected, compute_calories( INPUT ).unwrap() );
    }

    #[test]
    fn test_find_largest_elf()
    {
        let expected = Elf{id: 3, calories: 24000};
        let computed = compute_calories(INPUT ).unwrap();
        let result = find_elf_with_largest_calories(&computed ).unwrap();

        assert_eq!(&expected, result);
    }

    #[test]
    fn test_compute_total_calories()
    {
        let expected = 45000;
        let mut computed = compute_calories(INPUT ).unwrap();

        let result = compute_total_calories_of_slice(&mut computed, 3, true).unwrap();

        assert_eq!(expected, result);
    }
}