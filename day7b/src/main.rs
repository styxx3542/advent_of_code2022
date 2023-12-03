use atoi;
use std::iter::Peekable;
fn main() {
    let mut sum = 0;
    let mut directories: Vec<u64> = Vec::new();
    let input = include_str!("../input.txt");
    let total_size = process(&mut input.lines().peekable(), &mut sum, &mut directories);
    let free_space = 70_000_000 - total_size;
    let space_needed = 30_000_000 - free_space;
    let min_needed = directories
        .into_iter()
        .min_by_key(|x| {
            if *x < space_needed {
                return u64::MAX;
            }
            *x
        })
        .unwrap();
    println!("{min_needed}");
}

fn process<I>(input: &mut Peekable<I>, sum: &mut u64, directories: &mut Vec<u64>) -> u64
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
                size += process(input, sum, directories);
            }
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    directories.push(size);
    size
}
