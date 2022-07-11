


struct Node<Player> {
    visit_count: u32,
    current_actor: Player,

    outgoing_edges: Vec<Edge<Player>>,
}

struct Edge<Player> {
    visit_count: u32,
    prior_prob: f32,
    // expected reward for the player on incoming node
    expected_reward: f32,

    target_node: Option<Node<Player>>
}

impl<Player: Copy> Node<Player> {
    
}