use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use crate::{Maze, Solution};

pub fn bfs_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    let mut nodes_explored = 0;
    
    queue.push_back(maze.start);
    visited.insert(maze.start);
    
    while let Some(current) = queue.pop_front() {
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
                algorithm: "BFS".to_string(),
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
                queue.push_back(neighbor);
            }
        }
    }
    
    Solution {
        algorithm: "BFS".to_string(),
        path: Vec::new(),
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

