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
    let mut zeros = 0;
    for number in numbers {
        // Very straightforward solution without thinking about it too hard
        current = (current + number) % 100;
        if current == 0 {
            zeros +=1 ;
        }
    }
    println!("{} zero crossings", zeros)
}
