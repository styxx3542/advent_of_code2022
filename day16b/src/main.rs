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

fn partition(indices: Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let n = indices.len();
    let mut v = vec![];
    for i in 0..1 << n {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                a.push(indices[j]);
            } else {
                b.push(indices[j]);
            }
        }
        if a.len() > 0 && b.len() > 0 {
            v.push((a, b));
        }
    }
    v
}
fn main() {
    let map = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;

    let graph = build_graph(&map);
    let graph = floyd_warshalls(graph);

    let indices_with_positive_flow_rate = map
        .values()
        .filter(|v| v.flow_rate != 0)
        .map(|v| v.index)
        .collect::<Vec<_>>();

    let mut v = 0;
    let mut cache = HashMap::new();
    for (l, r) in partition(indices_with_positive_flow_rate) {
        let (mut l_visited, mut r_visited) = (0, 0);
        for i in &l {
            r_visited |= 1 << i;
        }
        for i in &r {
            l_visited |= 1 << i;
        }
        let left = dfs(
            &graph,
            map.values().find(|v| v.id == "AA").unwrap().index,
            &map,
            l_visited,
            26,
            &mut cache,
        );

        let right = dfs(
            &graph,
            map.values().find(|v| v.id == "AA").unwrap().index,
            &map,
            r_visited,
            26,
            &mut cache,
        );
        v = v.max(left + right);
    }

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
