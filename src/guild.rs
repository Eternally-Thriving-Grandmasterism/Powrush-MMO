use bevy::prelude::*;

#[derive(Component)]
pub struct GuildTrust(pub f32);

pub fn guild_trust_multiplier(
    mut query: Query<&mut GuildTrust>,
) {
    for mut trust in &mut query {
        trust.0 *= 1.1;  // Exponential thriving
    }
}

pub fn lattice_expansion_on_contrib(
    mut events: EventReader<MercyAllocEvent>,
    mut stats: ResMut<LatticeStats>,
) {
    for _ in events.read() {
        stats.nodes += 1;
        info!("Lattice expanded — new node born");
    }
}
