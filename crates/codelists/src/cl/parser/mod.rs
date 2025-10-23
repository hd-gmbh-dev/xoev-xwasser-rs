pub mod input;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_parse() {
        let icl: input::CodeList = raxb::de::from_str(include_str!(
            "../../../data/V0_9_5/Parameterausprägung_4.xml"
        ))
        .expect("parsable code list");

        let _: crate::cl::model::CodeList = icl.into();
    }
}
