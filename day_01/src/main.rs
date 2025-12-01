use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let numbers: Vec<_> = contents
        .lines()
        .map(|line| {
            let (direction, number) = line.split_at(1);
            match direction {
                "L" => -number.parse::<i32>().unwrap(),
                "R" => number.parse::<i32>().unwrap(),
                _ => panic!("Unexpected value"),
            }
        })
        .collect();
    let mut current = 50;
    let mut zero_crossings = 0;
    for number in numbers {
        zero_crossings += (number / 100).abs();  // multiple rotations
        let remainder = number % 100;

        if current!= 0 && (current + remainder < 0 || current + remainder > 100) {  // we crossed 0
            zero_crossings += 1;
        }
        current = (current + remainder + 100) % 100;
        if current == 0 {  // we ended at 0
            zero_crossings += 1;
        }
    }
    println!("{} zero crossings", zero_crossings)
}
