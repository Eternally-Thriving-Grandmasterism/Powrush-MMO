use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct MMONetPlugin;

impl Plugin for MMONetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins.set(ServerPlugin {
            tick_policy: TickPolicy::MaxTickRate(60),
            visibility_policy: VisibilityPolicy::InterestManagement,
            ..default()
        }))
        .add_plugins(RepliconPlugins.set(ClientPlugin {
            tick_policy: TickPolicy::EveryFrame,
            ..default()
        }))
        .replicate::<MercyPoints>()
        .replicate::<TrustCredits>()
        .replicate::<Inventory>()
        .replicate::<Quest>()
        .replicate::<CombatPlayer>()
        .replicate::<Boss>();
    }
}
