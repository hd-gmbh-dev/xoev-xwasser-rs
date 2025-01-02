pub mod input;
pub mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_parse() {
        let icl: input::CodeList = raxb::de::from_str(include_str!(
            "../../../data/V0_8_0/Parameterausprägung_3.xml"
        ))
        .expect("parsable code list");

        let _: crate::cl::model::CodeList = icl.into();
    }
}
