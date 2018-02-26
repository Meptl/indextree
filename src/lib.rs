//! # Arena based tree data structure
//!
//! This arena tree structure is using just a single `Vec` and numerical identifiers (indices in the vector) instead of
//! reference counted pointers like. This means there is no `RefCell` and mutability is handled in a way much more
//! idiomatic to Rust through unique (&mut) access to the arena. The tree can be sent or shared across threads like a `Vec`.
//! This enables general multiprocessing support like parallel tree traversals.
//!
//! # Example usage
//! ```
//! use indextree::Arena;
//!
//! // Create a new arena
//! let arena = &mut Arena::new();
//!
//! // Add some new nodes to the arena
//! let a = arena.new_node(1);
//! let b = arena.new_node(2);
//!
//! // Append b to a
//! a.append(b, arena);
//! assert_eq!(b.ancestors(arena).into_iter().count(), 2);
//! ```
pub use arena::*;
pub use nodeid::*;
pub(crate) use node::*;

mod arena;
mod node;
mod nodeid;
