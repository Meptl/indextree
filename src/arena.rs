use ::std::ops::{Index, IndexMut};
use super::{Node, NodeId};

#[derive(Clone, Debug)]
/// An `Arena` structure containing certain Nodes.
pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
    /// Create a new empty `Arena`
    pub fn new() -> Arena<T> {
        Arena { nodes: Vec::new() }
    }

    /// Create a new node from its associated data.
    pub fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();
        self.nodes.push(Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
            data: data,
        });
        NodeId(next_index)
    }

    /// Returns the number of elements in the arena.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if arena has no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the internal Node structure at NodeId.
    pub(crate) fn get_node(&self, id: NodeId) -> &Node<T> {
        &self.nodes[id.0]
    }

    /// Returns a mutable reference to the internal Node structure at NodeId.
    pub(crate) fn get_node_mut(&mut self, id: NodeId) -> &mut Node<T> {
        &mut self.nodes[id.0]
    }
}

pub trait GetPairMut<T> {
    /// Get mutable references to two distinct nodes. Panics if the two given IDs are the same.
    fn get_pair_mut(&mut self, a: usize, b: usize, same_index_error_message: &'static str) -> (&mut T, &mut T);
}

impl<T> GetPairMut<T> for Vec<T> {
    fn get_pair_mut(&mut self, a: usize, b: usize, same_index_error_message: &'static str) -> (&mut T, &mut T) {
        if a == b {
            panic!(same_index_error_message)
        }
        let (xs, ys) = self.split_at_mut(::std::cmp::max(a, b));
        if a < b {
            (&mut xs[a], &mut ys[0])
        } else {
            (&mut ys[0], &mut xs[b])
        }
    }
}

impl<T> Index<NodeId> for Arena<T> {
    type Output = T;

    fn index(&self, node: NodeId) -> &T {
        &self.nodes[node.0].data
    }
}

impl<T> IndexMut<NodeId> for Arena<T> {
    fn index_mut(&mut self, node: NodeId) -> &mut T {
        &mut self.nodes[node.0].data
    }
}

