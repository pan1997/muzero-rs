
pub(crate) struct Node<L, A> {
    label: L,
    visit_count: u32,
    outgoing_edges: Vec<Edge<L, A>>
}

pub(crate) struct Edge<L, A> {
    parent_node: *Node<L, A>,

    target_node: Option<Node<L, A>>,
    label: A, // Action from the perspective of the root player

    select_count: u32,
    sample_count: u32,
    expected_reward: f32, // Expected reward for the player that played this edge
}

impl<L, A> Node<L, A> where A: PartialEq {
    fn is_expanded(&self) -> bool {
        true
    }

    // we always create edges with all outgoing edges
    // only edges can be dangling
    pub(crate) fn is_terminal(&self) -> bool {
        self.outgoing_edges.is_empty()
    }

    pub(crate) fn get_edge(&self, label: &A) -> &Edge<L, A> {
        for edge in self.outgoing_edges.iter() {
            if *label == edge.label {
                return edge
            }
        }
        panic!("Edge not found")
    }
}


impl <L, A> Edge<L, A> where A: PartialEq {
    pub(crate) fn is_dangling(&self) -> bool {
        self.target_node.is_none()
    }

    pub(crate) fn action(&self) -> &A {
        &self.label
    }

    pub(crate) fn get_target_node(&self) -> &Node<L, A> {
        self.target_node.as_ref().unwrap()
    }
}