/*!
 * Living Freshness — v22.9.0
 *
 * ARK spoilage teaches order-of-operations. Here the unused vitality
 * composts back into the web and a nearby node. No health drain.
 *
 * Climate: Depths keep it (0.55×). Horizon lets it go (1.35×).
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::human_inventory::HumanInventory;
use crate::living_ecology::PersistentWeb;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::mercy_harvest_nodes::{apply_node_tend, MercyHarvestNode, NearbyMercyNode};

const AGE_BEFORE_RETURN: f32 = 48.0;
const RETURN_CHUNK: f32 = 0.35;

#[derive(Resource, Debug)]
pub struct LivingFreshness {
    pub age: f32,
    pub returns: u32,
    pub first_return_lived: bool,
}

impl Default for LivingFreshness {
    fn default() -> Self {
        Self {
            age: 0.0,
            returns: 0,
            first_return_lived: false,
        }
    }
}

pub struct LivingFreshnessPlugin;

impl Plugin for LivingFreshnessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingFreshness>()
            .add_systems(Update, compost_unused_vitality);
    }
}

fn climate_mul(realm: Option<u8>) -> f32 {
    match realm {
        Some(3) => 0.55,
        Some(4) | Some(1) => 1.35,
        Some(2) => 0.85,
        _ => 1.0,
    }
}

fn compost_unused_vitality(
    time: Res<Time>,
    realm: Res<SoftPlayerRealm>,
    nearby: Res<NearbyMercyNode>,
    mut fresh: ResMut<LivingFreshness>,
    mut pool: ResMut<SoftRbePool>,
    mut web: ResMut<PersistentWeb>,
    mut inv: ResMut<HumanInventory>,
    mut nodes: Query<&mut MercyHarvestNode>,
) {
    if pool.vitality < 0.45 {
        fresh.age = (fresh.age - time.delta_seconds() * 0.4).max(0.0);
        return;
    }
    fresh.age += time.delta_seconds() * climate_mul(realm.current);
    if fresh.age < AGE_BEFORE_RETURN {
        return;
    }
    fresh.age = 0.0;
    let chunk = pool.vitality.min(RETURN_CHUNK);
    if chunk <= 0.0 {
        return;
    }
    pool.vitality -= chunk;
    web.thread_strength = (web.thread_strength + chunk * 0.08).min(1.0);
    if let Some(entity) = nearby.entity {
        if let Ok(mut node) = nodes.get_mut(entity) {
            apply_node_tend(&mut node);
        }
    }
    fresh.returns = fresh.returns.saturating_add(1);
    inv.pickup_until = time.elapsed_seconds_f64() + 2.4;
    inv.pickup_line = "vitality went home to the grove".into();
    if !fresh.first_return_lived {
        fresh.first_return_lived = true;
        info!(target: "powrush::freshness", "first compost — carry wants to move");
    }
}
