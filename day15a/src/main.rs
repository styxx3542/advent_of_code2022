mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Position};
fn main() {
    let v: Vec<(Position, Position)> = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let target_row = 2_000_000;
    let mut ranges: Vec<(i32, i32)> = v
        .iter()
        .filter_map(|(a, b)| a.col_range(target_row, b))
        .collect();
    ranges.sort_by_key(|p| p.0);
    let mut non_overlapping_ranges: Vec<(&i32, &i32)> = vec![];
    for (start, end) in ranges.iter() {
        if !non_overlapping_ranges.is_empty() && start <= non_overlapping_ranges.last().unwrap().1 {
            let mut last = non_overlapping_ranges.pop().unwrap();
            last.1 = last.1.max(end);
            non_overlapping_ranges.push(last);
        } else {
            non_overlapping_ranges.push((start, end));
        }
    }
    println!("{:?}", non_overlapping_ranges);
    let s: i32 = non_overlapping_ranges
        .into_iter()
        .map(|(a, b)| (*a - *b).abs() + 1)
        .sum();
    println!("{s}");
}
