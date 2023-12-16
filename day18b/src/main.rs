use std::collections::{HashMap, HashSet};

fn is_out_of_bounds(cube: &[i32], bounds: &[(i32, i32)]) -> bool {
    for i in 0..3 {
        if cube[i] < bounds[i].0 || cube[i] > bounds[i].1 {
            return true;
        }
    }
    false
}

fn is_internal(
    cube: &[i32],
    set: &HashSet<Vec<i32>>,
    cache: &mut HashMap<Vec<i32>, i32>,
    bounds: &[(i32, i32)],
) -> bool {
    if let Some(value) = cache.get(cube) {
        return *value != -1;
    }
    let dir = [1, -1];
    let mut next_cube = cube.to_vec();
    cache.insert(cube.to_vec(), 0);
    for i in 0..3 {
        for d in &dir {
            next_cube[i] += *d;
            if is_out_of_bounds(&next_cube, bounds)
                || !(set.contains(&next_cube) || is_internal(&next_cube, set, cache, bounds))
            {
                cache.insert(cube.to_vec(), -1);
                return false;
            }
            next_cube[i] -= *d;
        }
    }
    cache.insert(cube.to_vec(), 1);
    true
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|s| {
            s.split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let bounds: Vec<(i32, i32)> = (0..3)
        .map(|i| {
            input
                .iter()
                .map(|v| v[i as usize])
                .fold((i32::MAX, i32::MIN), |acc, x| (acc.0.min(x), acc.1.max(x)))
        })
        .collect();

    let set: HashSet<Vec<i32>> = input.iter().cloned().collect();
    let mut result = set.len() * 6;
    let mut cache = HashMap::new();
    for cube in &input {
        let mut new_cube = cube.to_vec();
        for i in 0..3 {
            new_cube[i] -= 1;
            if set.contains(&new_cube) || is_internal(&new_cube, &set, &mut cache, &bounds) {
                result -= 1;
            }
            new_cube[i] += 2;
            if set.contains(&new_cube) || is_internal(&new_cube, &set, &mut cache, &bounds) {
                result -= 1;
            }
            new_cube[i] -= 1;
        }
    }
    println!("{}", result);
}
