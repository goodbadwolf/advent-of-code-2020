use std::collections::{HashMap, HashSet};
use std::fs;

use petgraph::{visit::EdgeRef, Graph, graph::NodeIndex, visit::Bfs};

fn parse_bag_types(inputs: &String) -> Vec<String> {
    let mut bags: HashSet<String> = HashSet::new();

    for line in inputs.lines() {
        let words:Vec<&str> = line.split_ascii_whitespace().collect();
        for i in 0..words.len() {
            if words[i].contains("bag") {
                let bag = words[i - 2].to_owned() + " " + words[i - 1];
                if bag != "no other" {
                    bags.insert(bag);
                }
            }
        }
    }

    bags.into_iter().collect()
}

fn build_bag_graph(inputs: &String, bags: &Vec<String>) -> (HashMap<String, NodeIndex>, Graph<u32, u32>, Graph<u32, u32>) {
    let mut graph = Graph::new();
    let mut graph_rev = Graph::new();
    let mut node_indices = HashMap::new();

    for bag in bags {
        let bag_idx = graph.add_node(0);
        graph_rev.add_node(0);
        node_indices.insert(bag.clone(), bag_idx);
    }

    for line in inputs.lines() {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let source = words[0].to_owned() + " " + words[1];
        let source_i = node_indices[&source];
        for i in 4..words.len() {
            if !words[i].contains("bag") {
                continue;
            }
            let dest = words[i - 2].to_owned() + " " + words[i - 1];
            if dest == "no other" {
                continue;
            }
            let dest_i = node_indices[&dest];
            let weight = words[i - 3].parse::<u32>().unwrap();
            graph.add_edge(source_i, dest_i, weight);
            graph_rev.add_edge(dest_i, source_i, weight);
        }
    }
    
    (node_indices, graph, graph_rev)
}

fn find_how_many_ways_gold(bag_node_indices: &HashMap<String, NodeIndex>, graph_rev: &Graph<u32, u32>) -> u32 {
    let gold_index = bag_node_indices["shiny gold"];
    let mut ways = 0_u32;

    let mut bfs_traversal = Bfs::new(&graph_rev, gold_index);
    while let Some(_) = bfs_traversal.next(&graph_rev) {
        ways += 1;
    }

    ways - 1
}

fn find_nesting_count(bag_node_indices: &HashMap<String, NodeIndex>, graph: &Graph<u32, u32>, bag_idx: NodeIndex, cached_counts: &mut HashMap<NodeIndex, u32>, depth: u32) -> u32 {
    if cached_counts.contains_key(&bag_idx) {
        cached_counts[&bag_idx]
    } else {
        let mut count = 0_u32;

        for edge in graph.edges_directed(bag_idx, petgraph::EdgeDirection::Outgoing) {
            count += edge.weight();
            count += edge.weight() * find_nesting_count(bag_node_indices, graph, edge.target(), cached_counts, depth + 1);
        }

        cached_counts.insert(bag_idx, count);

        count
    }
}

fn main() {
    let inputs = fs::read_to_string("input.txt").expect("Couldn't read input file");
    let bags = parse_bag_types(&inputs);
    let (bag_node_indices, graph, graph_rev) = build_bag_graph(&inputs, &bags);
    let gold_ways = find_how_many_ways_gold(&bag_node_indices, &graph_rev);
    println!("How many ways for shiny gold: {}", gold_ways);

    let gold_idx = bag_node_indices["shiny gold"];
    let gold_count = find_nesting_count(&bag_node_indices, &graph, gold_idx, &mut (HashMap::new()), 0);
    println!("Shiny gold nesting count: {}", gold_count);
}