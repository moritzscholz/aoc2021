use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub type Cost = u32;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
pub struct Edge {
    #[allow(dead_code)]
    from: Coordinate,
    to: Coordinate,
    cost: Cost,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(c: (usize, usize)) -> Self {
        Self { x: c.0, y: c.1 }
    }
}

impl Edge {
    pub fn new(from: Coordinate, to: Coordinate, cost: Cost) -> Edge {
        Edge { from, to, cost }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: Cost,
    position: Coordinate,
}

// Turn max-heap to min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Djikstra
pub fn shortest_path(
    edges: &HashMap<Coordinate, Vec<Edge>>,
    start: Coordinate,
    goal: Coordinate,
) -> Option<Cost> {
    let mut dist: HashMap<Coordinate, Cost> =
        edges.iter().map(|(&p, _)| (p, Cost::MAX)).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost.
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Check the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for edge in &edges[&position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.to,
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}
