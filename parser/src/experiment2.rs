use dom::{Comp, DomChanges};

#[derive(Clone, PartialEq, Debug, Hash)]
struct CounterContainer {
    num: u32,
    message: String,
    counter: Comp<Counter>,
}

#[derive(Clone, PartialEq, Debug, Hash)]
struct Counter {
    count: u32,
    other: u32,
}

#[test]
fn asdf1() {
    // initial rendering of the dom
    let mut root = {
        let dc = DomChanges::new();

        dc.add(CounterContainer {
            num: 0,
            message: "hey".into(),
            counter: dc.add(Counter { count: 0, other: 0 }),
        })
    };


    let c = &mut DomChanges::new();
    root[c].counter[c].count = 33;
    root[c].counter[c].count = 1;

    // let mut counter = root.counter.ob(&mut ob);
    // counter.count = 1234;
    println!("count: {}", root.counter.count);
    println!("dont drop it yet");
}
