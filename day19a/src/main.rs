mod parse;
use std::collections::VecDeque;
fn main() {
    let input = include_str!("../input.txt");
    let blueprints = parse::parse(input);
    let res: usize = blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * usize::from(geodes))
        .sum();
    println!("res: {}", res);
}

struct State {
    bots: [u16; 4],
    elapsed: u16,
    ores: [u16; 4],
}

fn max_geodes(blueprint: &[[u16; 4]; 4]) -> u16 {
    let mut max_geodes = 0;
    let max_time = 24;
    let mut queue: VecDeque<State> = VecDeque::new();
    let initial_state = State {
        bots: [1, 0, 0, 0],
        elapsed: 0,
        ores: [0, 0, 0, 0],
    };
    queue.push_back(initial_state);
    while let Some(State {
        bots,
        elapsed,
        ores,
    }) = queue.pop_front()
    {
        for i in 0..blueprint.len() {
            let costs = &blueprint[i];
            let wait_time = (0..3)
                .map(|idx| match costs[idx] {
                    cost if cost <= ores[idx] => 0,
                    _ if bots[idx] == 0 => max_time + 1,
                    _ => (costs[idx] - ores[idx] + bots[idx] - 1) / bots[idx],
                })
                .max()
                .unwrap();
            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }
            let mut new_ores = [0; 4];
            for idx in 0..bots.len() {
                new_ores[idx] = ores[idx] + bots[idx] * (wait_time + 1) - costs[idx]
            }
            let mut new_bots = bots;
            new_bots[i] += 1;
            queue.push_back(State {
                ores: new_ores,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }
        let geodes = ores[3] + bots[3] * (max_time - elapsed);
        max_geodes = max_geodes.max(geodes);
    }
    max_geodes
}
