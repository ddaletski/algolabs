pub mod a_star;
pub mod greedy;

pub use crate::traits::Solver;
pub use a_star::AStarSolver;
pub use greedy::GreedySolver;