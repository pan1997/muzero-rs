use rand::seq::SliceRandom;
use crate::lib::search_problem::{HiddenState, Observation, SearchProblem};
use crate::lib::Simulator;

pub(crate) struct RandomSimulator;

impl<P> Simulator<P> for RandomSimulator where P: SearchProblem, P::Player: Copy {
    fn simulate(&self, problem: &P, state: P::HiddenState, horizon: u32, discount: f32) -> Vec<(P::Player, f32)> {
        let mut all_players = problem.get_all_players();
        let mut scores = vec![];
        for player in all_players {
            scores.push((player, 0.0))
        }

        let mut discount_factor = 1.0;
        let mut current_state = state;
        for _ in 0..horizon {
            if current_state.is_terminal() {
                break;
            }

            for (player, score) in scores.iter_mut() {
                *score += discount_factor*problem.get_observation(&current_state, *player).reward()
            }

            let obs = problem.get_observation(&current_state, current_state.current_actor());


            let actions = obs.legal_actions();
            let action = actions.choose(&mut rand::thread_rng()).unwrap();
            current_state = current_state.apply(&action);

            discount_factor *= discount;
        }

        scores
    }
}


pub(crate) fn index_for_player<P: PartialEq>(rewards: &Vec<(P, f32)>, player: &P) -> f32 {
    for (p, r) in rewards.iter() {
        if *player == *p {
            return *r;
        }
    }
    0.0
}

pub(crate) fn reward_for_all_players<P: SearchProblem>(p: &P, hidden_state: &P::HiddenState) -> Vec<(P::Player, f32)> {
    let mut result = vec![];
    for player in p.get_all_players() {
        let obs = p.get_observation(hidden_state, player);
        result.push((player, obs.reward()))
    }
    result
}
