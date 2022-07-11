mod tree;
mod mcts;

enum EdgeLabel<A, B> {
    Action(A),
    Transition(B),
}


trait TreePolicy<N, P, E> {
    fn select_edge(&self, node: &N, current_player: P) -> &E;
}