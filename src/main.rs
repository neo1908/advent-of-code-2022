mod day1;

fn main() {
    let computed_calories =
        day1::compute_calories("inputs/day1.txt").expect("Error when computing calories");

    let largest_elf = day1::find_elf_with_largest_calories(&computed_calories)
        .expect("Error when finding the big elf");
    println!("Largest Elf is {:?}", largest_elf);

    let top_3_calorie_total = day1::compute_total_calories_of_slice(&computed_calories, 3, true)
        .expect("Error when computing slice total");
    println!("Top 3 total {}", top_3_calorie_total);
}
