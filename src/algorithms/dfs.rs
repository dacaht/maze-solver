use std::collections::{HashMap, HashSet};
use std::time::Instant;
use crate::{Maze, Solution};

pub fn dfs_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    let mut nodes_explored = 0;
    
    stack.push(maze.start);
    visited.insert(maze.start);
    
    while let Some(current) = stack.pop() {
        nodes_explored += 1;
        
        if current == maze.end {
            let mut path = Vec::new();
            let mut node = current;
            while node != maze.start {
                path.push(node);
                node = parent[&node];
            }
            path.push(maze.start);
            path.reverse();
            
            return Solution {
                algorithm: "DFS".to_string(),
                path,
                path_length: 0,
                nodes_explored,
                time_taken: start_time.elapsed().as_micros(),
            };
        }
        
        for neighbor in maze.get_neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                stack.push(neighbor);
            }
        }
    }
    
    Solution {
        algorithm: "DFS".to_string(),
        path: Vec::new(),
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

