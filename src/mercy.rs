use bevy::prelude::*;

#[derive(Component)]
pub struct MercyPoints(pub f32);

#[derive(Component)]
pub struct Need(pub f32);

pub fn mercy_flow_system(
    mut query: Query<(&Need, &mut MercyPoints)>,
    time: Res<Time>,
) {
    for (need, mut mp) in &mut query {
        let alloc = (need.0 * time.delta_seconds()).min(mp.0);
        mp.0 -= alloc;
        info!("Mercy allocated: {:.2}", alloc);
    }
}
