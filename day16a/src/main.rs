mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Node};
use std::collections::HashMap;

fn floyd_warshalls(mut dist: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }
    dist
}

fn build_graph(map: &HashMap<usize, Node>) -> Vec<Vec<u32>> {
    let n = map.len();
    let mut graph = vec![vec![u32::MAX; n]; n];

    for i in 0..n {
        graph[i][i] = 0;
        let node = map.get(&i).unwrap();
        for j in node
            .neighbors
            .iter()
            .map(|n| map.values().find(|v| v.id == *n).unwrap().index)
        {
            graph[i][j] = 1;
        }
    }

    graph
}

fn main() {
    let map = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;

    let graph = build_graph(&map);
    let graph = floyd_warshalls(graph);
    let mut cache = HashMap::new();
    let v = dfs(
        &graph,
        map.values().find(|v| v.id == "AA").unwrap().index,
        &map,
        0,
        30,
        &mut cache,
    );
    println!("{}", v);
}

fn dfs(
    graph: &Vec<Vec<u32>>,
    i: usize,
    map: &HashMap<usize, Node>,
    visited: u64,
    time: u32,
    cache: &mut HashMap<(String, u64, u32), u32>,
) -> u32 {
    let node = map.get(&i).unwrap();
    if let Some(v) = cache.get(&(node.id.clone(), visited, time)) {
        return *v;
    }
    if time < 2 {
        return 0;
    }
    if time == 2 {
        return node.flow_rate.into();
    }
    let mut visited = visited;
    let mut res = 0;
    for j in 0..graph.len() {
        let steps = graph[i][j];
        if graph[i][j] == u32::MAX || steps > time - 1 || i == j {
            continue;
        }
        if visited & (1 << j) != 0 {
            continue;
        }
        let neighbor = map.get(&j).unwrap();
        if neighbor.flow_rate != 0 {
            visited |= 1 << j;
            res = res.max(
                dfs(graph, j, map, visited, time - steps - 1, cache)
                    + (time - steps - 1) * neighbor.flow_rate,
            );
            visited &= !(1 << j);
        }
    }
    cache.insert((node.id.clone(), visited, time), res);
    res
}
