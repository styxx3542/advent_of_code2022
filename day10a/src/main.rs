fn process(instruction: &str, x: i32) -> (u32, i32) {
    if instruction.contains("noop") {
        return (1, x);
    }
    let (_, value) = instruction.split_once(' ').unwrap();
    let value: i32 = value.parse().unwrap();
    (2, x + value)
}

fn main() {
    let input = include_str!("../input.txt");
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut time: u32 = 0;
    let mut value: i32 = 1;
    let mut index: usize = 0;
    let mut sum = 0;
    for line in input.lines() {
        let (inc, x) = process(line, value);
        time += inc;
        if time >= cycles[index] {
            sum += value * cycles[index] as i32;
            index += 1;
        }
        if index == cycles.len() {
            break;
        }
        value = x;
    }
    println!("{sum}");
}
