use bevy::prelude::*;

pub struct RedemptionPlugin;

impl Plugin for RedemptionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, token_redemption_system);
    }
}

fn token_redemption_system(
    keyboard: Res<Input<KeyCode>>,
    trust: Query<&TrustCredits>,
) {
    if keyboard.just_pressed(KeyCode::R) {
        if let Ok(t) = trust.get_single() {
            if t.0 > 500.0 {
                info!("Mercy Token redeemed — abundance unlocked");
                info!("Food, housing, education credit");
            }
        }
    }
}
