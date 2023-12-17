use std::collections::HashMap;

fn update_map(state: &Vec<(usize, i32)>, map: &mut HashMap<usize, usize>) {
    for (i, (j, _)) in state.iter().enumerate() {
        map.insert(*j, i);
    }
}

fn calculate_final_position(element: i32, initial_pos: usize, length: usize) -> usize {
    let mut v = initial_pos as i32 + element;
    let length = length as i32 - 1;
    if v <= 0 {
        v = length - v.abs() % length;
    } else {
        v %= length;
    }
    v as usize
}
fn main() {
    let input: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = input.len();
    let mut state: Vec<(usize, i32)> = input.clone().into_iter().enumerate().collect();
    let mut map: HashMap<usize, usize> = ((0..n).zip(0..n)).collect();
    for (i, element) in input.into_iter().enumerate() {
        let initial_pos = map[&i];
        let final_pos = calculate_final_position(element, initial_pos, n);
        if initial_pos == final_pos {
            continue;
        }
        let removed = state.remove(initial_pos);
        state.insert(final_pos, removed);
        update_map(&state, &mut map);
    }
    let start = state.iter().position(|v| v.1 == 0).unwrap();
    let (a, b, c) = (
        state[(start + 1000) % n].1,
        state[(start + 2000) % n].1,
        state[(start + 3000) % n].1,
    );
    println!("{}, {}, {}, {}", a, b, c, a + b + c);
}
