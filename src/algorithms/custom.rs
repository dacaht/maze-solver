use std::collections::HashSet;
use std::time::Instant;
use crate::{Maze, Point, Solution};

pub fn custom_solve(maze: &Maze) -> Solution {
    let start_time = Instant::now();
    let mut path = vec![maze.start];
    let mut visited = HashSet::new();
    visited.insert(maze.start);
    let mut nodes_explored = 1;
    
    let mut direction = 1;
    let mut current = maze.start;
    
    let max_iterations = maze.width * maze.height * 4;
    let mut iterations = 0;
    
    while current != maze.end && iterations < max_iterations {
        iterations += 1;
        
        let right_dir = (direction + 1) % 4;
        let front_dir = direction;
        let left_dir = (direction + 3) % 4;
        
        let right_neighbor = get_neighbor_in_direction(current, right_dir);
        let has_right_wall = right_neighbor.is_none() || 
            !is_valid_path(maze, right_neighbor.unwrap());
        
        let front_neighbor = get_neighbor_in_direction(current, front_dir);
        let can_go_front = front_neighbor.is_some() && 
            is_valid_path(maze, front_neighbor.unwrap());
        
        if !has_right_wall {
            direction = right_dir;
            current = right_neighbor.unwrap();
            if !visited.contains(&current) {
                visited.insert(current);
                nodes_explored += 1;
            }
            path.push(current);
        } else if can_go_front {
            current = front_neighbor.unwrap();
            if !visited.contains(&current) {
                visited.insert(current);
                nodes_explored += 1;
            }
            path.push(current);
        } else {
            direction = left_dir;
        }
    }
    
    Solution {
        algorithm: "Wall-Following".to_string(),
        path,
        path_length: 0,
        nodes_explored,
        time_taken: start_time.elapsed().as_micros(),
    }
}

fn get_neighbor_in_direction(point: Point, direction: usize) -> Option<Point> {
    match direction {
        0 => Some(Point { x: point.x, y: point.y.wrapping_sub(1) }),
        1 => Some(Point { x: point.x + 1, y: point.y }),
        2 => Some(Point { x: point.x, y: point.y + 1 }),
        3 => Some(Point { x: point.x.wrapping_sub(1), y: point.y }),
        _ => None,
    }
}

fn is_valid_path(maze: &Maze, point: Point) -> bool {
    if point.x >= maze.width || point.y >= maze.height {
        return false;
    }
    maze.grid[point.y][point.x] == crate::Cell::Path
}
