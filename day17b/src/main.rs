use std::collections::{HashMap, HashSet};
fn move_rock(rock: &Vec<[i32; 2]>, x: i32, y: i32) -> Vec<[i32; 2]> {
    rock.iter().map(|v| [v[0] + x, v[1] + y]).collect()
}

fn check_valid_rock(rock: &Vec<[i32; 2]>, map: &HashSet<(i32, i32)>) -> bool {
    for (x, y) in rock.iter().map(|v| (v[0], v[1])) {
        if !(x >= 0 && x < 7 && !map.contains(&(x, y))) {
            return false;
        }
    }
    true
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct StateKey {
    relative_heights: [usize; 7],
    next_block_index: usize,
    jet_index: usize,
}

struct StateValue {
    max_height: i32,
    iteration: usize,
}

fn main() {
    let input = include_str!("../input.txt").as_bytes();
    let rocks = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ];
    let mut index = 0;
    let mut highest: i32 = 0;
    let mut map = (0..7).map(|x| (x, 0)).collect::<HashSet<_>>();
    let mut i = 0;
    let mut max_heights = [0usize; 7];
    let mut cache: HashMap<StateKey, StateValue> = HashMap::new();
    let mut cycle_length;
    let mut cycle_height = 0;
    let mut num_cycles = 0;
    let mut cycle_found = false;
    while i < 1000000000000 {
        let mut rock = rocks[i % 5].clone();
        let adjustment = highest + 4;
        rock = move_rock(&rock, 0, adjustment); // move rock to 4 units above highest rock
        loop {
            let direction = match input[index] {
                b'<' => -1,
                b'>' => 1,
                _ => panic!("Invalid input"),
            };
            index = (index + 1) % input.len();
            let pushed_rock = move_rock(&rock, direction, 0);
            if check_valid_rock(&pushed_rock, &map) {
                rock = pushed_rock;
            }
            let down_rock = move_rock(&rock, 0, -1);
            if check_valid_rock(&down_rock, &map) {
                rock = down_rock;
            } else {
                map.extend(rock.iter().map(|v| {
                    max_heights[v[0] as usize] = max_heights[v[0] as usize].max(v[1] as usize);
                    (v[0], v[1])
                }));
                highest = rock.iter().map(|v| v[1]).max().unwrap().max(highest);
                break;
            }
        }
        i += 1;
        if !cycle_found {
            let min_height = max_heights.iter().min().unwrap();
            let state_key = StateKey {
                relative_heights: max_heights.clone().map(|m| m - min_height),
                next_block_index: i % 5,
                jet_index: index.clone(),
            };
            let state_value = StateValue {
                max_height: highest.clone(),
                iteration: i.clone(),
            };

            if cache.contains_key(&state_key) {
                cycle_found = true;
                let prev_value = cache.get(&state_key).unwrap();
                cycle_length = i - prev_value.iteration;
                cycle_height = highest - prev_value.max_height;
                num_cycles = (1000000000000 - i) / cycle_length;
                i += num_cycles * cycle_length;
            } else {
                cache.insert(state_key, state_value);
            }
        }
    }
    let v: u64 = highest as u64 + (num_cycles as u64 * cycle_height as u64);
    println!("{v}");
}
