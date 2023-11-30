use std::collections::HashSet;
use std::collections::VecDeque;
fn get_start_position(s: &str) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    let mut v: VecDeque<char> = VecDeque::new();
    for (i, c) in s.chars().filter(|c| c.is_alphabetic()).enumerate() {
        while set.contains(&c) {
            let c = v.pop_front().unwrap();
            set.remove(&c);
        }
        v.push_back(c);
        set.insert(c);
        if v.len() == 14 {
            return i + 1;
        }
    }
    0
}
fn main() {
    println!("{}", get_start_position(include_str!("../input.txt")));
}
