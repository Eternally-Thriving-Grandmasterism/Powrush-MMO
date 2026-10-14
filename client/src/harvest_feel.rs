/*!
 * Harvest Feel — soft RBE pool + pad rumble (v22.2.0)
 *
 * Take grows vitality. Tend grows harmony. Allocate spends a stack.
 * Rumble is confirmation, never punishment.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use std::time::Duration;

use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};
use bevy::prelude::*;

use crate::lived_hour_bind::LivedHourBind;
use crate::lived_hour_support::RbeGlobalState;
use crate::rbe_allocate_choice::AllocatePath;

#[derive(Resource, Debug)]
pub struct SoftRbePool {
    pub vitality: f32,
    pub harmony: f32,
    pub joy: f32,
    pub last_credit: f32,
    pub harvests: u32,
    pub tends: u32,
    /// 1.0 = first-take camera punch. Decays every frame. Never steals look.
    pub kick: f32,
    pub blooms: u32,
}

impl Default for SoftRbePool {
    fn default() -> Self {
        Self {
            vitality: 0.0,
            harmony: 0.0,
            joy: 0.0,
            last_credit: 0.0,
            harvests: 0,
            tends: 0,
            kick: 0.0,
            blooms: 0,
        }
    }
}

impl SoftRbePool {
    pub fn credit_mercy_harvest(&mut self, node_vitality: f32) -> f32 {
        let amount = (0.85 + node_vitality * 0.35).clamp(0.6, 1.4);
        self.vitality += amount;
        self.harmony += amount * 0.45;
        self.joy += amount * 0.30;
        self.last_credit = amount;
        self.harvests = self.harvests.saturating_add(1);
        amount
    }

    pub fn credit_tend(&mut self, node_vitality: f32) -> f32 {
        let amount = (0.28 + node_vitality * 0.12).clamp(0.22, 0.48);
        self.harmony += amount;
        self.joy += amount * 0.70;
        self.vitality += amount * 0.20;
        self.last_credit = amount;
        self.tends = self.tends.saturating_add(1);
        amount
    }

    pub fn spend_allocate(&mut self, path: AllocatePath, want: f32) -> f32 {
        match path {
            AllocatePath::FlowOutward => {
                let n = want.min(self.vitality.max(0.0));
                self.vitality = (self.vitality - n).max(0.0);
                n
            }
            AllocatePath::StewardReserve => {
                let n = want.min(self.harmony.max(0.0));
                self.harmony = (self.harmony - n).max(0.0);
                n
            }
        }
    }

    pub fn line(&self) -> String {
        format!(
            "Pool  vitality {:.1} · harmony {:.1} · joy {:.1}",
            self.vitality, self.harmony, self.joy
        )
    }

    pub fn punch(&mut self, first: bool) {
        self.kick = if first { 1.0 } else { 0.42 };
        self.blooms = self.blooms.saturating_add(1);
    }

    pub fn tick_juice(&mut self, dt: f32) {
        self.kick = (self.kick - dt * 2.8).max(0.0);
    }
}


pub fn credit_soft_and_global(
    pool: &mut SoftRbePool,
    global: Option<&mut RbeGlobalState>,
    node_vitality: f32,
) -> f32 {
    let amount = pool.credit_mercy_harvest(node_vitality);
    if let Some(g) = global {
        g.total_abundance += amount;
        g.global_harmony_score = (g.global_harmony_score + 0.015).min(1.0);
    }
    amount
}

pub fn rumble_mercy_harvest(
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
    gamepads: impl IntoIterator<Item = Entity>,
) {
    rumble_harvest(rumble, gamepads, false);
}

pub fn rumble_harvest(
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
    gamepads: impl IntoIterator<Item = Entity>,
    first: bool,
) {
    let (strong, weak, ms) = if first {
        (0.28, 0.52, 160)
    } else {
        (0.12, 0.28, 90)
    };
    for gamepad in gamepads {
        rumble.write(GamepadRumbleRequest::Add {
            gamepad,
            intensity: GamepadRumbleIntensity {
                strong_motor: strong,
                weak_motor: weak,
            },
            duration: Duration::from_millis(ms),
        });
    }
}

/// After a successful first-hour E juice path.
/// A harvest bump leaves the lived-hour satchel and node state alone.
/// Camera punch, rumble, and glow stay put.
pub fn note_lived_hour_take(bind: Option<&mut LivedHourBind>) {
    let _ = bind;
}

fn tick_harvest_juice(time: Res<Time>, mut pool: ResMut<SoftRbePool>) {
    pool.tick_juice(time.delta_secs());
}

fn sync_lived_hour_take(
    pool: Res<SoftRbePool>,
    mut bind: ResMut<LivedHourBind>,
    mut last: Local<u32>,
) {
    if pool.harvests <= *last {
        return;
    }
    *last = pool.harvests;
    note_lived_hour_take(Some(&mut bind));
}

pub struct HarvestFeelPlugin;

impl Plugin for HarvestFeelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftRbePool>().add_systems(
            Update,
            (tick_harvest_juice, sync_lived_hour_take),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harvest_grows_pool_without_emptying() {
        let mut pool = SoftRbePool::default();
        let a = pool.credit_mercy_harvest(1.0);
        let b = pool.credit_mercy_harvest(0.5);
        assert!(a > 0.0 && b > 0.0);
        assert_eq!(pool.harvests, 2);
        assert!(pool.vitality > a);
    }

    #[test]
    fn tend_grows_harmony_more_than_vitality() {
        let mut pool = SoftRbePool::default();
        pool.credit_tend(1.0);
        assert!(pool.harmony > pool.vitality);
        assert_eq!(pool.tends, 1);
    }

    #[test]
    fn allocate_spends_the_named_stack() {
        let mut pool = SoftRbePool::default();
        pool.vitality = 3.0;
        pool.harmony = 2.0;
        let spent = pool.spend_allocate(AllocatePath::FlowOutward, 1.0);
        assert_eq!(spent, 1.0);
        assert!((pool.vitality - 2.0).abs() < 0.01);
    }

    #[test]
    fn first_punch_is_stronger_than_repeat() {
        let mut pool = SoftRbePool::default();
        pool.punch(true);
        let first = pool.kick;
        pool.punch(false);
        assert!(first > pool.kick);
        assert_eq!(pool.blooms, 2);
        pool.tick_juice(0.5);
        assert!(pool.kick < 0.42);
    }

    #[test]
    fn lived_hour_hook_is_optional() {
        note_lived_hour_take(None);
    }

    /// CARD TAKE-TENDS-ONCE-1 — a harvest bump leaves satchel and node state unchanged.
    #[test]
    fn harvest_bump_leaves_satchel_and_node_state_unchanged() {
        use shared::climate_node::NodeState;

        let _guard = user_dir_guard("harvest-bump");
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(glow_bind())
            .init_resource::<SoftRbePool>()
            .add_systems(Update, sync_lived_hour_take);

        let bind = app.world().resource::<LivedHourBind>();
        let satchel = bind.satchel_count();
        let states = node_states(bind);
        assert_eq!(satchel, 0);
        assert_eq!(states[0], NodeState::Glowing);

        app.world_mut()
            .resource_mut::<SoftRbePool>()
            .credit_mercy_harvest(1.0);
        assert_eq!(app.world().resource::<SoftRbePool>().harvests, 1);
        app.update();

        let bind = app.world().resource::<LivedHourBind>();
        assert_eq!(bind.satchel_count(), satchel);
        assert_eq!(node_states(bind), states);

        let mut bind = app.world_mut().resource_mut::<LivedHourBind>();
        note_lived_hour_take(Some(&mut bind));
        assert_eq!(bind.satchel_count(), satchel);
        assert_eq!(node_states(&bind), states);
    }

    /// CARD TAKE-TENDS-ONCE-1 — one glowing tend still lands through the mercy bridge.
    #[test]
    fn glowing_tend_through_mercy_bridge_gives_satchel_and_tended() {
        use crate::lived_sim_bridge::{sync_lived_hour_use, LivedSimBridge};
        use shared::climate_node::NodeState;

        let _guard = user_dir_guard("glow-tend");
        let mut bind = glow_bind();
        assert_eq!(bind.hour.nodes[0].state, NodeState::Glowing);
        assert_eq!(bind.satchel_count(), 0);

        let mut bridge = LivedSimBridge::default();
        bridge.session_id = 493_0001;
        sync_lived_hour_use(Some(&mut bind), Some(&bridge), 1.0, "tend", Some(0.0));

        assert_eq!(bind.satchel_count(), 1);
        assert_eq!(bind.hour.nodes[0].state, NodeState::Tended);
        assert_eq!(
            bind.hour
                .nodes
                .iter()
                .filter(|n| n.state == NodeState::Tended)
                .count(),
            1
        );
    }

    /// CARD TAKE-TENDS-ONCE-1 — juice kick still decays. `tick_juice` stays.
    #[test]
    fn kick_still_decays() {
        let mut pool = SoftRbePool::default();
        pool.punch(true);
        assert_eq!(pool.kick, 1.0);
        pool.tick_juice(0.1);
        assert!(pool.kick < 1.0);
        assert!(pool.kick > 0.0);
        pool.tick_juice(10.0);
        assert_eq!(pool.kick, 0.0);
    }

    fn glow_bind() -> LivedHourBind {
        use shared::climate_node::LivedHour;
        use shared::shard_climate::ShardClimate;
        use shared::shard_standing::ShardStanding;
        use shared::week_audit::WeekAudit;

        LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: Some(1),
            climate_slab: None,
        }
    }

    fn node_states(bind: &LivedHourBind) -> Vec<shared::climate_node::NodeState> {
        bind.hour.nodes.iter().map(|n| n.state).collect()
    }

    struct UserDirGuard {
        prev: Option<String>,
        _lock: crate::test_env::UserDirEnvGuard,
    }

    fn user_dir_guard(tag: &str) -> UserDirGuard {
        let lock = crate::test_env::lock();
        let dir = std::env::temp_dir().join(format!(
            "powrush-take-tends-once-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).expect("temp user dir");
        let prev = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);
        UserDirGuard { prev, _lock: lock }
    }

    impl Drop for UserDirGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => {
                    std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value)
                }
                None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV),
            }
        }
    }
}
