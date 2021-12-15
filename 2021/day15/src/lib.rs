use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Location = (usize, usize);
type Grid = Vec<Vec<usize>>;

pub fn read_input(filename: &str) -> Grid {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut nodes = Grid::new();
    for (_y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for (_x, c) in line.chars().enumerate() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        nodes.push(row);
    }

    nodes
}

pub fn grid_to_graph(grid: &Grid) -> Vec<Vec<Edge>> {
    let mut adj_graph = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _cost) in row.iter().enumerate() {
            let mut node_adjacency = Vec::new();
            let adjacency = get_adjacency(&grid, &(x, y));
            for (neighbor_idx, neighbor_cost) in adjacency {
                node_adjacency.push(Edge {
                    node: neighbor_idx,
                    cost: neighbor_cost,
                });
            }

            adj_graph.push(node_adjacency);
        }
    }
    adj_graph
}

pub fn grid_to_ubergrid(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();
    let mut uber_grid = Vec::new();

    // Expand out in the x direction 5 times

    for row in grid {
        let mut uber_row = Vec::new();
        for i in 0..5 {
            for c in row {
                let new_c = if c + i <= 9 { c + i } else { c + i - 9 };
                uber_row.push(new_c);
            }
        }

        new_grid.push(uber_row);
    }

    // Expand out the expanded rows in the y direction 5 times
    for i in 0..5 {
        for row in &new_grid {
            let mut uber_row = Vec::new();

            for c in row {
                let new_c = if c + i <= 9 { c + i } else { c + i - 9 };
                uber_row.push(new_c);
            }
            uber_grid.push(uber_row);
        }
    }

    uber_grid
}

fn get_adjacency(grid: &Grid, location: &Location) -> Vec<(usize, usize)> {
    // report out a list of the row-oriented indices and costs
    let mut neighbors = Vec::new();
    let max_x = grid[0].len();
    let max_y = grid.len();

    let (x, y) = *location;

    if x + 1 < max_x {
        let key = (x + 1, y);
        neighbors.push((key.1 * max_x + key.0, grid[key.1][key.0]));
    }

    if x > 0 {
        let key = (x - 1, y);
        neighbors.push((key.1 * max_x + key.0, grid[key.1][key.0]));
    }

    if y + 1 < max_y {
        let key = (x, y + 1);
        neighbors.push((key.1 * max_x + key.0, grid[key.1][key.0]));
    }

    if y > 0 {
        let key = (x, y - 1);
        neighbors.push((key.1 * max_x + key.0, grid[key.1][key.0]));
    }

    neighbors
}

// #![allow(unused)]
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
pub struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example1() {
        let grid = read_input("example.txt");
        let adj_graph = grid_to_graph(&grid);

        let risk = shortest_path(&adj_graph, 0, adj_graph.len() - 1);
        assert_eq!(risk, Some(40));
    }

    #[test]
    fn part1() {
        let grid = read_input("input.txt");
        let adj_graph = grid_to_graph(&grid);

        let risk = shortest_path(&adj_graph, 0, adj_graph.len() - 1);
        assert_eq!(risk, Some(621));
    }

    #[test]
    fn example2() {
        let grid = read_input("example.txt");
        let uber_grid = grid_to_ubergrid(&grid);
        let adj_graph = grid_to_graph(&uber_grid);

        let risk = shortest_path(&adj_graph, 0, adj_graph.len() - 1);
        assert_eq!(risk, Some(315));
    }

    #[test]
    fn part2() {
        let grid = read_input("input.txt");
        let uber_grid = grid_to_ubergrid(&grid);
        let adj_graph = grid_to_graph(&uber_grid);

        let risk = shortest_path(&adj_graph, 0, adj_graph.len() - 1);
        assert_eq!(risk, Some(2904));
    }
}
