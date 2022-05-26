pub mod a_star;
pub mod bfs;
pub mod dfs;
pub mod greedy;

pub use crate::traits::Solver;
pub use a_star::AStarSolver;
pub use bfs::BFSSolver;
pub use dfs::DFSSolver;
pub use greedy::GreedySolver;
