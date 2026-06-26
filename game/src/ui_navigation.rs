/*!
 * Spatial Grid UI Navigation System
 *
 * v12 - ChaCha20-Poly1305 Machine-Bound Encryption for Settings
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};
use bevy::audio::PlaybackSettings;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use sha2::{Sha256, Digest};

// ... (Focusable, Focused, UiFocus, NavDirection, UiAudioSettings remain the same)

/// Derives a 32-byte encryption key from the machine's unique ID
fn get_machine_key() -> Result<Key, String> {
    let uid = machine_uid::get().map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(uid.as_bytes());
    let hash = hasher.finalize();
    Ok(*Key::from_slice(&hash[..32]))
}

/// Encrypts the settings using ChaCha20-Poly1305 with machine binding
fn encrypt_settings(settings: &UiAudioSettings) -> Result<Vec<u8>, String> {
    let key = get_machine_key()?;
    let cipher = ChaCha20Poly1305::new(&key);

    let plaintext = ron::to_string(settings).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&[0u8; 12]); // In production, use a random nonce + store it

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| e.to_string())?;

    // Prepend nonce for storage (currently fixed for simplicity)
    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

/// Decrypts settings using the machine-bound key
fn decrypt_settings(data: &[u8]) -> Result<UiAudioSettings, String> {
    if data.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let key = get_machine_key()?;
    let cipher = ChaCha20Poly1305::new(&key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| e.to_string())?;

    let settings: UiAudioSettings =
        ron::from_str(&String::from_utf8_lossy(&plaintext)).map_err(|e| e.to_string())?;

    Ok(settings)
}

/// Loads encrypted settings from disk
pub fn load_ui_settings(mut commands: Commands) {
    if let Some(path) = get_settings_path() {
        if let Ok(encrypted_data) = fs::read(&path) {
            match decrypt_settings(&encrypted_data) {
                Ok(settings) => {
                    commands.insert_resource(settings);
                    info!("Loaded encrypted settings from {:?}", path);
                    return;
                }
                Err(e) => {
                    warn!("Failed to decrypt settings: {}. Using defaults.", e);
                }
            }
        }
    }

    info!("Using default UI audio settings");
    commands.insert_resource(UiAudioSettings::default());
}

/// Saves settings encrypted with machine binding
pub fn save_ui_settings(settings: Res<UiAudioSettings>) {
    if settings.is_changed() {
        if let Some(path) = get_settings_path() {
            match encrypt_settings(&*settings) {
                Ok(encrypted) => {
                    if let Err(e) = fs::write(&path, encrypted) {
                        warn!("Failed to write encrypted settings: {}", e);
                    }
                }
                Err(e) => {
                    warn!("Failed to encrypt settings: {}", e);
                }
            }
        }
    }
}

// ... (rest of the file remains the same)

pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Startup, load_ui_settings)
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, apply_focus_visuals)
            .add_systems(Update, activate_focused_button)
            .add_systems(Update, play_focus_change_sound)
            .add_systems(Update, play_button_activate_sound)
            .add_systems(Update, save_ui_settings);
    }
}
