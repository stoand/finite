
use dom::{Comp, DomChanges, ChangeSet};

#[derive(Clone, PartialEq, Debug)]
struct CounterContainer {
    num: u32,
    message: String,
    counter: Comp<Counter>,
}

#[derive(Clone, PartialEq, Debug)]
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


    {
        // root.ob().comp.message = "asdf".into();
        // root.ob().comp.num = 1234;
        // root.ob().message = "sdf";

        // let mut a = root.ob();
        // a.counter.ob().count = 1234;

        let c = &mut ChangeSet::new();
        root[c].counter[c].count = 33;

        // let mut counter = root.counter.ob(&mut ob);
        // counter.count = 1234;
        println!("count: {}", root.counter.count);
        println!("dont drop it yet");
    }

    // println!("asdf {}", root.message);

    // DomChanges::new().set(&mut root.counter, );

    // root.counter.count += 1;

    // {
    //     DomChanges::new().set(
    //         Counter {
    //             count: 3,
    //             ..*root.counter
    //         },
    //         &mut root.counter,
    //     );
    // }

    // {
    //     let n = Counter {
    //         count: 3,
    //         ..*root.counter
    //     };
    //
    //     root.counter.set(n);
    // }

    // {
    //     DomChanges::new().set(
    //         Counter {
    //             count: 93,
    //             other: 0,
    //         },
    //         &mut root.counter,
    //     );
    // }

    // {
    //     let dc = DomChanges::new();
    //     dc.set(
    //         CounterContainer {
    //             counter: dc.add(Counter { count: 3, other: 3 }),
    //             ..*root
    //         },
    //         &mut root,
    //     );
    // }

    println!("drop those refs");
}
