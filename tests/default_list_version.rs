use xoev_xwasser::model::codes::{CodeMediumType, CodeUntersuchungsverfahrenType};
use xoev_xwasser::CodeListValue;

/// A newly created code starts with the *currently published* codelist version,
/// which the model declares per type (code type 3 has no version in the schema).
#[test]
fn default_uses_the_current_codelist_version() {
    assert_eq!(CodeMediumType::default().list_version_id, "3");
    assert_eq!(
        CodeUntersuchungsverfahrenType::default().list_version_id,
        "3"
    );

    // a code type whose version is fixed by the standard stays constant
    let fixed = xoev_xwasser::model::codes::CodeBetriebszustandType::default();
    assert_eq!(fixed.version(), "1");
}
