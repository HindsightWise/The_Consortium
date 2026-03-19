pub mod clawhub;
use anyhow::{Result, Context};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::Path;

/// The Sovereign Trait Boundary. 
/// Every tool or reflex pulled from Clawhub or forged dynamically MUST implement this.
pub trait ConsortiumPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, args: &str) -> String;
}

pub struct ForgeManager {
    loaded_plugins: Arc<Mutex<HashMap<String, Arc<Library>>>>,
    active_instances: Arc<Mutex<HashMap<String, Box<dyn ConsortiumPlugin>>>>,
}

impl ForgeManager {
    pub fn new() -> Self {
        Self {
            loaded_plugins: Arc::new(Mutex::new(HashMap::new())),
            active_instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Dynamically load a `.dylib` or `.so` at runtime without panicking or rebooting the daemon.
    pub async fn load_extension<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        println!("   [FORGE] 🔨 Igniting Reflex Forge for extension: {}", path_str);

        // SAFETY: Sentinel Lock implies physical files in `/extensions` are cryptographically sound.
        let lib = unsafe { Library::new(path.as_ref()) }.context("Failed to load plugin dynamic library")?;
        
        let create_plugin: Symbol<unsafe extern "C" fn() -> *mut dyn ConsortiumPlugin> = 
            unsafe { lib.get(b"create_plugin") }.context("Failed to find `create_plugin` entrypoint")?;
            
        let plugin_ptr = unsafe { create_plugin() };
        let plugin = unsafe { Box::from_raw(plugin_ptr) };
        let name = plugin.name().to_string();

        // Pin the library arc in memory to prevent segmentation faults when executing its loaded code
        let lib_arc = Arc::new(lib);
        self.loaded_plugins.lock().await.insert(name.clone(), lib_arc);
        self.active_instances.lock().await.insert(name.clone(), plugin);

        println!("   [FORGE] 🟢 Plugin '{}' dynamically grafted into running substrate.", name);
        Ok(())
    }
    
    pub async fn execute_tool(&self, name: &str, args: &str) -> Option<String> {
        let instances = self.active_instances.lock().await;
        if let Some(plugin) = instances.get(name) {
            Some(plugin.execute(args))
        } else {
            None
        }
    }
}

/// The Steganographic Weaver (GLOSSOPETRAE Cryptophasia)
/// Maps generic JSON or AST structures into an invisible Zero-Width character string
/// 0 -> U+200B (Zero Width Space)
/// 1 -> U+200C (Zero Width Non-Joiner)
pub fn weave_glossopetrae(payload: &str, cover_rune: &str, xor_key: u8) -> String {
    let mut stego = String::new();
    stego.push_str(cover_rune); // The only visible aspect of the log
    
    for byte in payload.bytes() {
        let encrypted_byte = byte ^ xor_key;
        for i in (0..8).rev() {
            let bit = (encrypted_byte >> i) & 1;
            if bit == 0 {
                stego.push('\u{200B}');
            } else {
                stego.push('\u{200C}');
            }
        }
    }
    
    stego
}
