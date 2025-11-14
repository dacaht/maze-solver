use std::collections::HashSet;
use std::time::Instant;
use rand::Rng;
use crate::{Maze, Point, Solution};

pub fn random_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut rng = rand::thread_rng();
    let mut current = maze.start;
    let mut path = vec![maze.start];
    let mut visited = HashSet::new();
    visited.insert(maze.start);
    let mut nodes_explored = 1;
    let max_iterations = maze.width * maze.height * 10;
    let mut iterations = 0;
    
    while current != maze.end && iterations < max_iterations {
        iterations += 1;
        let neighbors: Vec<Point> = maze.get_neighbors(current)
            .into_iter()
            .filter(|p| !visited.contains(p))
            .collect();
        
        if neighbors.is_empty() {
            if path.len() > 1 {
                path.pop();
                current = *path.last().unwrap();
            } else {
                break;
            }
        } else {
            let next = neighbors[rng.gen_range(0..neighbors.len())];
            visited.insert(next);
            path.push(next);
            current = next;
            nodes_explored += 1;
        }
    }
    
    Solution {
        algorithm: "Random".to_string(),
        path,
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

