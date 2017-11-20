use std::ops::{Deref, Index, IndexMut};
use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Comp<T: Hash>(T);

trait Renderable {
    fn hash_inner_state(&self) -> u64;

    fn render_template(&self) -> String;
}

impl<T: Hash> Renderable for Comp<T> {
    fn hash_inner_state(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn render_template(&self) -> String {
        "".into()
    }
}

impl<'a, 'b, T: Hash> Index<&'a mut DomChanges<'b>> for Comp<T> {
    type Output = T;

    // do nothing - this is just necessary because
    // indexMut is required to have the same output
    fn index<'c>(&'c self, _: &'a mut DomChanges) -> &'c T {
        &self.0
    }
}

impl<'a, 'b, T: Hash> IndexMut<&'a mut DomChanges<'b>> for Comp<T> {
    fn index_mut<'c>(&'c mut self, changes: &'a mut DomChanges) -> &'c mut T {
        // changes.components.push(&self);
        &mut self.0
    }
}

impl<T: Hash> Deref for Comp<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct DomChanges<'a> {
    changes: Cell<u32>,
    components: Vec<&'a Renderable>,
    // components: Vec<Box<Renderable>>,
}

// pub struct DomChanges1<'a> {
//     changes: Cell<u32>,
// }

impl<'a> DomChanges<'a> {
    pub fn new() -> DomChanges<'a> {
        DomChanges {
            changes: Cell::new(0),
            components: Vec::new()
        }
    }

    pub fn add<T: Hash>(&self, state: T) -> Comp<T> {
        self.changes.set(self.changes.get() + 1);
        Comp(state)
    }

    pub fn set<T: Hash>(self, state: T, comp: &mut Comp<T>) -> Self {
        self.changes.set(self.changes.get() + 1);
        comp.0 = state;
        self
    }
}

impl<'a> Drop for DomChanges<'a> {
    fn drop(&mut self) {
        println!(
            "DomChanges Dropped! Now perform {} operations!",
            self.changes.get()
        );
    }
}
