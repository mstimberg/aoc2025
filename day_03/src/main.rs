use std::fs;

fn joltage(bank: &str) -> i32 {
    // highest number (ignoring the last one) -- for several numbers take the first one (inverse of default choice, therefore we revert)
    let first_idx = bank.len()
        - 2
        - bank[..bank.len() - 1]
            .chars()
            .rev()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .map(|(idx, _)| idx)
            .unwrap();
    // highest number following the first number
    let second_idx = bank.len()
        - (1 + bank[first_idx + 1..]
            .chars()
            .rev()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .map(|(idx, _)| idx)
            .unwrap());
    let first_value = bank[first_idx..first_idx + 1].parse::<i32>().unwrap();
    let second_value = bank[second_idx..second_idx + 1].parse::<i32>().unwrap();

    first_value * 10 + second_value
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let banks = contents.lines();
    let joltages: Vec<_> = banks.map(|b| joltage(b)).collect();
    let sum = joltages.into_iter().sum::<i32>();
    println!("Total sum: {}", sum);
}
