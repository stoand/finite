
use dom::{Comp, DomChanges};

struct CounterContainer {
    message: &'static str,
    counter: Comp<Counter>,
}

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
            message: "hey",
            counter: dc.add(Counter { count: 0, other: 0 }),
        })
    };

    // DomChanges::new().set(&mut root.counter, );

    // root.counter.count += 1;

    {
        DomChanges::new().set(
            Counter {
                count: 3,
                ..*root.counter
            },
            &mut root.counter,
        );
    }

    {
        let n = Counter {
            count: 3,
            ..*root.counter
        };
        // root.counter.set(n);
    }

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
