fn main() {
    let input = include_str!("../../inputs/d01.txt");

    // Return calories carried by each elf
    let elf_calories = input
        .split("\n\n")
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // Sum of the calories carried by each elf
    let elf_sum_calories = elf_calories
        .iter()
        .map(|cs| cs.iter().sum::<usize>())
        .collect::<Vec<usize>>();

    // Find the max calories
    match elf_sum_calories.iter().max() {
        Some(max) => println!("Max calories: {}", max),
        None => println!("No max found"),
    }

    // Find TOP 3 carried calories by sorting and reversing the sum calories vector
    let mut elf_sorted_sum_calories = elf_sum_calories;
    elf_sorted_sum_calories.sort();
    elf_sorted_sum_calories.reverse();

    // Print the TOP 3 and sum them
    println!(
        "Sum of TOP 3 calories: {}",
        elf_sorted_sum_calories.iter().take(3).sum::<usize>()
    );
}
