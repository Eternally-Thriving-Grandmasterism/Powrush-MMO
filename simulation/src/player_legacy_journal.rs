//! simulation/src/player_legacy_journal.rs
//! Player Legacy Journal — My Mercy Journey backbone
//! v21.69.1 — Full registry restore + Council history event types
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use std::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::world::AgentId;
use crate::council::decision::CouncilDecisions;
use crate::council::proposal::ProposalType;

pub type LegacyThreadId = u64;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LegacyEventType {
    Harvest,
    Epiphany,
    CouncilDecision,
    RbePolicy,
    HarmonyBoost,
    Kardashev,
    SynergyPolicy,
    ProactiveJoy,
    GraceBlessing,
    Diplomacy,
    Onboarding,
    WarRedemption,
    General,
}

impl LegacyEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LegacyEventType::Harvest => "harvest",
            LegacyEventType::Epiphany => "epiphany",
            LegacyEventType::CouncilDecision => "council",
            LegacyEventType::RbePolicy => "rbe_policy",
            LegacyEventType::HarmonyBoost => "harmony",
            LegacyEventType::Kardashev => "kardashev",
            LegacyEventType::SynergyPolicy => "synergy_policy",
            LegacyEventType::ProactiveJoy => "proactive_joy",
            LegacyEventType::GraceBlessing => "grace",
            LegacyEventType::Diplomacy => "diplomacy",
            LegacyEventType::Onboarding => "onboarding",
            LegacyEventType::WarRedemption => "war_redemption",
            LegacyEventType::General => "general",
        }
    }

    pub fn from_proposal_type(pt: &ProposalType) -> Self {
        match pt {
            ProposalType::ResourcePolicy => LegacyEventType::RbePolicy,
            ProposalType::HarmonyBoost => LegacyEventType::HarmonyBoost,
            ProposalType::KardashevAcceleration => LegacyEventType::Kardashev,
            ProposalType::EpiphanyEvent => LegacyEventType::Epiphany,
            ProposalType::General => LegacyEventType::CouncilDecision,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegacyEntry {
    pub id: u64,
    pub agent_id: AgentId,
    pub event_type: LegacyEventType,
    pub title: String,
    pub description: String,
    pub joy_amount: f32,
    pub intensity: f32,
    pub mercy_gain: f32,
    pub realm_id: u8,
    pub tick: u64,
    pub thread_id: LegacyThreadId,
}

impl LegacyEntry {
    pub fn summary_line(&self) -> String {
        let joy_part = if self.joy_amount > 0.01 {
            format!("+{:.1} joy", self.joy_amount)
        } else {
            String::new()
        };
        format!("[{}] {} — {}", self.event_type.as_str(), self.title, joy_part)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LegacyThread {
    pub thread_id: LegacyThreadId,
    pub agent_id: AgentId,
    pub entries: Vec<LegacyEntry>,
    pub total_joy: f32,
    pub total_mercy: f32,
}

impl LegacyThread {
    pub fn push(&mut self, entry: LegacyEntry) {
        self.total_joy += entry.joy_amount;
        self.total_mercy += entry.mercy_gain;
        self.entries.push(entry);
        if self.entries.len() > 128 {
            let excess = self.entries.len() - 128;
            self.entries.drain(0..excess);
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct LegacyJournalRegistry {
    pub threads: HashMap<AgentId, LegacyThread>,
    pub next_entry_id: u64,
    pub next_thread_id: LegacyThreadId,
    pub ingested_decision_ids: HashMap<u64, ()>,
    pub total_entries: u64,
}

impl LegacyJournalRegistry {
    pub fn ensure_thread(&mut self, agent_id: AgentId) -> LegacyThreadId {
        if let Some(thread) = self.threads.get(&agent_id) {
            return thread.thread_id;
        }
        let tid = self.next_thread_id;
        self.next_thread_id = self.next_thread_id.saturating_add(1);
        self.threads.insert(
            agent_id,
            LegacyThread {
                thread_id: tid,
                agent_id,
                entries: Vec::new(),
                total_joy: 0.0,
                total_mercy: 0.0,
            },
        );
        tid
    }

    pub fn record_entry(
        &mut self,
        agent_id: AgentId,
        event_type: LegacyEventType,
        title: impl Into<String>,
        description: impl Into<String>,
        joy_amount: f32,
        intensity: f32,
        mercy_gain: f32,
        realm_id: u8,
        tick: u64,
    ) -> u64 {
        let thread_id = self.ensure_thread(agent_id);
        let id = self.next_entry_id;
        self.next_entry_id = self.next_entry_id.saturating_add(1);
        self.total_entries = self.total_entries.saturating_add(1);

        let entry = LegacyEntry {
            id,
            agent_id,
            event_type,
            title: title.into(),
            description: description.into(),
            joy_amount,
            intensity,
            mercy_gain,
            realm_id,
            tick,
            thread_id,
        };

        if let Some(thread) = self.threads.get_mut(&agent_id) {
            thread.push(entry);
        }

        id
    }

    pub fn generate_proactive_joy_redemption_thread(
        &mut self,
        player_id: u64,
        reason: String,
        joy_amount: f32,
        intensity: f32,
        current_tick: u64,
        server_id: u64,
    ) {
        let realm_id = (server_id % 256) as u8;
        self.record_entry(
            player_id,
            LegacyEventType::ProactiveJoy,
            "Proactive Joy",
            reason,
            joy_amount,
            intensity,
            joy_amount * 0.15,
            realm_id,
            current_tick,
        );

        info!(
            target: "ra_thor::legacy::joy",
            player_id = player_id,
            joy_amount = joy_amount,
            "Proactive joy redemption thread seeded"
        );
    }

    pub fn record_council_decision(
        &mut self,
        agent_id: AgentId,
        proposal_type: &ProposalType,
        title: &str,
        mercy_factor: f32,
        strength: f32,
        realm_id: u8,
        tick: u64,
        decision_id: u64,
    ) {
        if self.ingested_decision_ids.contains_key(&decision_id) {
            return;
        }
        self.ingested_decision_ids.insert(decision_id, ());

        if self.ingested_decision_ids.len() > 512 {
            self.ingested_decision_ids.clear();
            self.ingested_decision_ids.insert(decision_id, ());
        }

        let event_type = LegacyEventType::from_proposal_type(proposal_type);
        let joy = (strength * 2.2 * mercy_factor).clamp(0.5, 10.0);
        let desc = format!(
            "[Realm {}] Council passed \"{}\" (mercy {:.2}, strength {:.2})",
            realm_id, title, mercy_factor, strength
        );

        self.record_entry(
            agent_id,
            event_type,
            title.to_string(),
            desc,
            joy,
            (0.2 + mercy_factor * 0.4).clamp(0.2, 0.9),
            mercy_factor * strength * 0.5,
            realm_id,
            tick,
        );
    }

    pub fn entries_for_agent(&self, agent_id: AgentId) -> Option<&[LegacyEntry]> {
        self.threads.get(&agent_id).map(|t| t.entries.as_slice())
    }

    pub fn recent_for_agent(&self, agent_id: AgentId, n: usize) -> Vec<&LegacyEntry> {
        match self.threads.get(&agent_id) {
            Some(thread) => {
                let start = thread.entries.len().saturating_sub(n);
                thread.entries[start..].iter().collect()
            }
            None => Vec::new(),
        }
    }
}

#[derive(Event, Clone, Debug)]
pub struct ProactiveJoyTriggered {
    pub agent_id: AgentId,
    pub joy_amount: f32,
    pub reason: String,
}

#[derive(Event, Clone, Debug)]
pub struct JoyBurstSpatialAudioEvent {
    pub position: Vec3,
    pub intensity: f32,
    pub joy_type: String,
}

#[derive(Component, Clone, Debug)]
pub struct JoyEffect {
    pub timer: Timer,
    pub intensity: f32,
    pub mercy_gain: f32,
    pub joy_description: String,
}

#[derive(Component)]
pub struct JoyParticle {
    pub velocity: Vec3,
    pub lifetime: Timer,
}

impl JoyParticle {
    pub fn new(velocity: Vec3, lifetime_secs: f32) -> Self {
        Self {
            velocity,
            lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
        }
    }
}

pub fn spawn_joy_particle_burst(
    commands: &mut Commands,
    position: Vec3,
    intensity: f32,
    count: usize,
    joy_type: &str,
) {
    commands.spawn(JoyBurstSpatialAudioEvent {
        position,
        intensity,
        joy_type: joy_type.to_string(),
    });

    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        let speed = 8.0 + intensity * 12.0;
        let vel = Vec3::new(angle.cos() * speed, 14.0 + intensity * 6.0, angle.sin() * speed);
        let lifetime = 0.6 + intensity * 0.4;

        commands.spawn((
            Transform::from_translation(position + Vec3::new(0.0, 10.0, 0.0)),
            GlobalTransform::default(),
            JoyParticle::new(vel, lifetime),
            Name::new("JoyParticle"),
        ));
    }
}

pub fn joy_effect_feedback_system(
    mut commands: Commands,
    time: Res<Time>,
    mut joy_effects: Query<(Entity, &mut JoyEffect, &Transform)>,
    mut particles: Query<(Entity, &mut Transform, &mut JoyParticle)>,
) {
    for (entity, mut effect, transform) in joy_effects.iter_mut() {
        effect.timer.tick(time.delta());

        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        if effect.timer.elapsed_secs() < 0.05 && effect.timer.elapsed_secs() > 0.0 {
            let joy_type = if effect.joy_description.contains("harvest") {
                "harvest"
            } else if effect.joy_description.contains("epiphany") || effect.joy_description.contains("Epiphany") {
                "epiphany"
            } else if effect.joy_description.contains("council") || effect.joy_description.contains("Council") {
                "council"
            } else if effect.joy_description.contains("synergy") || effect.joy_description.contains("policy") {
                "synergy_policy"
            } else {
                "rbe_abundance"
            };

            spawn_joy_particle_burst(
                &mut commands,
                transform.translation,
                effect.intensity,
                10,
                joy_type,
            );
        }
    }

    for (entity, mut transform, mut particle) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += particle.velocity * time.delta_seconds();
        particle.velocity.y -= 22.0 * time.delta_seconds();
    }
}

pub fn council_history_to_legacy_system(
    decisions: Option<Res<CouncilDecisions>>,
    mut registry: ResMut<LegacyJournalRegistry>,
) {
    let Some(decisions) = decisions else {
        return;
    };

    for decision in decisions.resolved_history.iter() {
        if registry.ingested_decision_ids.contains_key(&decision.decision_id) {
            continue;
        }

        registry.record_council_decision(
            decision.proposer,
            &decision.proposal_type,
            &decision.title,
            decision.mercy_factor,
            decision.strength,
            decision.realm_id,
            decision.created_tick,
            decision.decision_id,
        );
    }
}

pub fn legacy_journal_update_system() {}

pub struct PlayerLegacyJournalPlugin;

impl Plugin for PlayerLegacyJournalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LegacyJournalRegistry>()
            .add_event::<ProactiveJoyTriggered>()
            .add_event::<JoyBurstSpatialAudioEvent>()
            .add_systems(
                Update,
                (
                    council_history_to_legacy_system,
                    legacy_journal_update_system,
                    joy_effect_feedback_system,
                ),
            );

        info!("PlayerLegacyJournalPlugin — registry + council history drain active");
    }
}

// End of v21.69.1 — LegacyJournal restore + Council history event types.
// Thunder locked in. Yoi ⚡
