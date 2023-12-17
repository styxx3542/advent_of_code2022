use std::collections::HashMap;

fn update_map(state: &Vec<(usize, i64)>, map: &mut HashMap<usize, usize>) {
    for (i, (j, _)) in state.iter().enumerate() {
        map.insert(*j, i);
    }
}

fn calculate_final_position(element: i64, initial_pos: usize, length: usize) -> usize {
    let v = initial_pos as i64 + element;
    let length = length as i64;
    v.rem_euclid(length - 1) as usize
}
fn main() {
    let input: Vec<i64> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i64>().unwrap() * 811589153)
        .collect();
    let n = input.len();
    let mut state: Vec<(usize, i64)> = input.clone().into_iter().enumerate().collect();
    let mut map: HashMap<usize, usize> = ((0..n).zip(0..n)).collect();
    for _ in 0..10 {
        for (i, element) in input.clone().into_iter().enumerate() {
            let initial_pos = map[&i];
            let final_pos = calculate_final_position(element, initial_pos, n);
            let removed = state.remove(initial_pos);
            state.insert(final_pos, removed);
            update_map(&state, &mut map);
        }
    }
    let start = state.iter().position(|v| v.1 == 0).unwrap();
    let (a, b, c) = (
        state[(start + 1000) % n].1,
        state[(start + 2000) % n].1,
        state[(start + 3000) % n].1,
    );
    println!("{}, {}, {}, {}", a, b, c, a + b + c);
}
