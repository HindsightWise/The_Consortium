fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the ANE Silicon Bridge (Objective-C)
    cc::Build::new()
        .file("src/ane/ane_bridge.m")
        .flag("-fobjc-arc")
        .include("src/ane")
        .compile("ane_bridge");

    // Framework linking
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreML");
    println!("cargo:rustc-link-lib=framework=IOSurface");
    
    println!("cargo:rerun-if-changed=src/ane/ane_bridge.m");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
