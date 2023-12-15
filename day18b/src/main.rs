use std::collections::{HashMap, HashSet};

fn is_out_of_bounds(cube: &Vec<i32>, bounds: &Vec<(i32, i32)>) -> bool {
    for i in 0..3 {
        if cube[i] < bounds[i].0 || cube[i] > bounds[i].1 {
            return true;
        }
    }
    false
}
fn is_internal(
    cube: &Vec<i32>,
    set: &HashSet<Vec<i32>>,
    cache: &mut HashMap<Vec<i32>, i32>,
    bounds: &Vec<(i32, i32)>,
) -> bool {
    if cache.contains_key(cube) {
        return *cache.get(cube).unwrap() != -1;
    }
    let dir = [1, -1];
    let mut next_cube = cube.clone();
    cache.insert(cube.clone(), 0);
    for i in 0..3 {
        for d in dir.iter() {
            next_cube[i] += *d;
            if is_out_of_bounds(cube, bounds)
                || !(set.contains(&next_cube) || is_internal(&next_cube, set, cache, bounds))
            {
                cache.insert(cube.clone(), -1);
                return false;
            }
            next_cube[i] -= *d;
        }
    }
    cache.insert(cube.clone(), 1);
    true
}
fn main() {
    let input = include_str!("../input.txt").lines().map(|s| {
        s.split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let bounds: Vec<(i32, i32)> = (0..3)
        .map(|i| {
            input
                .clone()
                .map(|v| v[i as usize])
                .fold((i32::MAX, i32::MIN), |acc, x| (acc.0.min(x), acc.1.max(x)))
        })
        .collect();

    let set = input.clone().collect::<HashSet<_>>();
    let mut result = set.len() * 6;
    let mut cache = HashMap::new();
    for cube in input {
        let mut new_cube = cube.clone();
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
    println!("{result}");
}
