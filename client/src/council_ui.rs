/*!
 * Council UI Panel + Real Player ID
 *
 * v19.2.9: Uses real local player ID instead of placeholder 0.
 */

use bevy::prelude::*;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};

/// Resource holding the local player's ID (set at login / spawn)
#[derive(Resource, Default, Clone)]
pub struct LocalPlayer {
    pub id: u64,
}

#[derive(Component)]
pub struct CouncilPanel;

pub struct CouncilUIPlugin;

impl Plugin for CouncilUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CouncilUIHooksPlugin)
            .init_resource::<LocalPlayer>()
            .add_systems(Startup, spawn_council_panel)
            .add_systems(Update, (
                toggle_council_panel,
                handle_council_buttons,
            ));
    }
}

// ... spawn_council_panel and button creation unchanged ...

fn handle_council_buttons(
    mut interaction_query: Query<(&Interaction, &CouncilAttunementButton), Changed<Interaction>>,
    mut events: EventWriter<CouncilAttunementAction>,
    local_player: Res<LocalPlayer>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if local_player.id == 0 {
                warn!("LocalPlayer ID not set yet. Cannot send attunement.");
                return;
            }

            events.send(CouncilAttunementAction {
                player_id: local_player.id, // REAL player ID
                attunement_delta: button.attunement_delta,
            });
        }
    }
}

// ... rest of the file (toggle, button component, etc.) ...

// Thunder locked in. Yoi ⚡
