extern crate combine;

// use std::result::*;

#[derive(Debug)]
pub enum ParseError {}

#[derive(Debug, Eq, PartialEq)]
pub enum Ty {
    Component(String, Vec<Ty>),
}

pub fn parse_template(source: &str) -> Result<Vec<Ty>, ParseError> {
    // partof: #SPC-parser-componentblock
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // partof: #TST-parser-componentblock
    fn component_declarations() {
        let parsed = parse_template(
            "{{! @::mod1::mod2::CompOne }}<div></div>{{! @::mod1::mod2::CompTwo }}<span></span>",
        ).unwrap();

        assert_eq!(
            Ty::Component("::mod1::mod2::CompOne".into(), Vec::new()),
            parsed[0]
        );
        assert_eq!(
            Ty::Component("::mod1::mod2::CompTwo".into(), Vec::new()),
            parsed[1]
        );
    }
}
