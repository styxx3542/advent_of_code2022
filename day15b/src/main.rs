mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Position};
fn merge_overlapping_subintervals(mut ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    ranges.sort_by_key(|p| p.0);
    let mut non_overlapping_ranges: Vec<(i32, i32)> = vec![];
    for (start, end) in ranges.iter() {
        if !non_overlapping_ranges.is_empty() && *start <= non_overlapping_ranges.last().unwrap().1
        {
            let mut last = non_overlapping_ranges.pop().unwrap();
            last.1 = last.1.max(*end);
            non_overlapping_ranges.push(last);
        } else {
            non_overlapping_ranges.push((*start, *end));
        }
    }
    non_overlapping_ranges
}

fn main() {
    let v: Vec<(Position, Position)> = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    for target_row in 0..=4_000_000 {
        let ranges: Vec<(i32, i32)> = v
            .iter()
            .filter_map(|(a, b)| a.col_range(target_row, b))
            .collect();
        let non_overlapping_ranges = merge_overlapping_subintervals(ranges);
        if non_overlapping_ranges.len() > 1 {
            let x: i64 = (non_overlapping_ranges[1].0 - 1).into();
            let y: i64 = target_row.into();
            println!("x = {x}, y = {y}");
            let distress_signal = x * 4_000_000 + y;
            println!("Distress signal - {}", distress_signal);
            break;
        }
    }
}
