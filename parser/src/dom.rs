use std::ops::{Deref, DerefMut};
use experiment2::DomChanges;

pub struct Comp<T>(T);

impl<T> Deref for Comp<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Comp<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub struct DomChanges {
    changes: u32,
}

impl DomChanges {
    fn new() -> DomChanges {
        DomChanges { changes: 0 }
    }

    fn add<T>(&mut self, state: T) -> Comp<T> {
        self.changes += 1;
        Comp(state)
    }

    fn set<T>(&mut self, comp: &mut Comp<T>, state: T) -> &mut Self {
        self.changes += 1;
        comp.0 = state;
        self
    }
}

impl Drop for DomChanges {
    fn drop(&mut self) {
        println!("OpCursor Dropped! Now perform {} operations!", self.changes);
    }
}
