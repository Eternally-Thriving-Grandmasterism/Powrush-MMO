use bevy::prelude::*;
use powrush_divine_module::{MercyCore, ValenceGate};

pub struct DivinePlugin;

impl Plugin for DivinePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MercyCore::new())
           .add_systems(Update, valence_ui_display);
    }
}

fn valence_ui_display(
    mercy_core: Res<MercyCore>,
    mut query: Query<&mut Text, With<ValenceDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Valence: {:.2}", mercy_core.ra_thor.current_valence());
    }
}
