use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
pub struct Machine {
    n_lights: usize,
    desired: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

fn min_presses(machine: &Machine) -> usize {
    let current = vec![false; machine.n_lights];
    let mut queue = VecDeque::new();

    // Initial queue for breadth-first search
    for button in &machine.buttons {
        queue.push_back((current.clone(), button, 0));
    }

    loop {
        // we assume that there is a solution…
        let (state, button, presses) = queue.pop_front().unwrap();
        let mut new_state = state.clone();
        for switch in button {
            new_state[*switch] = !new_state[*switch];
        }
        if new_state == machine.desired {
            return presses + 1;
        } else {
            for b in &machine.buttons {
                queue.push_back((new_state.clone(), b, presses + 1));
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut machines = vec![];
    for line in contents.lines() {
        let n_lights = line.find(']').unwrap() - 1;
        let desired: Vec<_> = line[1..n_lights + 1].chars().map(|c| c == '#').collect();
        let button_def = &line[n_lights + 3..line.find('{').unwrap() - 1];
        let mut buttons = vec![];
        for b_def in button_def.split(" ") {
            let values: Vec<_> = b_def[1..b_def.len() - 1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            buttons.push(values);
        }
        machines.push(Machine {
            n_lights: n_lights,
            desired: desired,
            buttons: buttons,
        })
    }
    println!("Machines: ");
    let mut presses = vec![];
    for machine in machines {
        println!("{:?}", machine);
        let min_press = min_presses(&machine);
        println!("Minimum presses: {}", min_press);
        presses.push(min_press);
    }
    println!("Total presses: {}", presses.iter().sum::<usize>())
}
