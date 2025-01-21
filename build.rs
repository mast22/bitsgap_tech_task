fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/bitsgrap_tech_task.proto")?;
    Ok(())
}
