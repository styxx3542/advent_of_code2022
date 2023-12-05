fn process(instruction: &str, x: i32) -> (u32, i32) {
    if instruction.contains("noop") {
        return (0, x);
    }
    let (_, value) = instruction.split_once(' ').unwrap();
    let value: i32 = value.parse().unwrap();
    (1, x + value)
}

fn main() {
    let mut input = include_str!("../input.txt").lines();
    let mut wait_time = 0;
    let mut value = 1;
    let mut next_pos = 1;
    for i in 0..240 {
        let cycle = i % 40;
        if cycle == 0 {
            println!();
        }
        if wait_time != 0 {
            wait_time -= 1;
        } else {
            value = next_pos;
            (wait_time, next_pos) = process(input.next().unwrap(), value);
        }
        if (value - cycle).abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!();
}
