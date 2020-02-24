fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["crow.proto"], &["."])?;
    Ok(())
}
