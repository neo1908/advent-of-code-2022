mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    println!("----- day1");

    let computed_calories = day1::compute_calories("inputs/day1.txt").expect("Error when computing calories");

    let largest_elf = day1::find_elf_with_largest_calories(&computed_calories).expect("Error when finding the big elf");
    println!("Largest Elf is {:?}", largest_elf);

    let top_3_calorie_total =
        day1::compute_total_calories_of_slice(&computed_calories, 3, true).expect("Error when computing slice total");
    println!("Top 3 total {}", top_3_calorie_total);

    println!("----- day2");

    let computed_score = day2::part1_calculate_expected_score("inputs/day2.txt");
    println!("Part 1 total score {}", computed_score);

    let part2_computed_score = day2::part2_calculate_correct_strategy_score("inputs/day2.txt");
    println!("Part 2 total score {}", part2_computed_score);

    println!("----- day3");

    let priority_total = day3::find_sum_of_priorities("inputs/day3.txt");
    println!("Priority total {}", priority_total);

    let badge_priority_total = day3::find_priority_sum_of_badges("inputs/day3.txt");
    println!("Badge priority total {}", badge_priority_total);
}
