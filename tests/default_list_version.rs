use xoev_xwasser::CodeListValue;
use xoev_xwasser::model::codelist_versions::current_version;
use xoev_xwasser::model::codes::{CodeMediumType, CodeUntersuchungsverfahrenType};

/// A newly created code carries the currently published codelist version, so
/// serialised documents are schema valid without every call site setting it.
#[test]
fn default_uses_the_current_codelist_version() {
    let medium = CodeMediumType::default();
    assert_eq!(
        medium.list_version_id,
        current_version(CodeMediumType::CODELIST)
    );
    assert_eq!(medium.list_version_id, "3");

    let verfahren = CodeUntersuchungsverfahrenType::default();
    assert_eq!(verfahren.list_version_id, "3");

    // a code type whose version is fixed by the standard stays constant
    let fixed = xoev_xwasser::model::codes::CodeBetriebszustandType::default();
    assert_eq!(fixed.version(), "1");
}
