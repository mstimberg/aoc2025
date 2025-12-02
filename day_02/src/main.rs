use itertools::Itertools;
use std::fs;

fn invalid_sum_for_range(start: &str, end: &str) -> u64 {
    let start_value = start.parse::<u64>().unwrap();
    let end_value = end.parse::<u64>().unwrap();
    let mut sum = 0;
    // Maybe slightly more intelligent than going through the full range:
    // Create all possible invalid ids of the appropriate length and check whether they
    // are in the range
    let start_length_pair = ((start_value.ilog10() + 2) / 2)*2;  // round up to next pair
    let end_length_pair = ((end_value.ilog10() + 1) / 2) * 2;  // round down to next pair
    for sequence_length in start_length_pair/2..end_length_pair/2+1 {
        for sequence in 10_u64.pow(sequence_length-1)..10_u64.pow(sequence_length) {
            let invalid_value = sequence*10_u64.pow(sequence_length) + sequence;
            if invalid_value < start_value {
                continue;
            }
            if invalid_value > end_value {
                break;  // no need to create more values
            }
            sum += invalid_value;
        }
    }
    sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let ranges: Vec<_> = contents
        .split(',')
        .map(|range| range.split('-').next_tuple::<(&str, &str)>().unwrap())
        .collect();
    let sums: Vec<_> = ranges
        .into_iter()
        .map(|(start, end)| invalid_sum_for_range(start, end))
        .collect();
    let total: u64 = sums.iter().sum();
    println!("Sum of invalid ids: {}", total);
}
