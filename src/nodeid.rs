use ::std::mem;
use super::{Arena, GetPairMut, Node};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
/// A node identifier within a particular `Arena`.
pub struct NodeId(pub usize);

impl NodeId {
    /// Return an iterator over references to the node's ancestors.
    pub fn ancestors<T>(self, arena: &Arena<T>) -> Ancestors<T> {
        Ancestors {
            arena: arena,
            node: arena.get_node(self).parent,
        }
    }

    /// Return an iterator over references to the node's previous siblings.
    pub fn preceding_siblings<T>(self, arena: &Arena<T>) -> PrecedingSiblings<T> {
        PrecedingSiblings {
            arena: arena,
            node: arena.get_node(self).previous_sibling,
        }
    }

    /// Return an iterator over references to the node's following siblings.
    pub fn following_siblings<T>(self, arena: &Arena<T>) -> FollowingSiblings<T> {
        FollowingSiblings {
            arena: arena,
            node: arena.get_node(self).next_sibling,
        }
    }

    /// Return an iterator over references to the node's children.
    pub fn children<T>(self, arena: &Arena<T>) -> Children<T> {
        Children {
            arena: arena,
            node: arena.get_node(self).first_child,
        }
    }

    /// Detach a node from its parent.
    pub fn detach<T>(self, arena: &mut Arena<T>) {
        let (parent, previous_sibling, next_sibling) = {
            let node = &mut arena.get_node_mut(self);
            (node.parent.take(), node.previous_sibling.take(), node.next_sibling.take())
        };

        if let Some(next_sibling) = next_sibling {
            arena.get_node_mut(next_sibling).previous_sibling = previous_sibling;
        } else if let Some(parent) = parent {
            arena.get_node_mut(parent).last_child = previous_sibling;
        }

        if let Some(previous_sibling) = previous_sibling {
            arena.get_node_mut(previous_sibling).next_sibling = next_sibling;
        } else if let Some(parent) = parent {
            arena.get_node_mut(parent).first_child = next_sibling;
        }
    }

    /// Append a new child to this node, after existing children.
    pub fn append<T>(self, new_child: NodeId, arena: &mut Arena<T>) {
        new_child.detach(arena);
        let last_child_opt;
        {
            let (self_borrow, new_child_borrow) = arena.nodes.get_pair_mut(self.0,
                                                                           new_child.0,
                                                                           "Can not append a node to itself");
            new_child_borrow.parent = Some(self);
            last_child_opt = mem::replace(&mut self_borrow.last_child, Some(new_child));
            if let Some(last_child) = last_child_opt {
                new_child_borrow.previous_sibling = Some(last_child);
            } else {
                debug_assert!(self_borrow.first_child.is_none());
                self_borrow.first_child = Some(new_child);
            }
        }
        if let Some(last_child) = last_child_opt {
            debug_assert!(arena.get_node(last_child).next_sibling.is_none());
            arena.get_node_mut(last_child).next_sibling = Some(new_child);
        }
    }

    /// Prepend a new child to this node, before existing children.
    pub fn prepend<T>(self, new_child: NodeId, arena: &mut Arena<T>) {
        new_child.detach(arena);
        let first_child_opt;
        {
            let (self_borrow, new_child_borrow) = arena.nodes.get_pair_mut(self.0,
                                                                           new_child.0,
                                                                           "Can not prepend a node to itself");
            new_child_borrow.parent = Some(self);
            first_child_opt = mem::replace(&mut self_borrow.first_child, Some(new_child));
            if let Some(first_child) = first_child_opt {
                new_child_borrow.next_sibling = Some(first_child);
            } else {
                debug_assert!(&self_borrow.first_child.is_none());
                self_borrow.last_child = Some(new_child);
            }
        }
        if let Some(first_child) = first_child_opt {
            debug_assert!(arena.get_node(first_child).previous_sibling.is_none());
            arena.get_node_mut(first_child).previous_sibling = Some(new_child);
        }
    }

    /// Insert a new sibling after this node.
    pub fn insert_after<T>(self, new_sibling: NodeId, arena: &mut Arena<T>) {
        new_sibling.detach(arena);
        let next_sibling_opt;
        let parent_opt;
        {
            let (self_borrow, new_sibling_borrow) = arena.nodes.get_pair_mut(self.0,
                                                                             new_sibling.0,
                                                                             "Can not insert a node after itself");
            parent_opt = self_borrow.parent;
            new_sibling_borrow.parent = parent_opt;
            new_sibling_borrow.previous_sibling = Some(self);
            next_sibling_opt = mem::replace(&mut self_borrow.next_sibling, Some(new_sibling));
            if let Some(next_sibling) = next_sibling_opt {
                new_sibling_borrow.next_sibling = Some(next_sibling);
            }
        }
        if let Some(next_sibling) = next_sibling_opt {
            debug_assert!(arena.get_node(next_sibling).previous_sibling.unwrap() == self);
            arena.get_node_mut(next_sibling).previous_sibling = Some(new_sibling);
        } else if let Some(parent) = parent_opt {
            debug_assert!(arena.get_node(parent).last_child.unwrap() == self);
            arena.get_node_mut(parent).last_child = Some(new_sibling);
        }
    }

    /// Insert a new sibling before this node.
    pub fn insert_before<T>(self, new_sibling: NodeId, arena: &mut Arena<T>) {
        new_sibling.detach(arena);
        let previous_sibling_opt;
        let parent_opt;
        {
            let (self_borrow, new_sibling_borrow) = arena.nodes.get_pair_mut(self.0,
                                                                             new_sibling.0,
                                                                             "Can not insert a node before itself");
            parent_opt = self_borrow.parent;
            new_sibling_borrow.parent = parent_opt;
            new_sibling_borrow.next_sibling = Some(self);
            previous_sibling_opt = mem::replace(&mut self_borrow.previous_sibling, Some(new_sibling));
            if let Some(previous_sibling) = previous_sibling_opt {
                new_sibling_borrow.previous_sibling = Some(previous_sibling);
            }
        }
        if let Some(previous_sibling) = previous_sibling_opt {
            debug_assert!(arena.get_node(previous_sibling).next_sibling.unwrap() == self);
            arena.get_node_mut(previous_sibling).next_sibling = Some(new_sibling);
        } else if let Some(parent) = parent_opt {
            debug_assert!(arena.get_node(parent).first_child.unwrap() == self);
            arena.get_node_mut(parent).first_child = Some(new_sibling);
        }
    }
}

macro_rules! impl_node_iterator {
    ($name: ident, $next: expr) => {
        impl<'a, T> Iterator for $name<'a, T> {
            type Item = NodeId;

            fn next(&mut self) -> Option<NodeId> {
                match self.node.take() {
                    Some(node) => {
                        self.node = $next(&self.arena.get_node(node));
                        Some(node)
                    }
                    None => None
                }
            }
        }
    }
}

/// An iterator to the ancestors a given node.
pub struct Ancestors<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(Ancestors, |node: &Node<T>| node.parent);

/// An iterator to the siblings before a given node.
pub struct PrecedingSiblings<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(PrecedingSiblings, |node: &Node<T>| node.previous_sibling);

/// An iterator to the siblings after a given node.
pub struct FollowingSiblings<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(FollowingSiblings, |node: &Node<T>| node.next_sibling);

/// An iterator to the children of a given node.
pub struct Children<'a, T: 'a> {
    arena: &'a Arena<T>,
    node: Option<NodeId>,
}
impl_node_iterator!(Children, |node: &Node<T>| node.next_sibling);
