#[cfg(feature = "schema")]
#[test]
fn test_generate_schemas() -> anyhow::Result<()> {
    use raxb_xmlschema_build::XmlSchemaRegistry;

    let mut registry =
        XmlSchemaRegistry::new("./target/schemas").with_cache_dir("./schemas/remote");
    registry.register("./schemas/V0_9_1/xwasser.xsd")?;
    assert!(registry.save().is_ok());
    Ok(())
}
