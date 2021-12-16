mod pathfinding;
use crate::file_handler::read_lines;
use pathfinding::{shortest_path, Coordinate, Cost, Edge};
use std::collections::HashMap;
use std::path::Path;

pub struct Cave {
    // Entry (p, [e]) is all edges leaving point p.
    edges: HashMap<Coordinate, Vec<Edge>>,

    max_x: usize,
    max_y: usize,
}

impl Cave {
    pub fn from_grid_file<P>(file: P) -> Cave
    where
        P: AsRef<Path>,
    {
        let lines = read_lines(file).expect("Error reading file.");
        let mut nodes: HashMap<Coordinate, Cost> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (line_idx, line) in lines.into_iter().enumerate() {
            max_x = max_x.max(line_idx);

            for (node_idx, char) in line
                .expect("Error reading line")
                .chars()
                .into_iter()
                .enumerate()
            {
                max_y = max_y.max(node_idx);
                let value = char.to_digit(10).expect("Error parsing number.");
                nodes.insert(Coordinate::from((line_idx, node_idx)), value);
            }
        }

        let mut edges: HashMap<Coordinate, Vec<Edge>> = HashMap::new();
        for (p, _) in nodes.iter() {
            let mut qs: Vec<Coordinate> = vec![];

            if p.x > 0 {
                qs.push(Coordinate::new(p.x - 1, p.y));
            }

            if p.y > 0 {
                qs.push(Coordinate::new(p.x, p.y - 1));
            }

            if p.x < max_x {
                qs.push(Coordinate::new(p.x + 1, p.y));
            }

            if p.y < max_y {
                qs.push(Coordinate::new(p.x, p.y + 1));
            }

            let edges_p: Vec<Edge> = qs
                .into_iter()
                .map(|q| {
                    Edge::new(
                        *p,
                        q,
                        *nodes
                            .get(&q)
                            .expect("Error getting coordinate cost."),
                    )
                })
                .collect();

            edges.insert(*p, edges_p);
        }

        Cave {
            edges,
            max_x,
            max_y,
        }
    }

    pub fn lowest_total_risk(&self) -> u32 {
        shortest_path(
            &self.edges,
            Coordinate::new(0, 0),
            Coordinate::new(self.max_x, self.max_y),
        )
        .expect("Could not find path to goal.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_total_risk() {
        let cave = Cave::from_grid_file("data/day15/test.txt");
        assert_eq!(cave.lowest_total_risk(), 40);
    }
}
