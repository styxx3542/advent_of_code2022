use std::collections::HashSet;
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
fn main() {
    let input = include_str!("../input.txt").as_bytes();
    let rocks = vec![
        vec![[2, 0], [3, 0], [4, 0], [5, 0]],
        vec![[2, 1], [3, 1], [3, 2], [3, 0], [4, 1]],
        vec![[2, 0], [3, 0], [4, 0], [4, 1], [4, 2]],
        vec![[2, 0], [2, 1], [2, 2], [2, 3]],
        vec![[2, 0], [3, 0], [2, 1], [3, 1]],
    ];
    let duration = 2022;
    let mut index = 0;
    let mut highest = 0;
    let mut map = (0..7).map(|x| (x, 0)).collect::<HashSet<_>>();
    for i in 0..duration {
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
                map.extend(rock.iter().map(|v| (v[0], v[1])));
                highest = rock.iter().map(|v| v[1]).max().unwrap().max(highest);
                break;
            }
        }
    }
    println!("{highest}");
}
