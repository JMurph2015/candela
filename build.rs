fn main() -> Result<(), std::io::Error> {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_protos(&["src/types.proto"], &["src/"])?;
    Ok(())
}
