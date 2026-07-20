/*!
 * Hybrid CPU + GPU Economic / RBE Layer
 * v21.66.0 — Bevy EconomyState + Multi-Realm RBE Sustainability Snapshot
 *
 * RBE Council Policy Integration:
 * - Council attunement and bloom outcomes influence RBE pools, nodes, abundance, pressure, sustainability.
 * - High mercy/attunement = economic blessing; low = friction.
 *
 * v21.66: Bevy-facing EconomyState + organism-level RBE snapshot from multi-realm observatory.
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::world::SovereignWorldState;
use crate::mercy::{MercyGate, MercyViolation};
use crate::harvest::HarvestEvent;
use crate::emergence::DynamicEmergenceEvent;
use crate::multi_realm_harness::RealmAbundanceObservatory;
use tracing::{info_span, instrument, warn};

#[cfg(feature = "gpu")]
use crate::gpu_economic::{dispatch_gpu_economic_update, dispatch_gpu_economic_compute_async, GpuEconomicReadback};

#[cfg(feature = "gpu")]
use crate::engine::gpu_patsagi_bridge::GpuPatsagiResponse;

pub struct EconomicLayer {
    pub cpu_precision_mode: bool,
}

impl EconomicLayer {
    pub fn new() -> Self {
        Self { cpu_precision_mode: true }
    }

    #[instrument(skip(self, world, mercy_gate))]
    pub fn batch_update(
        &self,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        let _span = info_span!("economic_batch_update").entered();

        mercy_gate.pre_economic_tick_validate(world)?;

        if self.cpu_precision_mode {
            self.cpu_economic_update(world)?;
        } else {
            #[cfg(feature = "gpu")]
            {
                if let Err(e) = dispatch_gpu_economic_update(world) {
                    warn!("GPU dispatch failed ({}). Falling back to CPU precision path for this tick.", e);
                    self.cpu_economic_update(world)?;
                }
            }
            #[cfg(not(feature = "gpu"))]
            {
                self.cpu_economic_update(world)?;
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }

    fn cpu_economic_update(&self, world: &mut SovereignWorldState) -> Result<(), MercyViolation> {
        let _span = info_span!("cpu_economic_update").entered();
        let now_ms = world.sim_time;

        for node in world.resource_nodes.values_mut() {
            if node.depletion > 0.0 {
                node.depletion = (node.depletion - node.regen_rate).max(0.0);
                node.current_yield = node.base_yield * (1.0 - node.depletion * 0.7);
            }
            node.sustainability_score = (1.0 - node.depletion * 0.5).max(0.3);

            if node.depletion < 0.3 {
                node.stress_level = (node.stress_level - 0.02).max(0.0);
            }

            if node.harvest_restricted_until_ms > 0 && now_ms > node.harvest_restricted_until_ms {
                node.harvest_restricted_until_ms = 0;
                node.stress_level = (node.stress_level * 0.5).max(0.0);
            }

            if node.abundance_flow > 0.2 {
                let bonus = 1.0 + (node.abundance_flow - 0.2) * 1.8;
                node.regen_rate = (node.regen_rate * bonus).min(3.5);
                node.sustainability_score = (node.sustainability_score + node.abundance_flow * 0.12).min(1.0);
            } else if node.abundance_flow < -0.15 {
                node.stress_level = (node.stress_level + 0.28).min(1.0);
                if node.stress_level > 0.75 {
                    node.harvest_restricted_until_ms = now_ms + 90_000;
                }
            }
        }

        for pool in world.rbe_pools.values_mut() {
            pool.abundance_flow = (pool.abundance_flow * 0.98 + 0.02).clamp(0.0, 2.0);
            pool.sustainability_score = (pool.sustainability_score * 0.995 + pool.abundance_flow * 0.005).clamp(0.3, 1.0);
            pool.pressure = (pool.pressure * 0.9).max(0.0);
        }

        self.apply_rbe_sustainability_tick(world);

        Ok(())
    }

    pub fn apply_rbe_sustainability_tick(&self, world: &mut SovereignWorldState) {
        for node in world.resource_nodes.values_mut() {
            if node.depletion > 0.4 || node.stress_level > 0.5 {
                node.pressure = (node.pressure + 0.015).min(5.0);
            } else if node.depletion < 0.15 && node.stress_level < 0.3 {
                node.pressure = (node.pressure - 0.008).max(0.0);
            }

            if node.pressure > 1.5 {
                let decay = 0.002 * (node.pressure - 1.0);
                node.sustainability_score = (node.sustainability_score - decay).max(0.1);
            }

            if node.pressure > 3.0 {
                node.regen_rate = (node.regen_rate * 0.985).max(0.3);
                if node.pressure > 4.0 && node.harvest_restricted_until_ms == 0 {
                    node.harvest_restricted_until_ms = world.sim_time + 60_000;
                }
            }
        }

        for pool in world.rbe_pools.values_mut() {
            if pool.pressure > 2.0 || pool.sustainability_score < 0.4 {
                pool.pressure = (pool.pressure + 0.01).min(8.0);
            } else {
                pool.pressure = (pool.pressure - 0.005).max(0.0);
            }

            if pool.pressure > 3.5 {
                let decay = 0.0015 * (pool.pressure - 2.0);
                pool.sustainability_score = (pool.sustainability_score - decay).max(0.15);
            }

            if pool.pressure > 4.5 {
                pool.abundance_flow = (pool.abundance_flow * 0.97).max(-1.0);
            }
        }
    }

    pub fn apply_council_policy_impact(
        &mut self,
        collective_attunement: f32,
        bloom_success: bool,
        participant_count: u8,
        world: &mut SovereignWorldState,
    ) {
        let mercy_factor = collective_attunement.clamp(0.0, 1.0);
        let is_strong_council = bloom_success && mercy_factor > 0.65 && participant_count >= 3;

        for pool in world.rbe_pools.values_mut() {
            if is_strong_council {
                pool.abundance_flow = (pool.abundance_flow + mercy_factor * 0.8).min(4.0);
                pool.sustainability_score = (pool.sustainability_score + mercy_factor * 0.06).min(1.0);
                pool.pressure = (pool.pressure - mercy_factor * 1.2).max(0.0);
            } else if mercy_factor < 0.4 {
                pool.pressure = (pool.pressure + (1.0 - mercy_factor) * 0.9).min(8.0);
                pool.abundance_flow = (pool.abundance_flow - (1.0 - mercy_factor) * 0.35).max(-2.0);
                pool.sustainability_score = (pool.sustainability_score - 0.015).max(0.1);
            } else {
                pool.abundance_flow = (pool.abundance_flow + mercy_factor * 0.25).min(3.0);
                pool.pressure = (pool.pressure - mercy_factor * 0.4).max(0.0);
            }
        }

        for node in world.resource_nodes.values_mut() {
            if is_strong_council {
                node.abundance_flow = (node.abundance_flow + mercy_factor * 0.6).min(3.5);
                node.sustainability_score = (node.sustainability_score + mercy_factor * 0.05).min(1.0);
                node.pressure = (node.pressure - mercy_factor * 0.8).max(0.0);
                node.regen_rate = (node.regen_rate * (1.0 + mercy_factor * 0.3)).min(4.0);
            } else if mercy_factor < 0.4 {
                node.pressure = (node.pressure + (1.0 - mercy_factor) * 0.7).min(5.0);
                node.abundance_flow = (node.abundance_flow - (1.0 - mercy_factor) * 0.25).max(-1.5);
            }
        }
    }

    pub fn apply_harvest_event(
        &mut self,
        event: &HarvestEvent,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        mercy_gate.pre_economic_tick_validate(world)?;

        if let Some(node) = world.resource_nodes.get_mut(&event.node_id) {
            if event.sustainable {
                node.sustainability_score = (node.sustainability_score + 0.08).min(1.0);
                node.abundance_flow = (node.abundance_flow + 0.05).min(2.5);
                node.pressure = (node.pressure - 0.3).max(0.0);
            } else {
                node.stress_level = (node.stress_level + 0.15).min(1.0);
                node.abundance_flow = (node.abundance_flow - 0.08).max(-1.5);
                node.pressure = (node.pressure + 0.6).min(5.0);
            }

            for pool in world.rbe_pools.values_mut() {
                if event.council_amplified {
                    pool.abundance_flow = (pool.abundance_flow + 0.12).min(3.0);
                    pool.sustainability_score = (pool.sustainability_score + 0.04).min(1.0);
                } else {
                    pool.abundance_flow = (pool.abundance_flow + 0.03).min(2.5);
                }
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }

    pub fn apply_emergence_event(
        &mut self,
        event: &DynamicEmergenceEvent,
        world: &mut SovereignWorldState,
        mercy_gate: &MercyGate,
    ) -> Result<(), MercyViolation> {
        mercy_gate.pre_economic_tick_validate(world)?;

        for effect in &event.proposed_effects {
            match effect {
                crate::emergence::EmergenceEffect::ResourceDelta { resource: _, amount, is_abundance } => {
                    for pool in world.rbe_pools.values_mut() {
                        if *is_abundance {
                            pool.abundance_flow = (pool.abundance_flow + amount * 0.5).min(3.5);
                            pool.sustainability_score = (pool.sustainability_score + 0.03).min(1.0);
                            pool.pressure = (pool.pressure - 0.2).max(0.0);
                        } else {
                            pool.pressure = (pool.pressure + amount * 0.3).min(2.0);
                        }
                    }
                }
                crate::emergence::EmergenceEffect::BiomeResonance { intensity } => {
                    for node in world.resource_nodes.values_mut() {
                        node.abundance_flow = (node.abundance_flow + intensity * 0.2).min(3.0);
                        node.sustainability_score = (node.sustainability_score + intensity * 0.02).min(1.0);
                        node.pressure = (node.pressure - intensity * 0.15).max(0.0);
                    }
                }
                crate::emergence::EmergenceEffect::TemporaryMultiplier { multiplier, .. } => {
                    for pool in world.rbe_pools.values_mut() {
                        pool.abundance_flow = (pool.abundance_flow * multiplier).clamp(0.0, 3.5);
                    }
                }
                _ => {}
            }
        }

        mercy_gate.post_economic_tick_validate(world)?;
        Ok(())
    }

    #[cfg(feature = "gpu")]
    pub fn apply_gpu_regen_adjustments(&self, response: &GpuPatsagiResponse, world: &mut SovereignWorldState) -> bool {
        let mut applied = false;

        for (&node_id, &recommended_regen) in &response.recommended_regen_rates {
            if let Some(node) = world.resource_nodes.get_mut(&node_id) {
                let old_regen = node.regen_rate;
                node.regen_rate = recommended_regen.clamp(0.001, 2.0);
                node.regen_rate = (old_regen * 0.7 + node.regen_rate * 0.3).clamp(0.001, 2.0);
                applied = true;
            }
        }

        for (&node_id, &sustainability) in &response.sustainability_adjustments {
            if let Some(node) = world.resource_nodes.get_mut(&node_id) {
                node.sustainability_score = (node.sustainability_score * 0.6 + sustainability * 0.4)
                    .clamp(0.1, 1.0);
                applied = true;
            }
        }

        if !response.predicted_depletion.is_empty() {
            applied = true;
        }

        applied
    }
}

// ============================================================================
// BEVY-FACING ECONOMY STATE (v21.66)
// ============================================================================

/// Player / organism RBE economy state used by Bevy systems (Kardashev, UI, progression).
#[derive(Component, Resource, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct EconomyState {
    pub total_harvested: f32,
    pub cooperative_bonus: f32,
    pub average_sustainability: f32,
    pub average_pressure: f32,
    pub abundance_velocity: f32,
    pub sustainable_harvests: u32,
    pub stressed_harvests: u32,
}

impl Default for EconomyState {
    fn default() -> Self {
        Self {
            total_harvested: 0.0,
            cooperative_bonus: 0.0,
            average_sustainability: 0.75,
            average_pressure: 0.0,
            abundance_velocity: 0.0,
            sustainable_harvests: 0,
            stressed_harvests: 0,
        }
    }
}

impl EconomyState {
    pub fn record_harvest(&mut self, amount: f32, sustainable: bool, cooperative: bool) {
        self.total_harvested += amount.max(0.0);
        if sustainable {
            self.sustainable_harvests = self.sustainable_harvests.saturating_add(1);
            self.average_sustainability =
                (self.average_sustainability * 0.95 + 0.05).min(1.0);
            self.average_pressure = (self.average_pressure - 0.02).max(0.0);
        } else {
            self.stressed_harvests = self.stressed_harvests.saturating_add(1);
            self.average_sustainability =
                (self.average_sustainability * 0.97 - 0.01).max(0.1);
            self.average_pressure = (self.average_pressure + 0.08).min(5.0);
        }
        if cooperative {
            self.cooperative_bonus += amount * 0.15;
        }
        self.abundance_velocity =
            (self.abundance_velocity * 0.9 + amount * 0.1).min(50.0);
    }

    pub fn health_label(&self) -> &'static str {
        if self.average_sustainability > 0.8 && self.average_pressure < 0.5 {
            "Thriving"
        } else if self.average_pressure > 2.5 || self.average_sustainability < 0.4 {
            "Stressed"
        } else if self.abundance_velocity > 5.0 {
            "Abundant"
        } else {
            "Steady"
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResourceTransaction {
    pub resource_id: u32,
    pub amount: f32,
    pub from_entity: Option<Entity>,
    pub to_entity: Option<Entity>,
    pub sustainable: bool,
    pub tick: u64,
}

/// Soft post-scarcity allocator — distributes surplus toward under-supplied agents.
#[derive(Resource, Clone, Debug, Default)]
pub struct PostScarcityAllocator {
    pub surplus_pool: f32,
    pub allocations_this_tick: u32,
    pub last_tick: u64,
}

impl PostScarcityAllocator {
    pub fn deposit_surplus(&mut self, amount: f32) {
        self.surplus_pool = (self.surplus_pool + amount.max(0.0)).min(10_000.0);
    }

    pub fn allocate(&mut self, need: f32) -> f32 {
        let grant = need.min(self.surplus_pool).max(0.0);
        self.surplus_pool -= grant;
        if grant > 0.0 {
            self.allocations_this_tick = self.allocations_this_tick.saturating_add(1);
        }
        grant
    }

    pub fn tick_reset(&mut self, tick: u64) {
        self.allocations_this_tick = 0;
        self.last_tick = tick;
    }
}

// ============================================================================
// MULTI-REALM → RBE ORGANISM SNAPSHOT (v21.66)
// ============================================================================

/// Organism-level RBE health derived from RealmAbundanceObservatory.
#[derive(Resource, Clone, Debug, Default)]
pub struct MultiRealmRbeSnapshot {
    pub realm_count: u32,
    pub avg_sustainability: f32,
    pub avg_stress: f32,
    pub avg_flow: f32,
    pub total_yield: f32,
    pub thriving_ratio: f32,
    pub restricted_nodes: u32,
    pub health_label: &'static str,
    pub last_tick: u64,
}

impl MultiRealmRbeSnapshot {
    pub fn compute_label(sust: f32, stress: f32, flow: f32, thriving_ratio: f32) -> &'static str {
        if thriving_ratio > 0.6 && sust > 0.75 && stress < 0.3 {
            "Organism Thriving"
        } else if stress > 0.55 || sust < 0.45 {
            "Organism Stressed"
        } else if flow > 0.15 {
            "Organism Abundant"
        } else {
            "Organism Steady"
        }
    }
}

/// Soft system: aggregates multi-realm abundance observatory into organism RBE snapshot.
pub fn multi_realm_rbe_snapshot_system(
    observatory: Option<Res<RealmAbundanceObservatory>>,
    mut snapshot: ResMut<MultiRealmRbeSnapshot>,
    mut economy: Option<ResMut<EconomyState>>,
) {
    let Some(obs) = observatory else {
        return;
    };
    if obs.views.is_empty() {
        return;
    }

    let n = obs.views.len() as f32;
    let mut sust = 0.0;
    let mut stress = 0.0;
    let mut flow = 0.0;
    let mut yield_sum = 0.0;
    let mut thriving = 0u32;
    let mut restricted = 0u32;

    for view in obs.views.values() {
        sust += view.average_sustainability;
        stress += view.average_stress;
        flow += view.average_abundance_flow;
        yield_sum += view.total_current_yield;
        if view.is_thriving() {
            thriving += 1;
        }
        restricted += view.restricted_node_count;
    }

    let avg_s = sust / n;
    let avg_st = stress / n;
    let avg_f = flow / n;
    let thr_ratio = thriving as f32 / n;

    snapshot.realm_count = obs.views.len() as u32;
    snapshot.avg_sustainability = avg_s;
    snapshot.avg_stress = avg_st;
    snapshot.avg_flow = avg_f;
    snapshot.total_yield = yield_sum;
    snapshot.thriving_ratio = thr_ratio;
    snapshot.restricted_nodes = restricted;
    snapshot.health_label = MultiRealmRbeSnapshot::compute_label(avg_s, avg_st, avg_f, thr_ratio);
    snapshot.last_tick = obs.last_updated_tick;

    // Soft-feed organism EconomyState if present as resource
    if let Some(mut eco) = economy {
        eco.average_sustainability =
            (eco.average_sustainability * 0.85 + avg_s * 0.15).clamp(0.1, 1.0);
        eco.average_pressure =
            (eco.average_pressure * 0.85 + avg_st * 2.0 * 0.15).clamp(0.0, 5.0);
        eco.abundance_velocity =
            (eco.abundance_velocity * 0.9 + avg_f.max(0.0) * 10.0 * 0.1).min(50.0);
    }
}

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EconomyState>()
            .init_resource::<PostScarcityAllocator>()
            .init_resource::<MultiRealmRbeSnapshot>()
            .register_type::<EconomyState>()
            .add_systems(Update, multi_realm_rbe_snapshot_system);
    }
}

// End of v21.66 — Bevy EconomyState + Multi-Realm RBE Sustainability Snapshot.
// Thunder locked in. Yoi ⚡
