use std::collections::HashMap;
use std::fs;

fn paths_to_out<'a>(
    server: &'a str,
    rack: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(n_paths) = cache.get(server) {
        return *n_paths;
    }
    let mut n_paths = 0;
    for output in rack.get(server).unwrap() {
        if *output == "out" {
            n_paths += 1;
        } else {
            n_paths += paths_to_out(output, rack, cache);
        }
    }
    cache.insert(server, n_paths);
    n_paths
}

fn paths_to_out_with_visits<'a>(
    server: &'a str,
    rack: &HashMap<&'a str, Vec<&'a str>>,
    visited_dac: bool,
    visited_fft: bool,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(n_paths) = cache.get(&(server, visited_dac, visited_fft)) {
        return *n_paths;
    }
    let mut n_paths = 0;

    for output in rack.get(server).unwrap() {
        if *output == "out" {
            if visited_dac && visited_fft {
                n_paths += 1;
            }
        } else {
            n_paths += paths_to_out_with_visits(
                output,
                rack,
                visited_dac || *output == "dac",
                visited_fft || *output == "fft",
                cache,
            );
        }
    }
    cache.insert((server, visited_dac, visited_fft), n_paths);
    n_paths
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut rack = HashMap::new();
    for line in contents.lines() {
        let name = &line[..3];
        let outputs: Vec<_> = line[5..].split(" ").collect();
        rack.insert(name, outputs);
    }
    let rack = rack;
    // println!("Rack: {:?}", rack);

    let mut cache = HashMap::new();
    let paths = paths_to_out("you", &rack, &mut cache);
    println!("Paths from you to out: {}", paths);

    let mut cache = HashMap::new();
    let paths = paths_to_out_with_visits("svr", &rack, false, false, &mut cache);
    println!("Paths from svr to out via dac and fft: {}", paths);
}
