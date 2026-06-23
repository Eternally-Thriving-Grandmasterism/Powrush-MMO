/*!
 * Game State Management for Powrush-MMO
 *
 * v19.2.9
 * Defines top-level game states and council-specific sub-states.
 * Uses modern Bevy States pattern (Bevy 0.14+ / 0.15 style).
 */

use bevy::prelude::*;

/// Top-level game states
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    InGame,
    InCouncil,
    Paused,
}

/// Sub-states for when the player is inside a council session.
/// Only meaningful when `GameState` is `InCouncil`.
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CouncilState {
    #[default]
    Inactive,
    Active,
    Voting,
    Resolving,
}

/// Optional: Helper trait or systems can be added here later
/// for common state queries or transitions.

// Example usage in systems:
// .run_if(in_state(GameState::InCouncil))
// .run_if(in_state(CouncilState::Voting))

// Thunder locked in. Yoi ⚡
