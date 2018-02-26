extern crate indextree;
use indextree::Arena;

use std::cell::Cell;

#[test]
fn data_lifetimes() {
    struct DropTracker<'a>(&'a Cell<u32>);
    impl<'a> Drop for DropTracker<'a> {
        fn drop(&mut self) {
            self.0.set(&self.0.get() + 1);
        }
    }

    let drop_counter = Cell::new(0);
    {
        let mut new_counter = 0;
        let arena = &mut Arena::new();
        macro_rules! new {
            () => {{
                new_counter += 1;
                arena.new_node((new_counter, DropTracker(&drop_counter)))
            }}
        };

        let a = new!();  // 1
        a.append(new!(), arena);  // 2
        a.append(new!(), arena);  // 3
        a.prepend(new!(), arena);  // 4
        let b = new!();  // 5
        b.append(a, arena);
        a.insert_before(new!(), arena);  // 6
        a.insert_before(new!(), arena);  // 7
        a.insert_after(new!(), arena);  // 8
        a.insert_after(new!(), arena);  // 9
        {
            let c = new!();  // 10
            b.append(c, arena);
            c.detach(arena);
        }
        assert_eq!(drop_counter.get(), 0);
    }

    assert_eq!(drop_counter.get(), 10);
}

#[test]
fn ancestor() {
    let arena = &mut Arena::new();
    let a = arena.new_node(1);
    let b = arena.new_node(2);
    let c = arena.new_node(3);
    a.append(b, arena);
    b.append(c, arena);

    let mut ancestors = c.ancestors(arena);
    assert_eq!(ancestors.next().unwrap(), b);
    assert_eq!(ancestors.next().unwrap(), a);
    assert_eq!(ancestors.next(), None);
}

#[test]
fn prepend() {
    let arena = &mut Arena::new();
    let a = arena.new_node(1);
    let b = arena.new_node(2);
    a.prepend(b, arena);
}

#[test]
fn detach() {
    let arena = &mut Arena::new();
    let a = arena.new_node(1);
    let b = arena.new_node(1);
    a.append(b, arena);
    assert_eq!(a.children(arena).into_iter().count(), 1);
    b.detach(arena);
    assert_eq!(a.children(arena).into_iter().count(), 0);
}
