use bevy::prelude::*;

#[derive(Event)]
pub struct WorldEvent {
    pub kind: EventKind,
}

#[derive(Clone, Copy)]
pub enum EventKind {
    MercyRain,
    LatticeBloom,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldEvent>()
           .add_systems(Update, spawn_world_event);
    }
}

fn spawn_world_event(
    mut events: EventWriter<WorldEvent>,
    time: Res<Time>,
) {
    if (time.elapsed_seconds() % 300.0).abs() < time.delta_seconds() {
        events.send(WorldEvent { kind: EventKind::MercyRain });
    }
}
