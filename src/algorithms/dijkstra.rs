use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::{Maze, Point, Solution};

pub fn dijkstra_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut dist: HashMap<Point, usize> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut nodes_explored = 0;
    
    for y in 0..maze.height {
        for x in 0..maze.width {
            if maze.grid[y][x] == crate::Cell::Path {
                let point = Point { x, y };
                dist.insert(point, usize::MAX);
                unvisited.insert(point);
            }
        }
    }
    
    dist.insert(maze.start, 0);
    
    while !unvisited.is_empty() {
        let current = *unvisited.iter()
            .min_by_key(|p| dist.get(p).unwrap_or(&usize::MAX))
            .unwrap();
        
        if dist.get(&current).unwrap_or(&usize::MAX) == &usize::MAX {
            break;
        }
        
        unvisited.remove(&current);
        nodes_explored += 1;
        
        if current == maze.end {
            let mut path = Vec::new();
            let mut node = current;
            while node != maze.start {
                path.push(node);
                node = prev[&node];
            }
            path.push(maze.start);
            path.reverse();
            
            return Solution {
                algorithm: "Dijkstra".to_string(),
                path,
                path_length: 0,
                nodes_explored,
                time_taken: start_time.elapsed().as_micros(),
            };
        }
        
        for neighbor in maze.get_neighbors(current) {
            if unvisited.contains(&neighbor) {
                let alt = dist.get(&current).unwrap() + 1;
                if alt < *dist.get(&neighbor).unwrap_or(&usize::MAX) {
                    dist.insert(neighbor, alt);
                    prev.insert(neighbor, current);
                }
            }
        }
    }
    
    Solution {
        algorithm: "Dijkstra".to_string(),
        path: Vec::new(),
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

