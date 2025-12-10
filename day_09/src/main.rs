use itertools::{Itertools, iproduct};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn is_inside(
    tile: (i64, i64),
    green_tiles: &HashSet<(i64, i64)>,
    red_tiles: &[(i64, i64)],
    cache: &mut HashMap<(i64, i64), bool>,
) -> bool {
    if red_tiles.contains(&tile) || green_tiles.contains(&tile) {
        return true;
    }
    let cache_entry = cache.get(&tile);
    if cache_entry.is_some() {
        return *cache_entry.unwrap();
    }
    let in_same_row: Vec<_> = green_tiles
        .iter()
        .filter(|t| t.1 == tile.1)
        .map(|t| t.0)
        .collect();
    let in_same_col: Vec<_> = green_tiles
        .iter()
        .filter(|t| t.0 == tile.0)
        .map(|t| t.1)
        .collect();
    let result;
    if in_same_row.is_empty() || in_same_col.is_empty() {
        result = false; // this shouldn't be possible
    } else {
        let (top, bottom) = (
            in_same_col.iter().min().unwrap(),
            in_same_col.iter().max().unwrap(),
        );
        let (left, right) = (
            in_same_row.iter().min().unwrap(),
            in_same_row.iter().max().unwrap(),
        );
        result = tile.0 >= *left && tile.0 <= *right && tile.1 >= *top && tile.1 <= *bottom;
    }
    // println!("Check for {:?}: {}", tile, result);
    cache.insert(tile, result);
    result
}
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

    // Part 2
    let mut green_tiles: HashSet<(i64, i64)> = HashSet::new();
    let first_red_tile = &red_tiles[0];
    let mut next_tile = red_tiles
        .iter()
        .filter(|t| t.0 == first_red_tile.0 && t.1 != first_red_tile.1)
        .next()
        .unwrap(); // there should be only one
    for y in std::cmp::min(next_tile.1, first_red_tile.1)
        ..std::cmp::max(next_tile.1, first_red_tile.1) + 1
    {
        green_tiles.insert((first_red_tile.0, y));
    }

    let mut look_in_row = true;
    while next_tile != first_red_tile {
        let current_tile = next_tile;
        if look_in_row {
            next_tile = red_tiles
                .iter()
                .filter(|t| t.1 == next_tile.1 && t.0 != next_tile.0)
                .next()
                .unwrap();
            for x in std::cmp::min(next_tile.0, current_tile.0)
                ..std::cmp::max(next_tile.0, current_tile.0) + 1
            {
                green_tiles.insert((x, current_tile.1));
            }
        } else {
            next_tile = red_tiles
                .iter()
                .filter(|t| t.0 == next_tile.0 && t.1 != next_tile.1)
                .next()
                .unwrap();
            for y in std::cmp::min(next_tile.1, current_tile.1)
                ..std::cmp::max(next_tile.1, current_tile.1) + 1
            {
                green_tiles.insert((current_tile.0, y));
            }
        }
        look_in_row = !look_in_row;
    }

    for y in 0..12 {
        for x in 0..12 {
            if red_tiles.contains(&(x, y)) {
                print!("R");
            } else if green_tiles.contains(&(x, y)) {
                print!("G");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    let mut max_area2 = 0;
    let mut cache = HashMap::<(i64, i64), bool>::new();
    let mut counter = 0;
    let mut candidates = vec![];
    for (ll, tr) in iproduct!(&red_tiles, &red_tiles) {
        counter += 1;
        println!("{}/{}", counter, red_tiles.len()*red_tiles.len());
        if ll == tr {
            continue;
        }
        let area = (tr.0 - ll.0 + 1) * (ll.1 - tr.1 + 1);
        // Other corners have to be green (includes red tiles in our definition)
        let top_left = (ll.0, tr.1);
        let lower_right = (tr.0, ll.1);
        if is_inside((ll.0, tr.1), &green_tiles, &red_tiles, &mut cache)
                && !is_inside((tr.0, ll.1), &green_tiles, &red_tiles, &mut cache)
        {
            candidates.push((ll, tr, area));
        }
        // if area > max_area2 {
        //     println!("Checking area with size {} (best so far: {})", area, max_area2);
        //     if !is_inside((ll.0, tr.1), &green_tiles, &red_tiles, &mut cache)
        //         || !is_inside((tr.0, ll.1), &green_tiles, &red_tiles, &mut cache)
        //     {
        //         continue;
        //     }
        //     let mut all_inside = true;
        //     println!("  Need to check full circumference of rectangle");
        //     for x in top_left.0..lower_right.0 + 1 {
        //         if !is_inside((x, top_left.1), &green_tiles, &red_tiles, &mut cache)
        //             || !is_inside((x, lower_right.1), &green_tiles, &red_tiles, &mut cache)
        //         {
        //             all_inside = false;
        //             break;
        //         }
        //     }

        //     if all_inside {
        //         for y in top_left.1..lower_right.1 + 1 {
        //             if !is_inside((top_left.0, y), &green_tiles, &red_tiles, &mut cache)
        //                 || !is_inside((lower_right.0, y), &green_tiles, &red_tiles, &mut cache)
        //             {
        //                 all_inside = false;
        //                 break;
        //             }
        //         }
        //     }
        //     if all_inside {
        //         max_area2 = area;
        //     }
        // }
    }
    println!("Candidates: {}", candidates.len());
    for (lr, tl) in iproduct!(&red_tiles, &red_tiles) {
        if lr == tl {
            continue;
        }
        let area = (lr.0 - tl.0 + 1) * (lr.1 - tl.1 + 1);
        if area < 0 {
            continue;
        }
        if area > max_area2 {
            println!("Checking area with size {} (best so far: {})", area, max_area2);
            if !is_inside((lr.0, tl.1), &green_tiles, &red_tiles, &mut cache)
                || !is_inside((tl.0, lr.1), &green_tiles, &red_tiles, &mut cache)
            {
                continue;
            }
            let mut all_inside = true;
            println!("  Need to check full circumference of rectangle");
            for x in tl.0..lr.0 + 1 {
                if !is_inside((x, tl.1), &green_tiles, &red_tiles, &mut cache)
                    || !is_inside((x, lr.1), &green_tiles, &red_tiles, &mut cache)
                {
                    all_inside = false;
                    break;
                }
            }
            if all_inside {
                for y in tl.1..lr.1 + 1 {
                    if !is_inside((tl.0, y), &green_tiles, &red_tiles, &mut cache)
                        || !is_inside((lr.0, y), &green_tiles, &red_tiles, &mut cache)
                    {
                        all_inside = false;
                        break;
                    }
                }
            }
            if all_inside {
                max_area2 = area;
            }
        }
    }
    println!("Maximum area with additional restrictions: {}", max_area2);
}
