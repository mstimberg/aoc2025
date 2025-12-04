use ndarray::prelude::*;
use std::{fs, iter::zip};

fn print_map(map: &Array2<bool>) {
    for row in map.outer_iter() {
        for tile in row {
            print!("{}", if *tile { '@' } else { '.' });
        }
        println!()
    }
}

fn remove_rolls(map: &Array2<bool>) -> (Array2<bool>, u32){
    let x_neighbours = [-1, -1, -1, 0, 0, 1, 1, 1];
    let y_neighbours = [-1, 0, 1, -1, 1, -1, 0, 1];
    let map_shape = map.shape();
    let mut count = 0;
    let mut new_map = map.clone();
    for index in 0..map_shape[0]*map_shape[1] {
        let row = index / map_shape[1];
        let col = index % map_shape[1];
        if !map[[row, col]] {
            continue;
        }
        let mut neighbours = 0;
        for (dx, dy) in zip(x_neighbours, y_neighbours) {
            let x: i32 = row as i32 + dx;
            let y: i32 = col as i32 + dy;
            if x >= 0 && y >= 0 && x < map_shape[0] as i32 && y < map_shape[1] as i32 && map[[x as usize, y as usize]] {
                neighbours += 1;
            }
        }
        if neighbours < 4 {
            count += 1;
            new_map[[row, col]] = false;
        }
    }
    (new_map, count)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut rows = 0;
    let mut flat_map = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        flat_map.extend(line.chars());
        rows += 1;
    }
    let cols = flat_map.len() / rows;
    assert!(cols * rows == flat_map.len());
    let map = Array2::from_shape_vec(
        (rows, cols),
        flat_map.iter().map(|c| *c == '@').collect::<Vec<_>>(),
    )
    .unwrap();
    print_map(&map);
    let mut total = 0;
    let mut new_map = map;
    let mut removed = 0;
    loop {
        (new_map, removed) = remove_rolls(&new_map);
        println!("After removing {}", removed);
        print_map(&new_map);
        total += removed;
        if removed == 0 {
            println!("Nothing to remove anymore");
            break;
        }
    }
    println!("Removed {} rolls in total", total);
}
