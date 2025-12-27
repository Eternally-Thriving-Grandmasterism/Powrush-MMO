use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Event, Replicated)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub mercy_forgiven: bool,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>()
           .add_systems(Update, combat_system);
    }
}

fn combat_system(
    mouse: Res<Input<MouseButton>>,
    mut events: EventWriter<CombatEvent>,
    players: Query<(Entity, &Transform)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Simple duel trigger
        if let Ok((attacker, _)) = players.get_single() {
            // Target logic stub
            events.send(CombatEvent {
                attacker,
                target: attacker, // placeholder
                mercy_forgiven: true,
            });
        }
    }
}
