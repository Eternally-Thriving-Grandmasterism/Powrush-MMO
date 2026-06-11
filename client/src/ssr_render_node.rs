/*!
 * Example of per-entity previous transform storage for velocity prepass.
 */

use bevy::prelude::*;

/// Component to store the previous frame's GlobalTransform
#[derive(Component, Default)]
pub struct PreviousGlobalTransform(pub GlobalTransform);

/// System that copies current GlobalTransform into PreviousGlobalTransform every frame.
/// Add this system to your main schedule (e.g. Update).
pub fn store_previous_global_transforms(
    mut query: Query<(&GlobalTransform, &mut PreviousGlobalTransform)>,
) {
    for (current, mut previous) in &mut query {
        previous.0 = *current;
    }
}

// Usage:
// 1. Add PreviousGlobalTransform component to entities that need velocity.
// 2. In VelocityPrepassNode, query for entities with GlobalTransform + PreviousGlobalTransform
//    and use previous.0 to compute velocity.
// 3. Pass per-entity previous model matrix into the velocity shader.
