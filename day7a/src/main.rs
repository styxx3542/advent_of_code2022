use atoi;
use std::iter::Peekable;
fn main() {
    let mut sum = 0;
    let input = include_str!("../input.txt");
    process(&mut input.lines().peekable(), &mut sum);
    println!("{sum}");
}

fn process<I>(input: &mut Peekable<I>, sum: &mut u64) -> u64
where
    I: Iterator<Item = &'static str>,
{
    let mut size = 0;
    while let Some(line) = input.next() {
        match line {
            "$ cd .." => {
                break;
            }
            _ if &line[0..3] == "$ l" => {
                size = std::iter::from_fn(|| {
                    input.next_if(|line| line.chars().next().unwrap() != '$')
                })
                .filter(|i| i.chars().next().unwrap() != 'd')
                .filter_map(|i| atoi::atoi::<u64>(i.split(' ').next().unwrap().as_bytes()))
                .sum();
            }
            _ => {
                size += process(input, sum);
            }
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}
