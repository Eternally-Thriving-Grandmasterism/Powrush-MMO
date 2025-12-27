use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub struct MercyNetPlugin;

impl Plugin for MercyNetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconPlugins)
            .replicate::<MercyPoints>()
            .replicate::<TrustCredits>()
            .replicate::<LatticeNode>()
            .replicate::<Inventory>()
            .replicate::<Item>();
    }
}
