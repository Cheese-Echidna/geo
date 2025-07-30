use crate::*;

#[derive(Clone, Debug)]
pub(crate) struct Edge {
    pub start: Point,
    pub end: Point,
    pub weight: f32,
}

pub(crate) fn kruskals_mst(edges: Vec<(Point, Point)>) -> Vec<Edge> {
    let mut forest = DisjointSet::new(edges.len() * 2);
    // Assuming at most 2 unique points per edge
    let mut mst: Vec<Edge> = Vec::new();
    let mut sorted_edges: Vec<Edge> = edges
        .into_iter()
        .map(|(start, end)| Edge {
            start,
            end,
            weight: start.pos.distance(end.pos),
        })
        .collect();

    sorted_edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

    for edge in sorted_edges {
        if forest.find(&edge.start) != forest.find(&edge.end) {
            forest.union(&edge.start, &edge.end);
            mst.push(edge);
        }
    }

    mst
}

// Disjoint set (Union-Find) implementation
#[derive(Clone, Debug)]
struct DisjointSet<T> {
    parents: HashMap<T, T>,
}

impl<T: std::hash::Hash + Eq + Copy> DisjointSet<T> {
    fn new(_size: usize) -> Self {
        DisjointSet {
            parents: HashMap::new(),
        }
    }

    fn find(&mut self, item: &T) -> T {
        let parent = self.parents.get(item).cloned().unwrap_or_else(|| item.clone());
        if parent != *item {
            let root = self.find(&parent);
            self.parents.insert(item.clone(), root.clone());
            root
        } else {
            parent
        }
    }


    fn union(&mut self, a: &T, b: &T) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a != root_b {
            self.parents.insert(root_a, root_b);
        }
    }
}
