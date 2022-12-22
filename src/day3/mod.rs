use crate::utils::file_to_vec;

pub fn find_sum_of_priorities(input: &str) -> u32 {
    let lines = file_to_vec(input);

    let mut priority_total: u32 = 0;
    for line in lines {
        let midway_point = line.len() / 2;
        let compartment_1 = &line[0..midway_point];
        let compartment_2 = &line[midway_point..line.len()];

        let mut common_items: Vec<_> = compartment_1.chars().filter(|x| compartment_2.contains(*x)).collect();
        common_items.dedup();

        // Need to add 58 when uppercase -> 32 to adjust to ascii dec then 26 to put in 2nd half of alphabet
        let priority: u8 = common_items.iter().map(get_priority).sum();

        priority_total += priority as u32;
    }

    priority_total
}

fn get_priority(c: &char) -> u8 {
    let ascii = if c.is_uppercase() { *c as u8 + 58 } else { *c as u8 };

    ascii - 96
}

pub fn find_priority_sum_of_badges(input: &str) -> u32 {
    let lines = file_to_vec(input);

    let mut priority_total = 0;
    let mut window_start = 0;

    loop {
        if lines.len() < window_start + 3 {
            break;
        }
        let group = &lines[window_start..window_start + 3];

        if group.len() != 3 {
            break;
        }

        let mut common_elements: Vec<_> = group[0]
            .chars()
            .filter(|x| group[1].contains(*x) && group[2].contains(*x))
            .collect();
        common_elements.dedup();

        let priority: u8 = common_elements.iter().map(get_priority).sum();
        priority_total += priority as u32;
        window_start += 3;
    }

    priority_total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "src/test_inputs/day3_example.txt";

    #[test]
    fn test_find_sum_of_priorities() {
        let expected = 157;
        let result = find_sum_of_priorities(INPUT);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_generate_badges() {
        let expected = 70;
        let result = find_priority_sum_of_badges(INPUT);

        assert_eq!(expected, result);
    }
}
