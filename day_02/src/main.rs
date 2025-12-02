use itertools::Itertools;
use std::{collections::HashSet, fs};

fn invalid_sum_for_range(start: &str, end: &str) -> u64 {
    println!("{}-{}", start, end);
    let start_value = start.parse::<u64>().unwrap();
    let end_value = end.parse::<u64>().unwrap();
    let mut sum = 0;
    // Maybe slightly more intelligent than going through the full range and verifying all values:
    // Create all possible invalid ids of the appropriate length and check whether they
    // are in the range
    let start_length = start_value.ilog10() + 1;
    let end_length = end_value.ilog10() + 1;
    let mut counted = HashSet::new();
    for sequence_length in 1..end_length/2+1 {
        for sequence in 10_u64.pow(sequence_length-1)..10_u64.pow(sequence_length) {
            for repetitions in start_length / sequence_length..end_length/sequence_length+1 {
                
                if repetitions == 1 && sequence_length == 1 {
                    continue;  // Do not count single digits as repetitions
                }
                
                let mut invalid_value = 0;
                for rep in 0..repetitions {
                    invalid_value += sequence * 10_u64.pow(rep*sequence_length);
                }

                if invalid_value < start_value {
                    continue;
                }
                if invalid_value > end_value {
                    break;
                }
                if !counted.contains(&invalid_value) {
                    println!(" {} is invalid", invalid_value);
                    sum += invalid_value;
                    counted.insert(invalid_value);
                }
            }
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
