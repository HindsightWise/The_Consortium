fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/akkokanika.proto")?;

    #[cfg(target_os = "macos")]
    {
        // Build the ANE Silicon Bridge (Objective-C)
        cc::Build::new()
            .file("src/mcp/ane/ane_bridge.m")
            .flag("-fobjc-arc")
            .include("src/mcp/ane")
            .compile("ane_bridge");

        // Framework linking
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreML");
        println!("cargo:rustc-link-lib=framework=IOSurface");
    }

    println!("cargo:rerun-if-changed=proto/akkokanika.proto");
    println!("cargo:rerun-if-changed=src/mcp/ane/ane_bridge.m");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
