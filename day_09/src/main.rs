use itertools::{Itertools, iproduct};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let red_tiles: Vec<_> = contents
        .lines()
        .map(|l| l.split(",").next_tuple::<(&str, &str)>().unwrap())
        .map(|(x_str, y_str)| (x_str.parse::<i64>().unwrap(), y_str.parse::<i64>().unwrap()))
        .collect();
    let mut top_right_corners = vec![];
    let mut lower_left_corners = vec![];
    let mut top_left_corners = vec![];
    let mut lower_right_corners = vec![];
    for tile in &red_tiles {
        if !(red_tiles.iter().any(|t| t.0 == tile.0 && t.1 < tile.1)
            || red_tiles.iter().any(|t| t.1 == tile.1 && t.0 > tile.0))
        {
            top_right_corners.push(*tile);
        }
        if !(red_tiles.iter().any(|t| t.0 == tile.0 && t.1 > tile.1)
            || red_tiles.iter().any(|t| t.1 == tile.1 && t.0 < tile.0))
        {
            lower_left_corners.push(*tile);
        }
        if !(red_tiles.iter().any(|t| t.0 == tile.0 && t.1 < tile.1)
            || red_tiles.iter().any(|t| t.1 == tile.1 && t.0 < tile.0))
        {
            top_left_corners.push(*tile);
        }
        if !(red_tiles.iter().any(|t| t.0 == tile.0 && t.1 > tile.1)
            || red_tiles.iter().any(|t| t.1 == tile.1 && t.0 > tile.0))
        {
            lower_right_corners.push(*tile);
        }
    }
    // println!("Potential top right corners: {:?}", top_right_corners);
    // println!("Potential lower left corners: {:?}", lower_left_corners);
    // println!("Potential top left corners: {:?}", top_left_corners);
    // println!("Potential lower right corners: {:?}", lower_right_corners);
    let mut max_area = 0;
    // Try all combinations
    for (ll, tr) in iproduct!(lower_left_corners, top_right_corners) {
        let area = (tr.0 - ll.0 + 1) * (ll.1 - tr.1 + 1);

        if area > max_area {
            max_area = area;
        }
    }
    for (lr, tl) in iproduct!(lower_right_corners, top_left_corners) {
        let area = (lr.0 - tl.0 + 1) * (lr.1 - tl.1 + 1);
        if area > max_area {
            max_area = area;
        }
    }
    println!("Maximum area: {}", max_area);
}
