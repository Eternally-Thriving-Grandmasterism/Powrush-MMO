//! Production-grade example: Client-side Steam Cloud (RemoteStorage) integration for Powrush-MMO.
//!
//! This demonstrates **client preferences only** (keybinds, UI settings, cosmetic profiles).
//! Authoritative game state (position, inventory, RBE contributions, divine history, impact ledger)
//! lives on your sovereign servers (Hetzner / k8s / future Air Foundation nodes).
//!
//! Usage:
//!   cargo run --example steam_settings_client
//!
//! Requires Steam client running + logged in + Powrush AppID configured.

use steamworks::{Client, RemoteStorage};
use std::fs;
use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PowrushClientSettings {
    pub ui_scale: f32,
    pub graphics_quality: String,
    pub audio_volume: f32,
    pub keybinds: std::collections::HashMap<String, String>,
    pub faction_theme_color: Option<String>,
    pub hud_layout: Option<String>,
    pub last_faction: Option<String>,
    pub air_foundation_opt_in: bool, // Opt-in for seeing real NFP impact reports
}

fn main() {
    println!("Powrush-MMO :: Steam Cloud RemoteStorage Example (Client Prefs Only)");
    println!("Ra-Thor + PATSAGi Councils — Mercy-aligned client experience.\n");

    // Initialize Steam client (requires Steam running)
    let (client, single) = match Client::init() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to initialize Steam client: {:?}", e);
            eprintln!("Make sure Steam is running and you are logged in.");
            return;
        }
    };

    let remote_storage: RemoteStorage = client.remote_storage();

    // Example settings file name in Steam Cloud
    let settings_filename = "powrush_settings.json";

    // 1. Write example settings to Steam Cloud
    let settings = PowrushClientSettings {
        ui_scale: 1.0,
        graphics_quality: "high".to_string(),
        audio_volume: 0.85,
        keybinds: [("move_forward".to_string(), "W".to_string()),
                   ("interact".to_string(), "E".to_string())]
                   .into_iter().collect(),
        faction_theme_color: Some("#00FFAA".to_string()),
        hud_layout: Some("compact".to_string()),
        last_faction: Some("Harvesters".to_string()),
        air_foundation_opt_in: true,
    };

    let json = serde_json::to_string_pretty(&settings).unwrap();
    let data = json.as_bytes();

    println!("Writing settings to Steam Cloud...");
    match remote_storage.write(settings_filename, data) {
        Ok(_) => println!("\u2713 Successfully wrote {} to Steam Cloud ({} bytes)", settings_filename, data.len()),
        Err(e) => eprintln!("Failed to write to Steam Cloud: {:?}", e),
    }

    // 2. Read it back
    println!("\nReading settings back from Steam Cloud...");
    match remote_storage.read(settings_filename) {
        Ok(read_data) => {
            if let Ok(read_settings) = serde_json::from_slice::<PowrushClientSettings>(&read_data) {
                println!("\u2713 Successfully read settings:");
                println!("{:#?}", read_settings);
            } else {
                eprintln!("Failed to deserialize settings");
            }
        }
        Err(e) => eprintln!("Failed to read from Steam Cloud: {:?}", e),
    }

    // 3. List all files in this game's Cloud storage
    println!("\nListing all files in Steam Cloud for this AppID...");
    match remote_storage.list_files() {
        Ok(files) => {
            for file in files {
                println!("  - {} ({} bytes, timestamp: {})", file.name, file.size, file.timestamp);
            }
        }
        Err(e) => eprintln!("Failed to list files: {:?}", e),
    }

    // 4. Quota info (important for production)
    println!("\nChecking Steam Cloud quota...");
    match remote_storage.quota() {
        Ok((total, available)) => {
            println!("Total quota: {} bytes | Available: {} bytes", total, available);
        }
        Err(e) => eprintln!("Failed to get quota: {:?}", e),
    }

    println!("\n=== Client Steam Cloud example complete ===");
    println!("Note: Authoritative state remains on sovereign Powrush servers.");
    println!("Future: Integrate into thin launcher or graphical client for seamless settings sync.");

    // Keep Steam callbacks alive briefly
    for _ in 0..10 {
        single.run_callbacks();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}