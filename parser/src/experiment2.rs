use comp::Comp;

struct CounterContainer {
    message: &'static str,
    counter: Comp<Counter>,
}

struct Counter {
    count: u32,
}

#[test]
fn asdf1() {
    // initial rendering of the dom
    let mut root = {
        let mut dc = DomChanges::new();
        let counter = dc.add(Counter { count: 0 });
        dc.add(CounterContainer {
            message: "hey",
            counter,
        })
    };

    {
        // let mut dc = ;
        // root.counter.set(dc, Counter { count: 32 });
        DomChanges::new()
            .set(&mut root.counter, Counter { count: 3 })
            .set(&mut root.counter, Counter { count: 23 });
    }

    {
        DomChanges::new().set(&mut root.counter, Counter { count: 93 });
    }
}
