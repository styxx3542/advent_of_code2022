mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::parse_input;

fn main() {
    let (mut grid, max_row) = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    for i in 0..501 {
        grid[max_row as usize][i] = 1;
    }
    for count in 0..50000 {
        let (mut i, mut j) = (0, 250);
        if grid[i][j] == 1 {
            println!("{count}");
            break;
        }
        let dir: Vec<(i32, i32)> = vec![(1, 0), (1, -1), (1, 1)];
        loop {
            let mut flag = 0;
            for (dx, dy) in dir.iter() {
                let x = (i as i32) + dx;
                let y = (j as i32) + dy;
                if grid[x as usize][y as usize] != 1 {
                    i = x as usize;
                    j = y as usize;
                    flag = 0;
                    break;
                }
                flag = 1;
            }
            if flag != 0 {
                break;
            }
        }
        grid[i][j] = 1;
    }
}
