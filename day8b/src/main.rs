fn get_score(grid: &Vec<Vec<u32>>, row: &u32, col: &u32) -> u32 {
    if *row == 0 || *col == 0 {
        return 0;
    }
    let n = grid.len() as i32;
    let moves = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut score = 1;
    let tree = grid[*row as usize][*col as usize];
    for (dx, dy) in moves.into_iter() {
        let (mut i, mut j) = (*row as i32, *col as i32);
        i += dx;
        j += dy;
        let mut direction_score = 0;
        while 0 <= i && i < n && 0 <= j && j < n {
            direction_score += 1;
            if grid[i as usize][j as usize] >= tree {
                break;
            }
            i += dx;
            j += dy;
        }
        score *= direction_score;
        if score == 0 {
            break;
        }
    }
    score
}

fn get_grid(input: &str) -> Vec<Vec<u32>> {
    let mut vec: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        vec.push(row);
    }
    vec
}
fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<u32>> = get_grid(input);
    let max = grid
        .iter()
        .enumerate()
        .flat_map(|(row, v)| {
            let row = &(row as u32);
            v.iter()
                .enumerate()
                .map(|(col, _)| get_score(&grid, row, &(col as u32)))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();
    println!("{max}");
}
