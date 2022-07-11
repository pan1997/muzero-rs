use crate::lib::search::{EdgeLabel, TreePolicy};
use crate::lib::search_problem::{HiddenState, SearchProblem};
use crate::lib::search::tree::{Node, Edge};

struct MctsConfig<T, P: SearchProblem> {
    search_problem: P,
    players: Vec<P::Player>,
    tree_policy: T,
    discount: f32,
    horizon: u32,
}


impl<P, T> MctsConfig<T, P>
    where
        P: SearchProblem,
        P::Action: PartialEq,
        P::Player: PartialEq,
        T: TreePolicy<Node<P::Observation, P::Action>, P::Player, Edge<P::Observation, P::Action>> {
    fn select<'a>(
        &self,
        hidden_state: &P::HiddenState,
        mut nodes: Vec<&'a Node<P::Observation, P::Action>>,
    ) -> Vec<&'a Edge<P::Observation, P::Action>> {
        'outer: for _ in 0..self.horizon {
            // if any of the trees reaches a terminal node, all trees are in terminal
            if nodes[0].is_terminal() {
                break;
            }
            // todo: is this needed, or is the above sufficient?
            if hidden_state.is_terminal() {
                break;
            }

            let current_player = hidden_state.current_actor();
            let current_node = nodes[self.player_index(current_player)];

            let selected_action = self.tree_policy.select_edge(current_node, current_player).action();

            let mut edges = vec![];
            for (player, node) in self.search_problem.get_all_players().iter().zip(nodes.iter()) {
                edges.push(
                    node.get_edge(
                        &self
                            .search_problem
                            .get_visible_action(
                                hidden_state,
                                selected_action,
                                player
                            )
                    )
                );
            }
            nodes = vec![];
            for edge in edges.iter() {
                if edge.is_dangling() {
                    return edges
                }
                nodes.push(edge.get_target_node())
            }
        }
        vec![]
    }

    fn expand(&self) {}

    fn propagate(&self) {}

    fn player_index(&self, player: P::Player) -> usize {
        for (ix, p) in self.players.iter().enumerate() {
            if player == *p {
                return ix
            }
        }
        usize::MAX
    }
}