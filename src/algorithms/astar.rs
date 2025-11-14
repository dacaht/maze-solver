use std::collections::HashMap;
use std::time::Instant;
use crate::{Maze, Point, Solution};

pub fn astar_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut open_set = Vec::new();
    let mut came_from = HashMap::new();
    let mut g_score: HashMap<Point, usize> = HashMap::new();
    let mut f_score: HashMap<Point, usize> = HashMap::new();
    let mut nodes_explored = 0;
    
    let heuristic = |p: Point| -> usize {
        ((p.x as i32 - maze.end.x as i32).abs() + (p.y as i32 - maze.end.y as i32).abs()) as usize
    };
    
    g_score.insert(maze.start, 0);
    f_score.insert(maze.start, heuristic(maze.start));
    open_set.push((f_score[&maze.start], maze.start));
    
    while !open_set.is_empty() {
        open_set.sort_by(|a, b| b.0.cmp(&a.0));
        let (_, current) = open_set.pop().unwrap();
        nodes_explored += 1;
        
        if current == maze.end {
            let mut path = Vec::new();
            let mut node = current;
            while node != maze.start {
                path.push(node);
                node = came_from[&node];
            }
            path.push(maze.start);
            path.reverse();
            
            return Solution {
                algorithm: "A*".to_string(),
                path,
                path_length: 0,
                nodes_explored,
                time_taken: start_time.elapsed().as_micros(),
            };
        }
        
        for neighbor in maze.get_neighbors(current) {
            let tentative_g = g_score.get(&current).unwrap_or(&usize::MAX) + 1;
            
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor);
                f_score.insert(neighbor, f);
                
                if !open_set.iter().any(|(_, p)| *p == neighbor) {
                    open_set.push((f, neighbor));
                }
            }
        }
    }
    
    Solution {
        algorithm: "A*".to_string(),
        path: Vec::new(),
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

