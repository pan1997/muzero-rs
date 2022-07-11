use rand::seq::SliceRandom;
use crate::lib::search_problem::{HiddenState, Observation, SearchProblem};
use crate::lib::Simulator;

struct RandomSimulator;

impl<P> Simulator<P> for RandomSimulator where P: SearchProblem, P::HiddenState: Clone, P::Player: Copy {
    fn simulate(&self, problem: &P, state: &P::HiddenState, horizon: u32, discount: f32) -> Vec<(P::Player, f32)> {
        let mut all_players = problem.get_all_players();
        let mut scores = vec![];
        for player in all_players {
            scores.push((player, 0.0))
        }

        let mut discount_factor = 1.0;
        let mut current_state = state.clone();
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

