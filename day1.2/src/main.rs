fn main() {
    let mut arr = include_str!("../input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    arr.sort_unstable();

    println!("{}", arr.into_iter().rev().take(3).sum::<u32>())
}
