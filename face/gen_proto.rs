fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from("/tmp/akkokanika_proto_out");
    let _ = std::fs::create_dir_all(&out_dir);
    
    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&["proto/akkokanika.proto"], &["proto"])?;
        
    Ok(())
}
