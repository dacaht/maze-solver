mod algorithms;

use std::collections::HashSet;
use std::io::{self, Write};
use rand::Rng;
use algorithms::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub start: Point,
    pub end: Point,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Wall; width]; height];
        
        let mut maze = Maze {
            grid,
            width,
            height,
            start: Point { x: 1, y: 1 },
            end: Point { x: width - 2, y: height - 2 },
        };
        
        maze.generate();
        maze
    }
    
    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        
        let start = Point { x: 1, y: 1 };
        self.grid[start.y][start.x] = Cell::Path;
        visited.insert(start);
        stack.push(start);
        
        while let Some(current) = stack.pop() {
            let neighbors = self.get_unvisited_neighbors(current, &visited);
            
            if !neighbors.is_empty() {
                stack.push(current);
                let next = neighbors[rng.gen_range(0..neighbors.len())];
                
                let wall_x = (current.x + next.x) / 2;
                let wall_y = (current.y + next.y) / 2;
                self.grid[wall_y][wall_x] = Cell::Path;
                self.grid[next.y][next.x] = Cell::Path;
                
                visited.insert(next);
                stack.push(next);
            }
        }
        
        self.grid[self.start.y][self.start.x] = Cell::Path;
        self.grid[self.end.y][self.end.x] = Cell::Path;
    }
    
    fn get_unvisited_neighbors(&self, point: Point, visited: &HashSet<Point>) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
        
        for (dx, dy) in directions.iter() {
            let x = point.x as i32 + dx;
            let y = point.y as i32 + dy;
            
            if x > 0 && x < self.width as i32 - 1 && y > 0 && y < self.height as i32 - 1 {
                let neighbor = Point { x: x as usize, y: y as usize };
                if !visited.contains(&neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        
        neighbors
    }
    
    fn get_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for (dx, dy) in directions.iter() {
            let x = point.x as i32 + dx;
            let y = point.y as i32 + dy;
            
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let neighbor = Point { x: x as usize, y: y as usize };
                if self.grid[neighbor.y][neighbor.x] == Cell::Path {
                    neighbors.push(neighbor);
                }
            }
        }
        
        neighbors
    }
    
    fn display_with_path(&self, path: &[Point]) {
        let path_set: HashSet<Point> = path.iter().cloned().collect();
        
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let point = Point { x, y };
                if point == self.start {
                    print!("SS");
                } else if point == self.end {
                    print!("EE");
                } else if path_set.contains(&point) {
                    print!("··");
                } else {
                    match cell {
                        Cell::Wall => print!("██"),
                        Cell::Path => print!("  "),
                    }
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    pub algorithm: String,
    pub path: Vec<Point>,
    pub path_length: usize,
    pub nodes_explored: usize,
    pub time_taken: u128,
}


fn get_algorithm_choice() -> Vec<fn(&Maze) -> Solution> {
    let available_algorithms = vec![
        ("BFS", bfs_solve as fn(&Maze) -> Solution),
        ("DFS", dfs_solve as fn(&Maze) -> Solution),
        ("A*", astar_solve as fn(&Maze) -> Solution),
        ("Dijkstra", dijkstra_solve as fn(&Maze) -> Solution),
        ("Random", random_solve as fn(&Maze) -> Solution),
        ("Wall-Following", custom_solve as fn(&Maze) -> Solution),
    ];
    
    println!("\nAvailable algorithms:");
    for (i, (name, _)) in available_algorithms.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }
    println!("  a. All algorithms");
    
    let mut input = String::new();
    loop {
        print!("\nSelect algorithms (comma-separated numbers, or 'a' for all): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let selection = input.trim().to_lowercase();
        
        if selection == "a" || selection == "all" {
            return available_algorithms.iter().map(|(_, func)| *func).collect();
        }
        
        let mut selected = Vec::new();
        let mut valid = true;
        
        for part in selection.split(',') {
            let part = part.trim();
            if let Ok(num) = part.parse::<usize>() {
                if num >= 1 && num <= available_algorithms.len() {
                    selected.push(num - 1);
                } else {
                    println!("Invalid number: {}. Please enter numbers between 1 and {}.", num, available_algorithms.len());
                    valid = false;
                    break;
                }
            } else {
                println!("Invalid input: '{}'. Please enter numbers or 'a'.", part);
                valid = false;
                break;
            }
        }
        
        if valid && !selected.is_empty() {
            selected.sort();
            selected.dedup();
            
            let mut result = Vec::new();
            for idx in selected {
                result.push(available_algorithms[idx].1);
            }
            return result;
        }
    }
}

fn get_display_choice() -> bool {
    let mut input = String::new();
    loop {
        print!("\nDisplay maze visualizations? (y/n, default: y): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let choice = input.trim().to_lowercase();
        if choice.is_empty() || choice == "y" || choice == "yes" {
            return true;
        } else if choice == "n" || choice == "no" {
            return false;
        } else {
            println!("Please enter 'y' or 'n'.");
        }
    }
}

fn get_size_from_user() -> (usize, usize) {
    let mut input = String::new();
    
    loop {
        print!("Enter maze width (default: 41, minimum: 5): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let width = if input.trim().is_empty() {
            41
        } else {
            match input.trim().parse::<usize>() {
                Ok(w) if w >= 5 => w | 1,
                Ok(_) => {
                    println!("Width must be at least 5. Please try again.");
                    continue;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number or press Enter for default.");
                    continue;
                }
            }
        };
        
        print!("Enter maze height (default: 21, minimum: 5): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let height = if input.trim().is_empty() {
            21
        } else {
            match input.trim().parse::<usize>() {
                Ok(h) if h >= 5 => h | 1,
                Ok(_) => {
                    println!("Height must be at least 5. Please try again.");
                    continue;
                }
                Err(_) => {
                    println!("Invalid input. Please enter a number or press Enter for default.");
                    continue;
                }
            }
        };
        
        return (width, height);
    }
}

fn main() {
    let (width, height) = get_size_from_user();
    
    let selected_algorithms = get_algorithm_choice();
    
    let show_visualizations = get_display_choice();
    
    println!("\nGenerating random maze ({width}x{height})...");
    let maze = Maze::new(width, height);
    
    println!("Solving with selected algorithms...\n");
    
    let mut solutions: Vec<Solution> = selected_algorithms.iter()
        .map(|algo| algo(&maze))
        .collect();
    
    solutions.iter_mut().for_each(|s| {
        s.path_length = s.path.len();
    });
    
    if show_visualizations {
        for solution in &solutions {
            println!("\n=== {} Solution ===", solution.algorithm);
            maze.display_with_path(&solution.path);
            println!();
        }
    }
    
    println!("\n=== SUMMARY ===");
    println!("{:<16} | {:<12} | {:<15} | {:<12}", 
             "Algorithm", "Path Length", "Nodes Explored", "Time (μs)");
    println!("{}", "-".repeat(70));
    
    for solution in &solutions {
        println!("{:<16} | {:<12} | {:<15} | {:<12}",
                 solution.algorithm,
                 solution.path_length,
                 solution.nodes_explored,
                 solution.time_taken);
    }
    
    let best_path = solutions.iter()
        .min_by_key(|s| s.path_length)
        .unwrap();
    
    let fastest = solutions.iter()
        .min_by_key(|s| s.time_taken)
        .unwrap();
    
    let most_efficient = solutions.iter()
        .min_by_key(|s| s.nodes_explored)
        .unwrap();
    
    println!("\n{}", "-".repeat(70));
    println!("Best Path Length: {} ({} steps)", best_path.algorithm, best_path.path_length);
    println!("Fastest: {} ({} μs)", fastest.algorithm, fastest.time_taken);
    println!("Most Efficient: {} ({} nodes explored)", most_efficient.algorithm, most_efficient.nodes_explored);
    println!("{}", "-".repeat(70));

    println!("\nPress Enter to exit...");
    let mut exit_input = String::new();
    let _ = std::io::stdin().read_line(&mut exit_input);
}