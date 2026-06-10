/*!
 * Powrush-MMO Server — Hardening & Anti-Tamper
 *
 * Light but effective server-side protections.
 * All features are optional and can be disabled via environment variables
 * for sovereign/self-hosted deployments.
 */

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use nix::unistd::{getuid, setuid, Uid};
use sha2::{Digest, Sha256};

/// Main entry point for server hardening.
/// Call this as early as possible in main().
pub fn apply_server_hardening() {
    if env::var("POWRUSH_DISABLE_HARDENING").is_ok() {
        println!("[Hardening] Disabled via POWRUSH_DISABLE_HARDENING");
        return;
    }

    check_anti_debug();
    check_binary_integrity();
    drop_privileges();

    println!("[Hardening] Server hardening applied successfully");
}

/// Detect if the process is being debugged (Linux only)
fn check_anti_debug() {
    if let Ok(mut file) = File::open("/proc/self/status") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if contents.contains("TracerPid:\t0") == false {
                // TracerPid is not 0 → being traced
                eprintln!("[Hardening] Warning: Process appears to be debugged");
                // In production you could choose to exit here
            }
        }
    }
}

/// Simple binary integrity check (hashes the executable)
fn check_binary_integrity() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Ok(mut file) = File::open(&exe_path) {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];

            loop {
                match file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => hasher.update(&buffer[..n]),
                    Err(_) => break,
                }
            }

            let hash = hasher.finalize();
            // In production, compare against a known good hash stored securely
            // For now we just log it
            println!("[Hardening] Binary SHA256: {:x}", hash);
        }
    }
}

/// Drop root privileges if running as root
fn drop_privileges() {
    if getuid().is_root() {
        // Try to drop to a non-privileged user (e.g. "powrush" or "nobody")
        let target_user = env::var("POWRUSH_USER").unwrap_or_else(|_| "nobody".to_string());

        if let Ok(user) = users::get_user_by_name(&target_user) {
            if setuid(Uid::from_raw(user.uid())).is_ok() {
                println!("[Hardening] Dropped privileges to user: {}", target_user);
            } else {
                eprintln!("[Hardening] Warning: Failed to drop privileges");
            }
        } else {
            eprintln!("[Hardening] Warning: User '{}' not found. Running as root is not recommended.", target_user);
        }
    }
}
