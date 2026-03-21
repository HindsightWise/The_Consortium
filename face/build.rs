fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/akkokanika.proto")?;

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "macos" {
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

        println!("cargo:rerun-if-changed=src/mcp/ane/ane_bridge.m");
    } else {
        println!("cargo:rustc-cfg=not_macos");

        // Write dummy implementation
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let dummy_c = format!("{}/dummy_ane_bridge.c", out_dir);
        std::fs::write(&dummy_c, "
#include <stdint.h>
void* ane_iosurface_create(int w, int h, int d) { return 0; }
void* ane_iosurface_get_ptr(void* a) { return 0; }
void ane_iosurface_destroy(void* a) {}
void* ane_model_create_from_mil(const char* a) { return 0; }
void ane_model_compile(void* a) {}
void ane_model_load(void* a) {}
void ane_model_evaluate(void* a, void* b, void* c) {}
void ane_model_unload(void* a) {}
void ane_model_destroy(void* a) {}
void ane_bridge_init() {}
void ane_bridge_compute() {}
void ane_bridge_teardown() {}
")?;

        cc::Build::new()
            .file(&dummy_c)
            .flag("-w")
            .compile("ane_bridge");
    }

    println!("cargo:rerun-if-changed=proto/akkokanika.proto");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
