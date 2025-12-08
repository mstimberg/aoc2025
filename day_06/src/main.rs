use std::fs;

fn turned(text: String) -> Vec<Vec<char>> {
    let mut first_line = true;
    let mut turned_around: Vec<Vec<_>> = vec![];
    for line in text.lines() {
        if first_line {
            for c in line.chars() {
                turned_around.push(vec![c]);
            }
            first_line = false;
        } else {
            for (index, c) in line.chars().enumerate() {
                turned_around[index].push(c);
            }
        }
    }
    // We add an empty vector in the end, so that every block ends with one
    turned_around.push(vec![' ']);
    turned_around
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut values_per_column: Vec<Vec<u64>> = vec![];
    let mut first_line = true;
    let mut results: Vec<_> = vec![];

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        // First line (we don't know the number of columns yet)
        if first_line {
            for val in line.split_whitespace() {
                let values = vec![val.parse::<u64>().unwrap()];
                values_per_column.push(values);
            }
            first_line = false;
        } else {  // Last line with operators
            if line[0..1] == *"+" || line[0..1] == *"*"  { // last line
                for (index, val) in line.split_whitespace().enumerate() {
                    let values = &values_per_column[index];
                    if val == "+" {
                        results.push(values.iter().sum::<u64>());
                    } else {
                        results.push(values.iter().fold(1u64, |acc, el| acc * *el));
                    }
                }    
            break;
            }
            // Normal line with values
            for (index, val) in line.split_whitespace().enumerate() {
                values_per_column[index].push(val.parse::<u64>().unwrap());
            }
        }
    }
    let total = results.iter().sum::<u64>();
    println!("Grand total: {}", total);

    // Part 2
    let contents2 = turned(contents.clone());
    let mut results2 = vec![];
    let mut new_calculation = true;
    let mut current_operation = '+';
    let mut values = vec![];
    for column in contents2 {
        if new_calculation {
            current_operation = column[column.len()-1];
            values.clear();
            new_calculation = false;
        }
        let combined: String = column[0..column.len()-1].iter().collect::<String>().trim().to_string();
        if combined.len() > 0 {
            values.push(combined.parse::<u64>().unwrap());
        } else {
            // empty column: do the calculation and start again
            if current_operation == '*' {
                results2.push(values.iter().fold(1u64, |acc, el| acc * *el));
            } else {
                results2.push(values.iter().sum::<u64>());
            }
            new_calculation = true;
        }
    }
    let total2 = results2.iter().sum::<u64>();
    println!("Grand total 2: {}", total2);
}
