enum DomOp<'a> {
    SetInner { selector: &'a str, value: &'a str },
}

// #[derive(DomRenderable)]
struct App {
    message: &'static str,
    counter: Counter,
}

// #[derive(DomRenderable)]
struct Counter {
    count: u32,
}

enum DomRenderable {
    Text(String),
    Component(),
}

impl<'a> From<&'a str> for DomRenderable {
    fn from(v: &'a str) -> DomRenderable {
        DomRenderable::Text(v.to_string())
    }
}

impl From<u32> for DomRenderable {
    fn from(v: u32) -> DomRenderable {
        DomRenderable::Text(v.to_string())
    }
}

impl From<Counter> for DomRenderable {
    fn from(_v: Counter) -> DomRenderable {
        DomRenderable::Component()
    }
}

trait Component {
    const TPL: &'static str;

    fn render_component(&mut Vec<DomOp>, Option<&Self>);
    // fn render_compon() -> Self
}

impl Component for App {
    const TPL: &'static str = include_str!("./app.hbs");

    fn render_component(ops: &mut Vec<DomOp>, prev: Option<&Self>) {
        // ops.
        // ops.push();
    }
}

impl Component for Counter {
    const TPL: &'static str = include_str!("./counter.hbs");

    fn render_component(ops: &mut Vec<DomOp>) {

    }
}

// #[test]
fn asdf() {
    let mut state = App {
        message: "howdy",
        counter: Counter { count: 300 },
    };

    let mut buf = String::new();
    state.render_tpl(&mut buf);

    assert_eq!(buf, "");
}
