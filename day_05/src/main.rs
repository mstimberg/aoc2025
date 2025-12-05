use std::fs;
use itertools::Itertools;
// Cheating a bit…
use interval::{interval_set::*, ops::Range, prelude::Empty, prelude::Contains, prelude::Cardinality};


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut first_section = true;
    let mut fresh_and_available = 0;
    let mut fresh = IntervalSet::empty();
    for line in contents.lines() {
        if first_section {
            if line.is_empty() {
                first_section = false;
                continue;
            }
            let (start, end) = line.split("-").map(|s| s.parse::<u64>().unwrap()).next_tuple::<(u64, u64)>().unwrap();
            fresh.extend(IntervalSet::new(start, end));
        } else {
            let value = line.parse::<u64>().unwrap();
            if fresh.contains(&value) {
                fresh_and_available += 1;
            }
        }
    }
    println!("{} available ingredients are fresh", fresh_and_available);
    let fresh_total = fresh.iter().map(|interval| interval.size()).sum::<u64>();
    println!("Total ingredients are fresh: {}", fresh_total);
}
