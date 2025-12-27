use bevy::prelude::*;
use rand::Rng;

#[derive(Event)]
pub struct WorldMercyEvent {
    pub kind: WorldEventKind,
    pub duration: Timer,
}

#[derive(Clone, Copy)]
pub enum WorldEventKind {
    MercyRain,     // Global trust +20%
    LatticeBloom,  // +50 nodes
    ShareWave,     // All trades *1.5
}

pub struct WorldEventsPlugin;

impl Plugin for WorldEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldMercyEvent>()
           .add_systems(Update, (spawn_world_event_system, world_event_effect_system));
    }
}

fn spawn_world_event_system(
    mut events: EventWriter<WorldMercyEvent>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.001 * time.delta_seconds()) {
        let kind = match rng.gen_range(0..3) {
            0 => WorldEventKind::MercyRain,
            1 => WorldEventKind::LatticeBloom,
            _ => WorldEventKind::ShareWave,
        };
        events.send(WorldMercyEvent {
            kind,
            duration: Timer::from_seconds(300.0, TimerMode::Once),
        });
        info!("World event — {:?}", kind);
    }
}

fn world_event_effect_system(
    mut events: EventReader<WorldMercyEvent>,
    mut trust: Query<&mut TrustCredits>,
    mut lattice: ResMut<LatticeStats>,
) {
    for event in events.read() {
        match event.kind {
            WorldEventKind::MercyRain => {
                for mut t in &mut trust {
                    t.0 *= 1.2;
                }
            }
            WorldEventKind::LatticeBloom => lattice.nodes += 50,
            WorldEventKind::ShareWave => {
                // Trading bonus (handled in trading system)
            }
        }
    }
}
