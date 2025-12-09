use std::{collections::HashSet, collections::HashMap, fs};
use itertools::sorted;
use ndarray::prelude::*;

#[derive(Debug)]
pub struct Vertex {
    source: usize,
    target: usize,
    distance: i64
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut coords = vec![];
    for line in contents.lines() {
        for number in line.split(",") {
            coords.push(number.parse::<i64>().unwrap());
        }
    }
    let n_boxes = coords.len()/3;
    let matrix = Array2::from_shape_vec((n_boxes, 3), coords.clone()).unwrap();
    // Square of distance, but we don't care
    let distances = (matrix.clone().insert_axis(Axis(2)) - matrix.t()).mapv(|x| x.pow(2)).sum_axis(Axis(1));    
    let mut vertices = vec![];
    for ((r, c), v) in distances.indexed_iter() {
        if r < c {
            vertices.push(Vertex{source: r,
            target: c,
            distance: *v})
        }
    }
    vertices.sort_by(|a, b| a.distance.cmp(&b.distance));    
    let mut n_connections = 0;
    let mut connections = vertices.iter();
    let mut groups = HashMap::<usize, usize>::new();
    let mut all_groups = Vec::<HashSet<usize>>::new();
    const CONNECTIONS: usize = 1000;
    while n_connections < vertices.len() {
        n_connections += 1;
         if n_connections == CONNECTIONS {
            let group_sizes: Vec<_> = sorted(all_groups.iter().map(|s| s.len())).rev().collect();
            println!("Multiplied three biggest groups after {} connections: {}", n_connections, group_sizes[0] * group_sizes[1] * group_sizes[2]);
        }
        // println!("After {} connections", n_connections);
        // println!("{:?}", all_groups);
        let vertex = connections.next().unwrap();
        let source = vertex.source;
        let target = vertex.target;
        // println!("Connecting {} to {}", source, target);
        let source_group = groups.get(&source);
        let target_group = groups.get(&target);
        if source_group.is_none() && target_group.is_none() {
            let mut new_group = HashSet::<usize>::new();
            new_group.insert(source);
            new_group.insert(target);
            let group_idx = all_groups.len();
            all_groups.push(new_group);
            groups.insert(source, group_idx);
            groups.insert(target, group_idx);            
        } else if source_group.is_some() && target_group.is_some() && source_group.unwrap() == target_group.unwrap() {
            continue;  // already connected
        } else if source_group.is_some() && target_group.is_none() {
            let src_idx = *source_group.unwrap();
            all_groups[src_idx].insert(target);
            groups.insert(target, src_idx);
        } else if source_group.is_none() && target_group.is_some() {
            let tgt_idx = *target_group.unwrap();
            all_groups[tgt_idx].insert(source);
            groups.insert(source, tgt_idx);
        } else {  // need to merge two groups
            let src_idx = *source_group.unwrap();
            let tgt_idx = *target_group.unwrap();
            let mut to_insert = HashSet::<usize>::new();
            for el in all_groups[tgt_idx].iter().cloned() {
                to_insert.insert(el);
                groups.insert(el, src_idx);
            }
            all_groups[src_idx].extend(to_insert);
            all_groups[tgt_idx].clear();
        }
        let source_group = groups.get(&source).unwrap();
        if all_groups[*source_group].len() == n_boxes { // all elements are in the group
            println!("After {} connections, all are in one group", n_connections);
            println!("Last vertices that were connected: {} and {}", vertex.source, vertex.target);
            let x1 = matrix[[vertex.source, 0]];
            let x2 = matrix[[vertex.target, 0]];
            println!("x coordinates: {} x {} = {}", x1, x2, x1*x2)
        }
    }
    // println!("{:?}", all_groups);
    
    // println!("{:?}", group_sizes);
    
}
