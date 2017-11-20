use std::ops::{Deref, DerefMut, Add, Index, IndexMut};
use std::cell::Cell;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub struct Comp<T>(pub T);


pub struct ChangeSet;

impl ChangeSet {
    pub fn new() -> Self {
        ChangeSet {}
    }
}

impl<'a, T: Clone + PartialEq + Debug> Index<&'a mut ChangeSet> for Comp<T> {
    type Output = T;

    fn index<'b>(&'b self, index: &'a mut ChangeSet) -> &'b T {
        &self.0
    }
}

impl<'a, T: Clone + PartialEq + Debug> IndexMut<&'a mut ChangeSet> for Comp<T> {
    fn index_mut<'b>(&'b mut self, index: &'a mut ChangeSet) -> &'b mut T {
        &mut self.0
    }
}

// impl<T: Clone + PartialEq + Debug> Comp<T> {
//     pub fn ob<'a, N>(&'a mut self, observer: &'a mut Observer<'a, T>) -> &'a mut Observer<'a, T> {
//         observer.observe(self);
//         observer
//     }

//     pub fn c(&mut self, _change_set: &mut ChangeSet) -> &mut T {
//         &mut self.0
//     }
// }

// pub struct Observer<'a, T: 'a + Clone + PartialEq + Debug> {
//     pub comp: Option<&'a mut Comp<T>>,
//     pub orig_state: Option<T>,
// }

// impl<'a, T: Clone + PartialEq + Debug> Observer<'a, T> {
//     pub fn new() -> Observer<'a, T> {
//         Observer {
//             comp: None,
//             orig_state: None,
//             // orig_state: comp.0.clone(),
//             // comp,
//         }
//     }

//     fn observe(&mut self, comp: &'a mut Comp<T>) {
//         self.comp = Some(comp);
//     }
// }


impl<T: Clone + PartialEq + Debug> Deref for Comp<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// impl<T: Clone + PartialEq + Debug> DerefMut for Comp<T> {
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.0
//     }
// }

// impl<'a, T: Clone + PartialEq + Debug> Deref for Observer<'a, T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.comp.unwrap().0
//     }
// }

// impl<'a, T: Clone + PartialEq + Debug> DerefMut for Observer<'a, T> {
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.comp.unwrap().0
//     }
// }


// impl<'a, T: Clone + PartialEq + Debug> Drop for Observer<'a, T> {
//     fn drop(&mut self) {
//         if let (&Some(ref a), &Some(ref b)) = (&self.comp, &self.orig_state) {

//             println!("got val {:?}", b);

//             // if a.0 != b {
//             //     println!(
//             //         "observer dropped, found changes: {:?}",
//             //         self.comp.unwrap().0
//             //     );
//             // } else {
//             //     println!("observer dropped, no changes: {:?}", self.comp.unwrap().0);
//             // }
//         }
//     }
// }

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
}

impl Drop for DomChanges {
    fn drop(&mut self) {
        println!(
            "DomChanges Dropped! Now perform {} operations!",
            self.changes.get()
        );
    }
}
