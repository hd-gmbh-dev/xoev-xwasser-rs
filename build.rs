use raxb_xmlschema_build::XmlSchemaRegistry;

fn main() -> anyhow::Result<()> {
    let mut registry =
        XmlSchemaRegistry::new("./target/schemas").with_cache_dir("./schemas/remote");
    registry.register("./schemas/V0_5_3/xwasser.xsd")?;
    registry.save()?;

    Ok(())
}
