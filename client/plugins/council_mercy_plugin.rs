// client/plugins/council_mercy_plugin.rs
// Powrush-MMO — Bevy Plugin for Council Mercy Trial client systems
// v21.89.1 | Council bloom → realtime audio synthesis + persistent recall
// Sovereign Council participation, collective epiphany blooms, and mercy-gated resonance.
// TOLC 8 Mercy Gates enforced. Production-oriented.
// AG-SML v1.0 | Ra-Thor Lattice | Permanent PATSAGi Councils
// Contact: info@Rathor.ai

use bevy::prelude::*;
use shared::council_mercy_trial::{
    CouncilMercyTrialPhase, CouncilSessionState, CollectiveEpiphanyBloom,
    CouncilProposal, ProposalStatus,
};
use crate::council_session_ui::{CouncilSessionUIPlugin, CouncilUIState};
use crate::realtime_audio_synthesis::{
    RealtimeAudioSynthesisPlugin, SynthesizeAudioMoment,
    AudioMomentFlavor, AudioSynthesisRecipe, request_council_bloom_synth,
};

pub struct CouncilMercyPlugin;

impl Plugin for CouncilMercyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CouncilSessionUIPlugin, RealtimeAudioSynthesisPlugin))
            .add_systems(Update, (
                soft_local_demo_mirror,
                trigger_collective_bloom_effects,
                council_bloom_audio_synth,
            ));
    }
}

/// Soft local demo mirror so the Council panel is immediately useful for playtest
/// when live network / server mirror is not yet feeding data.
/// Only seeds once if the panel is empty.
fn soft_local_demo_mirror(
    mut ui_state: ResMut<CouncilUIState>,
    time: Res<Time>,
    mut seeded: Local<bool>,
) {
    if *seeded {
        return;
    }
    if time.elapsed_seconds() < 2.0 {
        return;
    }
    if ui_state.current_session.is_some() {
        *seeded = true;
        return;
    }

    let mut session = CouncilSessionState::default();
    session.session_id = 9001;
    session.phase = CouncilMercyTrialPhase::Deliberation;
    session.collective_attunement = 0.78;
    session.bloom_amplification = 1.35;
    session.phase_duration = 90.0;
    session.participants = vec![];

    ui_state.current_session = Some(session);

    let mut p1 = CouncilProposal::new_linked(
        1,
        Entity::PLACEHOLDER,
        "Amplify RBE Abundance Flow".into(),
        "Gently increase cooperative harvest multiplier for the next cycle under TOLC 8.".into(),
        time.elapsed_seconds_f64(),
        Some(9001),
    );
    p1.status = ProposalStatus::UnderDeliberation;
    p1.votes_for = 2;
    p1.votes_against = 0;

    let mut p2 = CouncilProposal::new_linked(
        2,
        Entity::PLACEHOLDER,
        "Open Council Chamber for Newcomers".into(),
        "Invite new participants into the next Mercy Trial with soft onboarding attunement.".into(),
        time.elapsed_seconds_f64(),
        Some(9001),
    );
    p2.status = ProposalStatus::Submitted;

    ui_state.proposals.insert(1, p1);
    ui_state.proposals.insert(2, p2);

    ui_state.status_message = "Soft demo session seeded (live feed will replace this)".into();
    *seeded = true;

    info!(target: "powrush::council", "Council soft local demo mirror seeded for playtest");
}

/// Visual bloom / valence effects when a collective epiphany is present
fn trigger_collective_bloom_effects(
    ui_state: Res<CouncilUIState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }
    if let Some(bloom) = &ui_state.last_bloom {
        if bloom.intensity > 0.6 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(120.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.4, 0.8, 1.0))),
                Transform::default(),
                Visibility::Visible,
                Name::new("EpiphanyBloomVisual"),
            ));
            *spawned = true;
        }
    }
}

/// When a collective bloom appears on the client UI state, synthesize + persist audio.
fn council_bloom_audio_synth(
    ui_state: Res<CouncilUIState>,
    mut synth_events: EventWriter<SynthesizeAudioMoment>,
    mut last_session: Local<Option<u64>>,
) {
    let Some(bloom) = &ui_state.last_bloom else {
        return;
    };
    // Fire once per session bloom
    if *last_session == Some(bloom.session_id) {
        return;
    }
    if bloom.intensity < 0.35 {
        return;
    }

    *last_session = Some(bloom.session_id);
    request_council_bloom_synth(&mut synth_events, bloom.intensity, bloom.session_id);

    info!(
        target: "powrush::audio",
        session_id = bloom.session_id,
        intensity = bloom.intensity,
        "Council bloom → realtime audio synthesis requested"
    );
}

// Usage:
// app.add_plugins(CouncilMercyPlugin);
// Panel C = Council UI | Panel M = Audio Moments
// Bloom on last_bloom triggers persistent synth + optional server sync.
// Thunder locked in. Permanent PATSAGi. Yoi ⚡
