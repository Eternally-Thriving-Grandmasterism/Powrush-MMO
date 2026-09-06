//! T-offline — shard climate time-lapse (v23.2.41)
//!
//! Same ledgers tick without a second human. Not on the Bevy boot card.
//! Harness / tests only. Contact: info@Rathor.ai

use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::week_audit::WeekAudit;

/// Offline tick count for a parked time-lapse.
#[derive(Debug, Clone, Default)]
pub struct ShardSim {
    pub ticks: u32,
}

impl ShardSim {
    /// One soft hour tick: regen eases stress; peace follows harmony.
    pub fn tick(
        &mut self,
        climate: &mut ShardClimate,
        standing: &mut ShardStanding,
        week: &mut WeekAudit,
    ) {
        self.ticks = self.ticks.saturating_add(1);
        let regen = climate.regen;
        climate.stress = (climate.stress - regen * 0.15).clamp(0.0, 1.0);
        climate.harmony = (climate.harmony + regen * 0.08).clamp(0.0, 1.0);
        climate.updated_at = climate.updated_at.saturating_add(1);

        standing.peace = (standing.peace + climate.harmony * 0.02 - climate.stress * 0.02)
            .clamp(0.0, 1.0);
        standing.harmony = climate.harmony;
        standing.updated_at = standing.updated_at.saturating_add(1);
        standing.clamp_fields();

        week.sync_from_climate(climate.tons_moved, climate.restored_count);
    }

    /// Run N ticks. Returns final stress for callers/tests.
    pub fn lapse(
        &mut self,
        climate: &mut ShardClimate,
        standing: &mut ShardStanding,
        week: &mut WeekAudit,
        n: u32,
    ) -> f32 {
        for _ in 0..n {
            self.tick(climate, standing, week);
        }
        climate.stress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_ticks_ease_stress() {
        let mut sim = ShardSim::default();
        let mut climate = ShardClimate {
            stress: 0.8,
            regen: 0.2,
            ..Default::default()
        };
        let mut standing = ShardStanding::default();
        let mut week = WeekAudit::default();
        let before = climate.stress;
        let after = sim.lapse(&mut climate, &mut standing, &mut week, 8);
        assert!(after < before);
        assert_eq!(sim.ticks, 8);
        assert!(!standing.declared_lethal);
    }

    #[test]
    fn not_a_boot_card_surface() {
        // Documented contract: ShardSim is harness-only.
        assert_eq!(std::any::type_name::<ShardSim>().contains("shard_sim"), true);
    }
}
