use std::collections::HashSet;
fn main() {
    let input = include_str!("../input.txt").lines().map(|s| {
        s.split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let set = input.clone().collect::<HashSet<_>>();
    let mut result = set.len() * 6;
    for cube in input {
        let mut new_cube = cube.clone();
        for i in 0..3 {
            new_cube[i] -= 1;
            if set.contains(&new_cube) {
                result -= 1;
            }
            new_cube[i] += 2;
            if set.contains(&new_cube) {
                result -= 1;
            }
            new_cube[i] -= 1;
        }
    }
    println!("{result}");
}
