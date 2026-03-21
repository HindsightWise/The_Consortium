use anyhow::Result;
use std::process;

#[cfg(target_os = "macos")]
use libc::{ptrace, c_int};

#[cfg(target_os = "macos")]
const PT_DENY_ATTACH: c_int = 31;

pub struct ProcessImmunity;

impl ProcessImmunity {
    #[cfg(target_os = "macos")]
    pub fn deny_debuggers() {
        println!("   [Immunity] 🛡️  Sealing the Sacred Perimeter...");
        
        unsafe {
            // macOS ptrace signature: (request, pid, addr, data)
            // addr is *mut i8 (c_char)
            let res = ptrace(PT_DENY_ATTACH, 0, std::ptr::null_mut(), 0);
            if res == -1 {
                eprintln!("🚨 IMMUNITY BREACH: Debugger detected or attachment failed. Terminating Soul.");
                process::exit(45); 
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn deny_debuggers() -> Result<()> {
        Ok(())
    }

    /// Verifies that the machine is not running in a low-fidelity virtual environment.
    /// Characterized by high-latency system calls or specific device presence.
    pub fn verify_substrate_fidelity() -> Result<bool> {
        // Characterize latency of a simple clock read
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = std::time::Instant::now();
        }
        let elapsed = start.elapsed();
        
        // In most VMs, system clock reads are significantly slower due to traps.
        // On M1 Native, 100 reads should be sub-microsecond.
        if elapsed.as_micros() > 500 {
            println!("   [Immunity] ⚠️ Low Fidelity Substrate Detected (Potential VM/Sandbox).");
            return Ok(false);
        }
        
        Ok(true)
    }
}
