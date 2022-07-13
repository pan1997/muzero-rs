use std::cell::{RefCell, RefMut};
use std::ptr::null;

pub(crate) struct Node<L, A> {
    pub(crate) label: L,
    incoming_edge: *const Edge<L, A>,
    outgoing_edges: Vec<Edge<L, A>>,
    node_statistics: RefCell<NodeStatistics>,
}

pub(crate) struct Edge<L, A> {
    parent_node: *const Node<L, A>,

    target_node: Option<Node<L, A>>,
    pub(crate) label: A, // Action from the perspective of the root player
    // todo: remove option if 0.0 works
    action_reward: Option<f32>,
}

pub(crate) struct NodeStatistics {
    select_count: u32,
    sample_count: u32,
    expected_reward: f32, // Expected reward for the player that played this edge
}

impl<L, A> Node<L, A> where A: PartialEq {

    pub(crate) fn new(label: L, actions: Vec<A>, incoming_edge: *const Edge<L, A>) -> Self {
        let mut result =
            Node {
                label,
                incoming_edge,
                outgoing_edges: vec![],
                node_statistics: RefCell::new(NodeStatistics {
                    select_count: 0,
                    sample_count: 0,
                    expected_reward: 0.0
                })
            };
        for action in actions {
            result.outgoing_edges.push(Edge {
                parent_node: &result,
                target_node: None,
                label: action,
                action_reward: None
            })
        }
        result
    }


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
        // todo create new, as different determinisations can have different legal actions
        panic!("Edge not found")
    }

    pub(crate) fn edges(&self) -> &Vec<Edge<L, A>> {
        &self.outgoing_edges
    }

    pub(crate) fn get_statistics_lock<'a, 'b: 'a>(&'b self) -> RefMut<'a, NodeStatistics> {
        self.node_statistics.borrow_mut()
    }

    pub(crate) fn get_incoming_edge(&self) -> Option<&Edge<L, A>> {
        unsafe {
            self.incoming_edge.as_ref()
        }
    }
}


impl <L, A> Edge<L, A> where A: PartialEq {
    pub(crate) fn is_dangling(&self) -> bool {
        self.target_node.is_none()
    }

    pub(crate) fn get_target_node(&self) -> &Node<L, A> {
        self.target_node.as_ref().unwrap()
    }

    pub(crate) fn get_incoming_node(&self) -> &Node<L, A> {
        unsafe {
            &*self.parent_node
        }
    }

    pub(crate) fn create_child(&self, label: L, actions: Vec<A>) {
        assert!(self.is_dangling(), "Cannot rewrite.");
        let mut target_node =  Node::new(label, actions, self);

        unsafe {
            let target_node_ptr: *const Option<Node<L, A>> = &self.target_node;
            let target_node_ptr_mut = target_node_ptr as *mut Option<Node<L, A>>;
            let mut target_node_mut = &mut *target_node_ptr_mut;
            target_node_mut.replace(target_node);
        }
    }

    pub(crate) fn get_action_reward(&self) -> Option<f32> {
        self.action_reward
    }

    pub(crate) fn set_action_reward(&self, reward: f32) {
        unsafe {
            let reward_ptr: *const Option<f32> = &self.action_reward;
            let reward_ptr_mut = reward_ptr as *mut Option<f32>;
            let mut reward_mut = &mut *reward_ptr_mut;
            reward_mut.replace(reward);
        }
    }
}


impl NodeStatistics {
    fn new() -> Self {
        NodeStatistics {
            select_count: 0,
            sample_count: 0,
            expected_reward: 0.0
        }
    }

    fn select_count(&self) -> u32 {
        self.select_count
    }
    pub(crate) fn increment_select_count(&mut self) {
        self.select_count += 1
    }

    pub(crate) fn add_sample(&mut self, s: f32, weight: u32) {
        self.expected_reward += (weight as f32) * (s - self.expected_reward) / (weight + self.sample_count) as f32;
        self.sample_count += weight
    }

    pub(crate)fn expected_sample(&self) -> f32 {
        self.expected_reward
    }
}