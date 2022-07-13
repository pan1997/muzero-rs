pub(crate) mod tree;
pub(crate) mod mcts;

enum EdgeLabel<A, B> {
    Action(A),
    Transition(B),
}


pub(crate) trait TreePolicy<N, H, E> {
    fn select_edge<'a>(&self, node: &'a N, hidden_state: &H) -> &'a E;
}