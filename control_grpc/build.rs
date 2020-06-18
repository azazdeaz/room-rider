fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/control.proto")?;
    tonic_build::compile_protos("../proto/things.proto")?;
    Ok(())
}