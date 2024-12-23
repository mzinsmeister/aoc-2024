use std::collections::{BTreeMap, BTreeSet};

use aoclib::read_input;
use petgraph::graph::{NodeIndex, UnGraph};

/*  
algorithm BronKerbosch1(R, P, X) is
    if P and X are both empty then
        report R as a maximal clique
    for each vertex v in P do
        BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        P := P \ {v}
        X := X ⋃ {v}
*/

fn bron_kerbosch(graph: &UnGraph<(), ()>, r: BTreeSet<NodeIndex>, mut p: BTreeSet<NodeIndex>, mut x: BTreeSet<NodeIndex>, results: &mut Vec<BTreeSet<NodeIndex>>) {
    if p.is_empty() && x.is_empty() {
        results.push(r.clone());
    }
    for _ in 0..p.len() {
        let v = *p.first().unwrap();
        let mut new_r = r.clone();
        new_r.insert(v);
        let neighbors = graph.neighbors(v).collect();
        let new_p = p.intersection(&neighbors).cloned().collect();
        let new_x = x.intersection(&neighbors).cloned().collect();
        bron_kerbosch(graph, new_r, new_p, new_x, results);
        p.pop_first();
        x.insert(v);
    }
}

fn main() {
    let input = read_input("input.txt");

    let input: Vec<([char; 2], [char;2]) >= input.trim_last_newline().lines().map(|line| {
        let parts = line.split_once('-').unwrap();
        let from = parts.0.chars().collect::<Vec<_>>().try_into().unwrap();
        let to = parts.1.chars().collect::<Vec<_>>().try_into().unwrap();
        (from, to)
    }).collect();

    let mut graph = UnGraph::<(), ()>::new_undirected();

    let mut nodes = BTreeSet::<[char; 2]>::new();

    for (from, to) in input.iter() {
        nodes.insert(*from);
        nodes.insert(*to);
    }

    let nodes = nodes.into_iter().collect::<Vec<_>>();

    let mut node_index = BTreeMap::<[char; 2], NodeIndex>::new();
    let mut reverse_index = BTreeMap::<NodeIndex, [char; 2]>::new();

    for n in nodes.iter() {
        let ni = graph.add_node(());
        node_index.insert(*n, ni);
        reverse_index.insert(ni, *n);
    }

    for (from, to) in input.iter() {
        let from = node_index[from];
        let to = node_index[to];
        graph.add_edge(from, to, ());
    }

    let t_nodes = nodes.iter().filter(|n| n[0] == 't');

    let mut found = BTreeSet::new();

    for t_node in t_nodes {
        let node = node_index[t_node];

        // Neighbors are candidates
        let neighbors: Vec<NodeIndex> = graph.neighbors(node).collect();

        for (i, n1) in neighbors.iter().enumerate() {
            for n2 in neighbors.iter().skip(i+1) {
                if graph.contains_edge(*n1, *n2) {
                    let n1_name = reverse_index[n1];
                    let n2_name = reverse_index[n2];
                    let mut component = [*t_node, n1_name, n2_name];
                    component.sort();
                    found.insert(component);
                }
            }
        }
    }

    println!("{}", found.len());

    let mut cliques = vec![];

    let node_indexes = node_index.values().cloned().collect();

    bron_kerbosch(&graph, BTreeSet::new(), node_indexes, BTreeSet::new(), &mut cliques);

    let result = cliques.iter().max_by_key(|c| c.len()).unwrap();

    let mut result: Vec<String> = result.iter().map(|ni| reverse_index[ni].iter().collect()).collect();

    result.sort();

    println!("{}", result.join(","));
    
}
