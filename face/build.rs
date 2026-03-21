fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/akkokanika.proto")?;
    
    // Only compile the ANE bridge Objective-C code on macOS
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rerun-if-changed=src/core/cognitive/ane_bridge.m");
        cc::Build::new()
            .file("src/core/cognitive/ane_bridge.m")
            .flag("-fmodules")
            .compile("ane_bridge");

        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreML");
    }

    Ok(())
}
