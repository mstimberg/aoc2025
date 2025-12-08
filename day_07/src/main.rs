use std::{collections::HashMap, fs};

fn timelines(lines: Vec<&str>, beam_pos: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
    if lines.is_empty() {
        return 1;
    }
    let key = (lines.len(), beam_pos);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let paths;
    let line = lines[0];
    if line[beam_pos..beam_pos+1] == *"." {
        paths = timelines(lines[1..].to_vec(), beam_pos, cache);
    } else { // split
        let left_timelines = timelines(lines[1..].to_vec(), beam_pos - 1, cache);
        let right_timelines = timelines(lines[1..].to_vec(), beam_pos + 1, cache);
        paths = left_timelines + right_timelines;
    }
    cache.insert(key, paths);
    return paths;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = contents.lines().collect();
    let start = lines[0].find('S').unwrap();
    let mut cache = HashMap::<(usize, usize), u64>::new();
    let all_timelines = timelines(lines[1..].to_vec(), start, &mut cache);
    println!("Timelines: {}", all_timelines);
    // println!("Cache: {:?}", cache);
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
