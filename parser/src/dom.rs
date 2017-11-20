use std::ops::{Deref, DerefMut};
use std::cell::Cell;

pub struct Comp<T>(T);

impl<T> Comp<T> {
    // pub fn set() -> DomChanges {
    //     DomChanges::new().set(state, self)
    // }
}

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


impl<T> Drop for Comp<T> {
    fn drop(&mut self) {
        println!("dropping component");
    }
}

pub struct DomChanges {
    changes: Cell<u32>,
}

impl DomChanges {
    pub fn new() -> DomChanges {
        DomChanges {
            changes: Cell::new(0),
        }
    }

    pub fn add<T>(&self, state: T) -> Comp<T> {
        self.changes.set(self.changes.get() + 1);
        Comp(state)
    }

    pub fn set<T>(self, state: T, comp: &mut Comp<T>) -> Self {
        self.changes.set(self.changes.get() + 1);
        comp.0 = state;
        self
    }

    pub fn set1<T>(state: T, comp: &mut Comp<T>) {

    }
}

impl Drop for DomChanges {
    fn drop(&mut self) {
        println!(
            "DomChanges Dropped! Now perform {} operations!",
            self.changes.get()
        );
    }
}
