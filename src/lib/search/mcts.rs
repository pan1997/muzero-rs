use std::ptr::null;
use crate::lib::search::{EdgeLabel, TreePolicy};
use crate::lib::search_problem::{HiddenState, Observation, SearchProblem};
use crate::lib::search::tree::{Node, Edge};
use crate::lib::Simulator;
use crate::lib::utils::{index_for_player, reward_for_all_players};

pub(crate) struct MctsConfig<T, P: SearchProblem, S> {
    pub(crate) search_problem: P,
    pub(crate) players: Vec<P::Player>,
    pub(crate) tree_policy: T,
    pub(crate) simulator: S,
    pub(crate) discount: f32,
    pub(crate) horizon: u32,
}


impl<T, P, S> MctsConfig<T, P, S>
    where
        P: SearchProblem,
    // Not storing state in tree nodes yet, only current player
        T: TreePolicy<Node<P::Player, P::Action>, P::HiddenState, Edge<P::Player, P::Action>>,
        S: Simulator<P> {

    fn select<'a>(
        &self,
        mut hidden_state: P::HiddenState,
        mut nodes: Vec<&'a Node<P::Player, P::Action>>,
    ) -> (
        P::HiddenState, // The resulting state
        Vec<&'a Node<P::Player, P::Action>>
    ) {
        'outer: for _ in 0..self.horizon {

            for node in nodes.iter() {
                node.get_statistics_lock().increment_select_count();
            }

            // if any of the trees reaches a terminal node, all trees are in terminal
            // this is wrong because the players might have different set of legal moves,
            // and this is based only on move count
            //if nodes[0].is_terminal() {
            //    break;
            //}
            if hidden_state.is_terminal() {
                break;
            }

            let current_player = hidden_state.current_actor();
            let current_node = nodes[self.player_index(current_player)];
            let selected_edge =  self.tree_policy.select_edge(current_node, &hidden_state);

            let selected_action = &selected_edge.label;

            let mut edges = vec![];
            for (player, node) in self.search_problem.get_all_players().iter().zip(nodes.iter()) {
                edges.push(
                    node.get_edge(
                        &self
                            .search_problem
                            .get_visible_action(
                                &hidden_state,
                                selected_action,
                                player
                            )
                    )
                );
            }

            hidden_state = hidden_state.apply(selected_action);

            nodes = vec![];
            let trajectory_terminal = selected_edge.is_dangling();

            for edge in edges.iter() {
                // only when the edge on the player to move's tree is dangling,
                // do we need to terminate the trajectory

                //if edge.is_dangling() {
                //    return (hidden_state, edges)
                //}

                if edge.is_dangling() {
                    let obs = self.search_problem.get_observation(&hidden_state, hidden_state.current_actor());
                    edge.set_action_reward(obs.reward());
                    edge.create_child(hidden_state.current_actor(), obs.legal_actions());
                }
                nodes.push(edge.get_target_node())
            }

            if trajectory_terminal {
                return (hidden_state, nodes)
            }
        }
        (hidden_state, vec![])
    }

    #[deprecated]
    fn expand<'a>(
        &self,
        hidden_state: &P::HiddenState,
        edges: &Vec<&'a Edge<P::Player, P::Action>>
    ) -> Vec<&'a Node<P::Player, P::Action>> {
        let mut nodes = vec![];
        for (edge, player) in edges.iter().zip(self.players.iter()) {
            if edge.is_dangling() {
                let observation = self.search_problem.get_observation(hidden_state, *player);
                edge.create_child(hidden_state.current_actor(), observation.legal_actions());
                nodes.push(edge.get_target_node());
            }
        }
        nodes
    }

    fn propagate(&self, mut nodes: Vec<&Node<P::Player, P::Action>>, mut values: Vec<(P::Player, f32)>) {
        while !nodes.is_empty() {
            let mut next = vec![];
            for node in nodes {
                node.get_statistics_lock().add_sample(index_for_player(&values, &node.label), 1);
                if let Some(e) = node.get_incoming_edge() {
                    next.push(e.get_incoming_node());
                }
            }
            nodes = next;
        }
    }

    fn player_index(&self, player: P::Player) -> usize {
        for (ix, p) in self.players.iter().enumerate() {
            if player == *p {
                return ix
            }
        }
        usize::MAX
    }
}


pub(crate) fn once<T, P, S>(config: &MctsConfig<T, P, S>, hidden_state: P::HiddenState, nodes: Vec<&Node<P::Player, P::Action>>)
    where
        P: SearchProblem,
        T: TreePolicy<Node<P::Player, P::Action>, P::HiddenState, Edge<P::Player, P::Action>>,
        S: Simulator<P> {
    let (new_state, nodes) = config.select(hidden_state, nodes);
    // this handles already created terminal nodes correctly
    let mut value = vec![];
    if new_state.is_terminal() {
        // if one is terminal, then all are terminal
        value = reward_for_all_players(&config.search_problem, &new_state);
    } else {
        value = config.simulator.simulate(
            &config.search_problem,
            new_state,
            config.horizon,
            config.discount
        );
    }
    config.propagate(nodes, value);
}

pub(crate) fn initialise<P: SearchProblem>(p: &P, state: &P::HiddenState) -> Vec<Node<P::Player, P::Action>> {
    let mut result = vec![];
    for player in p.get_all_players() {
        let obs = p.get_observation(state, player);
        result.push(Node::new(state.current_actor(), obs.legal_actions(), null()));
    }
    result
}
