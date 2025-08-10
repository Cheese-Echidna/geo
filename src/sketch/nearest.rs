use std::collections::{HashMap, VecDeque};
use crate::sketch::*;


pub(crate) fn bfs(edges: Vec<(Point, Point)>) -> (Vec<(Point, Point)>, Vec<(Point, usize)>) {
    // Convert edge list to adjacency list
    let mut graph = HashMap::new();
    for (a, b) in &edges {
        graph.entry(a.clone()).or_insert_with(Vec::new).push(b.clone());
        graph.entry(b.clone()).or_insert_with(Vec::new).push(a.clone());
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut tree_edges = Vec::new();
    let mut distances = Vec::new();

    // Assume the first point in the edge list is the starting node
    if let Some((start, _)) = edges.first() {
        queue.push_back(start.clone());
        visited.insert(start.clone(), true);
        distances.push((start.clone(), 0));
    }

    while let Some(current) = queue.pop_front() {
        let current_distance = distances.iter().find(|(p, _)| p == &current).unwrap().1;

        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    queue.push_back(neighbor.clone());
                    visited.insert(neighbor.clone(), true);
                    tree_edges.push((current.clone(), neighbor.clone()));
                    distances.push((neighbor.clone(), current_distance + 1));
                }
            }
        }
    }

    (tree_edges, distances)
}
