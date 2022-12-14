use crate::utils::file_to_vec;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Elf {
    id: u32,
    calories: u32,
}

// Implement "reverse" ordering - largest first
impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        other.calories.cmp(&self.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.calories.cmp(&self.calories))
    }
}

pub fn compute_calories(path: &str) -> Result<Vec<Elf>, Box<dyn std::error::Error>> {
    let mut computed_calories = vec![];

    let mut elf_id = 0;
    let mut elf_calories = 0;

    let lines = file_to_vec(path);

    for line in lines {
        if line.is_empty() {
            computed_calories.push(Elf {
                id: elf_id,
                calories: elf_calories,
            });
            elf_id += 1;
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<u32>()?;
        }
    }

    // It'd be nice to do this in one go - detect the last line as part of the loop
    computed_calories.push(Elf {
        id: elf_id,
        calories: elf_calories,
    });

    Ok(computed_calories)
}

pub fn find_elf_with_largest_calories(computed_calories: &[Elf]) -> Option<&Elf> {
    computed_calories
        .iter()
        .reduce(|a, b| if a.calories > b.calories { a } else { b })
}

pub fn compute_total_calories_of_slice(elves: &[Elf], slice_size: usize, order: bool) -> Option<u32> {
    let mut local_vec = elves.to_vec();
    if order {
        local_vec.sort();
    }

    local_vec.as_slice()[0..slice_size]
        .iter()
        .map(|e| e.calories)
        .reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "src/test_inputs/day1_example.txt";

    #[test]
    fn test_example_input() {
        let mut expected = vec![];
        expected.push(Elf { id: 0, calories: 6000 });
        expected.push(Elf { id: 1, calories: 4000 });
        expected.push(Elf { id: 2, calories: 11000 });
        expected.push(Elf { id: 3, calories: 24000 });
        expected.push(Elf { id: 4, calories: 10000 });

        assert_eq!(expected, compute_calories(INPUT).unwrap());
    }

    #[test]
    fn test_find_largest_elf() {
        let expected = Elf { id: 3, calories: 24000 };
        let computed = compute_calories(INPUT).unwrap();
        let result = find_elf_with_largest_calories(&computed).unwrap();

        assert_eq!(&expected, result);
    }

    #[test]
    fn test_compute_total_calories() {
        let expected = 45000;
        let mut computed = compute_calories(INPUT).unwrap();

        let result = compute_total_calories_of_slice(&mut computed, 3, true).unwrap();

        assert_eq!(expected, result);
    }
}
