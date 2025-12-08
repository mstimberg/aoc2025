use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut beams = vec![];
    let mut splits = 0;
    for line in contents.lines() {
        if beams.len() == 0 { // first line
            beams.resize(line.len(), false);
            beams[line.find("S").unwrap()] = true;
        } else {
            let mut new_beams = vec![false; beams.len()];
            for index in 0..beams.len() {
                if beams[index] && line[index..index+1] == *"." {  // continues
                    new_beams[index] = true;
                } else if beams[index] && line[index..index+1] == *"^" { // splits
                    splits += 1;
                    if index > 0 {
                        new_beams[index - 1] = true;
                    }
                    if index < beams.len()-1 {
                        new_beams[index + 1] = true;
                    }
                }
            }
            beams = new_beams;
        }
    }
    println!("Splits: {}", splits);
}
