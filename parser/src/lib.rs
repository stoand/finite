extern crate combine;

// use std::result::*;

#[derive(Debug)]
pub enum ParseError {}

#[derive(Debug, Eq, PartialEq)]
pub enum Ty {
    Component(String, Vec<Ty>),
}

pub fn parse_template(source: &str) -> Result<Vec<Ty>, ParseError> {
    // partof: #SPC-parse-componentblock

    // todo

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // partof: #TST-parse-componentblock
    fn component_declarations() {
        let parsed =
            parse_template("{{! @CompOne }}<div></div>{{! @CompTwo }}<span></span>").unwrap();

        assert_eq!(Ty::Component("a".into(), Vec::new()), parsed[0]);
    }
}
