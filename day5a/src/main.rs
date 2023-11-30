use itertools::Itertools;

fn main() {
    let (boxes, rest) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut stacks = vec![vec![]; 9];
    for l in boxes.lines().rev().skip(1).map(str::as_bytes) {
        for i in 0..stacks.len() {
            let c = l[i * 4 + 1];
            if c.is_ascii_alphabetic() {
                stacks[i].push(c as char);
            }
        }
    }
    let instructions = rest
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();
    for (times, from, to) in instructions {
        for _ in 0..times {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
    for v in stacks.iter() {
        let top = v.last().unwrap();
        print!("{top}");
    }
    println!();
}
