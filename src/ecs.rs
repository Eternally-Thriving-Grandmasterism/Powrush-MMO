use bevy::prelude::*;

// Group hot components for cache efficiency
#[derive(Bundle)]
pub struct HotBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub velocity: Velocity,
    pub mercy_points: MercyPoints,
    pub trust_credits: TrustCredits,
}

// Archetype optimization — all frequently accessed together
pub struct ECSPlugin;

impl Plugin for ECSPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HotBundle>()
           .add_systems(Update, hot_component_system);
    }
}

fn hot_component_system(
    mut query: Query<&mut HotBundle>,
) {
    // All hot data in one archetype — cache-friendly
    for mut bundle in &mut query {
        bundle.velocity.0 += Vec3::Y * 0.1;  // Example
    }
}
