fn main() {
    let v = include_str!("../input.txt")
        .split("\n")
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a_start, a_end) = a.split_once('-').unwrap();
            let (b_start, b_end) = b.split_once('-').unwrap();
            let a_start = a_start.parse::<u32>().unwrap();
            let a_end = a_end.parse::<u32>().unwrap();
            let b_start = b_start.parse::<u32>().unwrap();
            let b_end = b_end.parse::<u32>().unwrap();
            if (a_start <= b_start && a_end >= b_start) || (b_start <= a_start && b_end >= a_start)
            {
                return true;
            }
            false
        })
        .count();

    println!("{v}");
}
