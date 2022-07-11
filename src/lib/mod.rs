use std::fmt::{Debug, Display};
use crate::lib::search_problem::SearchProblem;
use crate::lib::search_problem::HiddenState;
use crate::lib::search_problem::Observation;
use rand::seq::SliceRandom;

mod search_problem;
mod tree;
mod tzf8;
mod utils;
mod search;


trait Simulator<P: SearchProblem> {
    fn simulate(&self, problem: &P, state: &P::HiddenState, horizon: u32, discount: f32) -> Vec<(P::Player, f32)>;
}
