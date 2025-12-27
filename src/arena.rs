use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component)]
pub struct ArenaPlayer {
    pub health: f32,
    pub mercy_shield: f32,  // No death — shield absorbs
}

#[derive(Event, Replicated)]
pub struct ArenaDuelEvent(pub Entity, pub Entity);  // Player1 vs Player2

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ArenaDuelEvent>()
           .add_systems(Update, arena_duel_system);
    }
}

fn arena_duel_system(
    mut events: EventReader<ArenaDuelEvent>,
    mut players: Query<&mut ArenaPlayer>,
) {
    for event in events.read() {
        if let Ok(mut p1) = players.get_mut(event.0) {
            if let Ok(mut p2) = players.get_mut(event.1) {
                // Mercy duel — shield absorbs, no kill
                p1.mercy_shield -= 10.0;
                p2.mercy_shield -= 10.0;
                if p1.mercy_shield <= 0.0 {
                    p1.mercy_shield = 100.0;  // Respawn mercy
                    info!("Player 1 shielded — mercy return");
                }
                if p2.mercy_shield <= 0.0 {
                    p2.mercy_shield = 100.0;
                    info!("Player 2 shielded — mercy return");
                }
            }
        }
    }
}
