use std::fs;

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
            println!("Length of line: {}", line.len());
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
}
