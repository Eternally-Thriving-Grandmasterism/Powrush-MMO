use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LatticeStats {
    pub nodes: u32,
    pub connections: u32,
    pub mercy_flow: f32,
}

#[derive(Component, Replicated)]
pub struct TrustCredits(pub f32);

pub struct MercyPlugin;

impl Plugin for MercyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LatticeStats::default())
           .add_systems(Update, (mercy_flow_system, trust_growth_system, lattice_expansion_system));
    }
}

fn mercy_flow_system(
    time: Res<Time>,
    mut lattice: ResMut<LatticeStats>,
) {
    lattice.mercy_flow += time.delta_seconds() * 0.5;
}

fn trust_growth_system(
    mut query: Query<&mut TrustCredits>,
    time: Res<Time>,
) {
    for mut trust in &mut query {
        trust.0 += time.delta_seconds() * 0.1;
    }
}

fn lattice_expansion_system(
    mut lattice: ResMut<LatticeStats>,
    time: Res<Time>,
) {
    if rand::thread_rng().gen_bool(0.05 * time.delta_seconds() as f64) {
        lattice.nodes += 1;
        lattice.connections += 2;
    }
}
