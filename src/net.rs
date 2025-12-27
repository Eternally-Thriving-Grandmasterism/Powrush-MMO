use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct MMONetPlugin;

impl Plugin for MMONetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins)
            .replicate::<MercyPoints>()
            .replicate::<TrustCredits>()
            .replicate::<Inventory>()
            .replicate::<Quest>()
            .replicate::<CombatPlayer>()
            .replicate::<Boss>();
    }
}
