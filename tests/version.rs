use xoev_xwasser::Version;


fn testfile(name: &str) -> String {
    std::fs::read_to_string(
        std::env::current_dir()
            .unwrap()
            .join(&format!("tests/{name}")),
    )
    .unwrap()
}

#[test]
pub fn detect_version_test() -> anyhow::Result<()> {
    let xml = testfile("quality_report_minimal.xml");
    let xml = &xml[0..1024];
    let version = xoev_xwasser::detect_version(xml);
    assert_eq!(version, Version::V0_5_3);
    Ok(())
}