use std::collections::{HashSet, VecDeque};

fn bfs(input: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut queue = VecDeque::new();
    let (m, n) = (input.len(), input[0].len());
    let mut set = HashSet::new();
    let dir: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (0, -1), (1, 0)];
    queue.push_back(start);
    set.insert(start);
    let mut count: u32 = 0;
    while !queue.is_empty() {
        let l = queue.len();
        for _ in 0..l {
            let (i, j) = queue.pop_front().unwrap();
            for (dx, dy) in dir.iter() {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if 0 <= x
                    && (x as usize) < m
                    && 0 <= y
                    && (y as usize) < n
                    && (input[x as usize][y as usize] as i32) - (input[i][j] as i32) <= 1
                    && !set.contains(&(x as usize, y as usize))
                {
                    let x = x as usize;
                    let y = y as usize;
                    if (x, y) == end {
                        return count + 1;
                    }
                    queue.push_back((x, y));
                    set.insert((x, y));
                }
            }
        }
        count += 1;
    }
    u32::MAX
}
fn main() {
    let mut input = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (m, n) = (input.len(), input[0].len());
    let mut start = Vec::new();

    let mut end = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if input[i][j] == 'S' || input[i][j] == 'a' {
                input[i][j] = 'a';
                start.push((i, j));
            }
            if input[i][j] == 'E' {
                input[i][j] = 'z';
                end = (i, j);
            }
        }
    }
    println!("{}, {}", start[1].0, start[1].1);
    println!(
        "{}",
        start
            .into_iter()
            .map(|s| bfs(&input, s, end))
            .min()
            .unwrap()
    );
}
