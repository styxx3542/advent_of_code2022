use std::collections::VecDeque;

fn get_start_position(s: &str) -> usize {
    let mut v: VecDeque<char> = VecDeque::new();
    for (i, c) in s.chars().filter(|c| c.is_alphabetic()).enumerate() {
        while v.contains(&c) {
            v.pop_front();
        }
        v.push_back(c);
        if v.len() == 4 {
            return i + 1;
        }
    }
    0
}
fn main() {
    println!("{}", get_start_position(include_str!("../input.txt")));
}
