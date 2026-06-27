/*!
 * Generalized Settings System for Powrush-MMO
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

pub mod game_settings;
pub mod audio_settings;
pub mod persistence;

pub use game_settings::GameSettings;
pub use audio_settings::AudioSettings;
pub use persistence::{load_settings, save_settings};
