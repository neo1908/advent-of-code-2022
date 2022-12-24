use crate::utils::file_to_vec;

pub fn find_overlapping_pairs_count(input: &str) -> u32 {
    let lines = file_to_vec(input);

    let mut total_overlapping_pairs = 0;

    for line in lines {
        let pairings: Vec<_> = line.split(',').into_iter().collect();

        let pairing1_ranges: Vec<_> = create_range_vec(pairings[0]);
        let pairing2_ranges: Vec<_> = create_range_vec(pairings[1]);

        let pairing_1_larger = pairing1_ranges[0] <= pairing2_ranges[0] && pairing1_ranges[1] >= pairing2_ranges[1];
        let pairing_2_larger = pairing2_ranges[0] <= pairing1_ranges[0] && pairing2_ranges[1] >= pairing1_ranges[1];

        if pairing_1_larger || pairing_2_larger {
            total_overlapping_pairs += 1;
        }
    }

    total_overlapping_pairs
}

pub fn find_pairs_overlapping_anywhere_count(input: &str) -> u32 {
    let lines = file_to_vec(input);

    let mut total_overlapping_pairs = 0;

    for line in lines {
        let pairings: Vec<_> = line.split(',').into_iter().collect();

        let mut all_pairing_values: Vec<u32> = vec![];

        let pairing1_ranges: Vec<_> = create_range_vec(pairings[0]);
        all_pairing_values.extend(pairing1_ranges[0]..pairing1_ranges[1] + 1);

        let pairing2_ranges: Vec<_> = create_range_vec(pairings[1]);
        all_pairing_values.extend(pairing2_ranges[0]..pairing2_ranges[1] + 1);

        let with_duplicates_size = all_pairing_values.len();
        all_pairing_values.sort();
        all_pairing_values.dedup();

        if all_pairing_values.len() < with_duplicates_size {
            total_overlapping_pairs += 1;
        }
    }

    total_overlapping_pairs
}

fn create_range_vec(x: &str) -> Vec<u32> {
    x.split('-').into_iter().map(|y| y.parse::<u32>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "src/test_inputs/day4_example.txt";

    #[test]
    pub fn test_find_overlapping_pairs_count() {
        let expected = 2;
        let result = find_overlapping_pairs_count(INPUT);

        assert_eq!(expected, result)
    }

    #[test]
    pub fn test_pairs_overlap_anywhere_count() {
        let expected = 4;
        let result = find_pairs_overlapping_anywhere_count(INPUT);

        assert_eq!(expected, result);
    }
}
