use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use rayon::prelude::*;
use petgraph::graph::{Graph, NodeIndex};
use num_complex::Complex64;

#[derive(Debug)]
pub struct Nd {
    id: usize,
    val: f64,
    pos: Complex64,
}

#[derive(Debug)]
pub struct Ed {
    wt: f64,
    flow: f64,
}

pub struct Grf {
    g: Graph<Nd, Ed>,
    idx_map: HashMap<usize, NodeIndex>,
}

impl Grf {
    pub fn new() -> Self {
        Self {
            g: Graph::new(),
            idx_map: HashMap::new(),
        }
    }

    pub fn add_nd(&mut self, id: usize, val: f64, x: f64, y: f64) -> NodeIndex {
        let nd = Nd {
            id,
            val,
            pos: Complex64::new(x, y),
        };
        let idx = self.g.add_node(nd);
        self.idx_map.insert(id, idx);
        idx
    }

    pub fn add_ed(&mut self, from: usize, to: usize, wt: f64) {
        let u = self.idx_map[&from];
        let v = self.idx_map[&to];
        self.g.add_edge(u, v, Ed { wt, flow: 0.0 });
    }

    pub fn mst(&self) -> Vec<(usize, usize, f64)> {
        let mut res = Vec::new();
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::new();

        if let Some(start) = self.g.node_indices().next() {
            seen.insert(start);
            for e in self.g.edges(start) {
                heap.push(Edge::new(start, e.target(), -e.weight().wt));
            }

            while let Some(Edge { u, v, wt }) = heap.pop() {
                if seen.contains(&v) {
                    continue;
                }

                seen.insert(v);
                res.push((
                    self.g[u].id,
                    self.g[v].id,
                    -wt
                ));

                for e in self.g.edges(v) {
                    if !seen.contains(&e.target()) {
                        heap.push(Edge::new(v, e.target(), -e.weight().wt));
                    }
                }
            }
        }
        res
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> f64 {
        let source = self.idx_map[&s];
        let sink = self.idx_map[&t];
        let mut flow = 0.0;

        loop {
            let path = self.find_path(source, sink);
            if path.is_empty() {
                break;
            }

            let mut min_cap = f64::INFINITY;
            for i in 0..path.len()-1 {
                let u = path[i];
                let v = path[i+1];
                let e = self.g.find_edge(u, v).unwrap();
                min_cap = min_cap.min(self.g[e].wt - self.g[e].flow);
            }

            for i in 0..path.len()-1 {
                let u = path[i];
                let v = path[i+1];
                let e = self.g.find_edge(u, v).unwrap();
                self.g[e].flow += min_cap;
            }

            flow += min_cap;
        }
        flow
    }

    fn find_path(&self, s: NodeIndex, t: NodeIndex) -> Vec<NodeIndex> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        let mut prev = HashMap::new();

        seen.insert(s);
        queue.push_back(s);

        while let Some(u) = queue.pop_front() {
            for e in self.g.edges(u) {
                let v = e.target();
                if !seen.contains(&v) && e.weight().wt > e.weight().flow {
                    seen.insert(v);
                    prev.insert(v, u);
                    queue.push_back(v);
                }
            }
        }

        let mut path = Vec::new();
        let mut curr = t;
        while let Some(&p) = prev.get(&curr) {
            path.push(curr);
            curr = p;
            if curr == s {
                path.push(s);
                path.reverse();
                return path;
            }
        }
        Vec::new()
    }
}

#[derive(Debug)]
struct Edge {
    u: NodeIndex,
    v: NodeIndex,
    wt: f64,
}

impl Edge {
    fn new(u: NodeIndex, v: NodeIndex, wt: f64) -> Self {
        Self { u, v, wt }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wt.partial_cmp(&other.wt).unwrap()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.wt == other.wt
    }
}

impl Eq for Edge {}