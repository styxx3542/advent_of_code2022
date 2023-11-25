fn get_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return (item as u32) - ('a' as u32) + 1;
    }
    (item as u32) - ('A' as u32) + 27
}

fn get_score(first: &str, second: &str, third: &str) -> u32 {
    first
        .chars()
        .filter(|ch| second.contains(*ch) && third.contains(*ch))
        .map(get_priority)
        .next()
        .unwrap()
}
fn main() {
    let v: u32 = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|v| {
            let (first, second, third) = (v[0], v[1], v[2]);
            get_score(first, second, third)
        })
        .sum();
    println!("{v}");
}
