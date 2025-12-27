use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct CombatPlayer {
    pub health: f32,
    pub mercy_shield: f32,
    pub cooldown: Timer,
}

#[derive(Event, Replicated)]
pub struct CombatAttackEvent(pub Entity, pub Entity);

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatAttackEvent>()
           .add_systems(Update, (combat_input_system, combat_damage_system));
    }
}

fn combat_input_system(
    mouse: Res<Input<MouseButton>>,
    mut events: EventWriter<CombatAttackEvent>,
    players: Query<Entity, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(player) = players.get_single() {
            events.send(CombatAttackEvent(player, player));  // Placeholder target
        }
    }
}

fn combat_damage_system(
    mut events: EventReader<CombatAttackEvent>,
    mut query: Query<&mut CombatPlayer>,
) {
    for event in events.read() {
        if let Ok(mut target) = query.get_mut(event.1) {
            target.health -= 20.0;
            if target.health <= 0.0 {
                target.health = 100.0;
                target.mercy_shield += 50.0;
                info!("Mercy forgiveness — shield renewed");
            }
        }
    }
}
