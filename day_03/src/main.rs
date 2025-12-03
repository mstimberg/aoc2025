use std::fs;

fn joltage(bank: &str) -> u64 {
    let mut indices: Vec<usize> = vec![];
    // highest number (ignoring the eleven last ones) -- for several numbers take the first one (inverse of default choice, therefore we revert)
    let first_idx = bank.len()
        - 12
        - bank[..bank.len() - 11]
            .chars()
            .rev()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .map(|(idx, _)| idx)
            .unwrap();
    indices.push(first_idx);

    for digit in 1..12 {
        let start = indices[digit - 1] + 1;
        let stop = bank.len() - 12 + digit;
        // highest number following the previous number
        let local_idx = bank[start..stop + 1]
            .chars()
            .rev()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .map(|(idx, _)| idx)
            .unwrap();
        let next_idx = stop - local_idx;
        indices.push(next_idx);
    }

    let mut value: u64 = 0;
    for (digit, index) in indices.into_iter().enumerate() {
        value += 10_u64.pow(11 - TryInto::<u32>::try_into(digit).unwrap())
            * bank[index..index + 1].parse::<u64>().unwrap();
    }
    println!("{}: {}", bank, value);
    value
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let banks = contents.lines();
    let joltages: Vec<_> = banks.map(|b| joltage(b)).collect();
    let sum = joltages.into_iter().sum::<u64>();
    println!("Total sum: {}", sum);
}
