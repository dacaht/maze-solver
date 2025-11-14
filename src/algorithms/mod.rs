pub mod bfs;
pub mod dfs;
pub mod astar;
pub mod dijkstra;
pub mod random;
pub mod custom;

pub use bfs::bfs_solve;
pub use dfs::dfs_solve;
pub use astar::astar_solve;
pub use dijkstra::dijkstra_solve;
pub use random::random_solve;
pub use custom::custom_solve;

