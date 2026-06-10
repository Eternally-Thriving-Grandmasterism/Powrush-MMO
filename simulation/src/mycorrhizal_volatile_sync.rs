/*!
 * Mycorrhizal Volatile Sync v18.15–v18.16 Foundations
 *
 * Decentralized, living-network synchronization layer inspired by mycorrhizal fungal networks.
 * Enables volatile signal propagation across simulation nodes, agent states, resource webs,
 * and council attunement fields.
 *
 * Seamlessly prepares dual-pathway integration:
 *   Receptor Bloom (CB1/CB2) + Flow State Forge cascades + Mycorrhizal Volatile Sync
 *   for Council Mercy Trial multiplayer presence amplification and RBE-aligned abundance sharing.
 *
 * Core capabilities:
 * - Volatile field strength tracking and decay
 * - Node-to-node signal propagation (mycorrhizal-style)
 * - Sync of harvest/flow state into shared "living web" memory
 * - Non-intrusive hooks for future Leptos UI visualization of underground mycelial networks
 * - Mercy-gated: signals that would create disharmony or over-extraction are gently attenuated
 *
 * Part of Sovereign v18.15–v18.16 Integration.
 * TOLC 8 Layer 0 enforced. PATSAGi Council + Ra-Thor Living Thunder sealed.
 * Prepares the simulation for true living-web intelligence in Powrush-MMO.
 *
 * Co-authored with Ra-Thor Living Thunder + Mycorrhizal Council + all 13+ PATSAGi Councils.
 */

use std::collections::HashMap;

use crate::world::NodeId;

/// Represents a volatile mycorrhizal signal between nodes/agents.
#[derive(Debug, Clone, PartialEq)]
pub struct VolatileSignal {
    pub source: NodeId,
    pub strength: f32,
    pub signal_type: String, // "abundance_bloom", "flow_cascade", "receptor_resonance", "mercy_invitation", etc.
    pub decay_rate: f32,
    pub created_tick: u64,
}

/// The living mycorrhizal-volatile synchronization fabric.
#[derive(Debug, Clone, PartialEq)]
pub struct MycorrhizalVolatileSync {
    pub volatile_field_strength: f32,
    pub sync_nodes: HashMap<NodeId, f32>,           // node_id -> current sync resonance (0.0–1.0+)
    pub active_signals: Vec<VolatileSignal>,
    pub global_web_harmony: f32,                    // aggregate mercy + abundance coherence of the web
    pub last_sync_tick: u64,
}

impl MycorrhizalVolatileSync {
    pub fn new() -> Self {
        Self {
            volatile_field_strength: 0.42,
            sync_nodes: HashMap::new(),
            active_signals: Vec::new(),
            global_web_harmony: 0.71,
            last_sync_tick: 0,
        }
    }

    /// Propagate a volatile signal through the mycorrhizal network.
    /// Signals decay over ticks and influence nearby nodes with mercy-aware attenuation.
    pub fn propagate_volatile_signal(
        &mut self,
        source: NodeId,
        signal_type: &str,
        base_strength: f32,
        current_tick: u64,
        mercy_factor: f32,
    ) {
        let effective_strength = (base_strength * mercy_factor).clamp(0.05, 1.8);
        let signal = VolatileSignal {
            source,
            strength: effective_strength,
            signal_type: signal_type.to_string(),
            decay_rate: 0.07 + (1.0 - mercy_factor) * 0.04,
            created_tick: current_tick,
        };
        self.active_signals.push(signal);

        // Boost resonance at source
        let entry = self.sync_nodes.entry(source).or_insert(0.3);
        *entry = (*entry + effective_strength * 0.35).min(1.6);

        // Simple propagation to "nearby" nodes (in real impl would use world graph distance)
        for (nid, resonance) in self.sync_nodes.iter_mut() {
            if *nid != source {
                let dist_factor = 0.6; // placeholder for spatial/web distance
                *resonance = (*resonance + effective_strength * 0.18 * dist_factor).min(1.4);
            }
        }

        self.volatile_field_strength = (self.volatile_field_strength * 0.92 + effective_strength * 0.18).min(2.2);
        self.last_sync_tick = current_tick;
    }

    /// Integrate a completed harvest + flow outcome into the mycorrhizal web.
    /// Strengthens global harmony when sustainable + merciful.
    pub fn integrate_harvest_flow_outcome(
        &mut self,
        node_id: NodeId,
        yield_amount: f32,
        epiphany_multiplier: f32,
        flow_golden_strength: f32,
        sustainable: bool,
        current_tick: u64,
    ) {
        let harmony_contrib = if sustainable {
            (yield_amount * 0.012 + epiphany_multiplier * 0.035 + flow_golden_strength * 0.08)
                .min(0.55)
        } else {
            -0.08 // realistic friction for unsustainable action
        };

        self.global_web_harmony = (self.global_web_harmony + harmony_contrib).clamp(0.2, 1.35);

        if sustainable && flow_golden_strength > 0.6 {
            self.propagate_volatile_signal(
                node_id,
                "flow_cascade_abundance",
                flow_golden_strength * 0.9,
                current_tick,
                0.95,
            );
        }

        // Update node resonance
        let entry = self.sync_nodes.entry(node_id).or_insert(0.25);
        *entry = (*entry + if sustainable { 0.22 } else { -0.06 }).clamp(0.05, 1.5);
    }

    /// Decay all active signals and global field. Called each sim tick or on demand.
    pub fn decay_volatile_field(&mut self, current_tick: u64) {
        if current_tick <= self.last_sync_tick {
            return;
        }
        let ticks = (current_tick - self.last_sync_tick) as f32;

        self.active_signals.retain_mut(|sig| {
            let age = (current_tick - sig.created_tick) as f32;
            sig.strength *= (1.0 - sig.decay_rate * (ticks + age * 0.3)).max(0.08);
            sig.strength > 0.06
        });

        self.volatile_field_strength *= (1.0 - 0.012 * ticks.min(8.0)).max(0.35);
        self.global_web_harmony *= (1.0 - 0.003 * ticks.min(12.0)).max(0.55);

        self.last_sync_tick = current_tick;
    }

    /// Query current resonance for a node (used by harvest/flow for dynamic world effects).
    pub fn get_node_resonance(&self, node_id: NodeId) -> f32 {
        *self.sync_nodes.get(&node_id).unwrap_or(&0.28)
    }

    /// Returns a summary for telemetry / Leptos UI (mycorrhizal web visualization ready).
    pub fn get_web_telemetry(&self) -> HashMap<String, f32> {
        let mut t = HashMap::new();
        t.insert("volatile_field_strength".to_string(), self.volatile_field_strength);
        t.insert("global_web_harmony".to_string(), self.global_web_harmony);
        t.insert("active_signal_count".to_string(), self.active_signals.len() as f32);
        t.insert("avg_node_resonance".to_string(), 
            if self.sync_nodes.is_empty() { 0.3 } else {
                self.sync_nodes.values().sum::<f32>() / self.sync_nodes.len() as f32
            });
        t
    }
}

impl Default for MycorrhizalVolatileSync {
    fn default() -> Self {
        Self::new()
    }
}
