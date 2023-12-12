use nom::{
    bytes::complete::tag, character::complete as cc, multi::separated_list1, sequence::tuple,
    IResult,
};

//513,151 -> 513,155 -> 510,155 -> 510,161 -> 519,161 -> 519,155 -> 515,155 -> 515,151
fn parse_rocks(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (s, v) = separated_list1(tag(" -> "), tuple((cc::i32, tag(","), cc::i32)))(s)?;
    Ok((s, v.into_iter().map(|(x, _, y)| (x - 250, y)).collect()))
}

pub fn parse_input(s: &str) -> IResult<&str, (Vec<Vec<i32>>, i32)> {
    let (s, v) = separated_list1(cc::multispace1, parse_rocks)(s)?;
    let mut grid = vec![vec![0; 501]; 250];
    let mut max_row = 0;
    for rocks in v.iter() {
        let (mut k, mut l) = rocks[0];
        for (i, j) in rocks.iter() {
            for x in k.min(*i)..=k.max(*i) {
                for y in l.min(*j)..=l.max(*j) {
                    grid[y as usize][x as usize] = 1;
                    max_row = max_row.max(y);
                }
            }
            k = *i;
            l = *j;
        }
    }
    Ok((s, (grid, max_row + 2)))
}
