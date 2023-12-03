use std::collections::HashSet;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn find_min_pos(input: String) -> HashSet<Position> {
    let mut h: HashSet<Position> = HashSet::new();
    let n = input.lines().count();
    let mut col_wise_max: Vec<i32> = vec![-1; n];
    for (row, line) in input.lines().enumerate() {
        let mut curr_max: i32 = -1;
        for (col, c) in line.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as i32;
            if c > curr_max {
                curr_max = c;
                h.insert(Position { row, col });
            }
            if c > col_wise_max[col] {
                col_wise_max[col] = c;
                h.insert(Position { row, col });
            }
        }
    }
    h
}
fn main() {
    let input = include_str!("../input.txt");
    let set_a = find_min_pos(input.to_string());
    let n = input.lines().count();
    let rev_input: String = input.chars().rev().collect::<String>().into();
    let set_b = find_min_pos(rev_input);
    let mut count = set_a.len() + set_b.len();
    for pos in set_a.iter() {
        let rev_pos = Position {
            row: n - 1 - pos.row,
            col: n - 1 - pos.col,
        };
        if set_b.contains(&rev_pos) {
            count -= 1;
        }
    }
    println!("{count}");
}
