use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct CraftingRecipe {
    pub inputs: Vec<Item>,
    pub output: Item,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crafting_system);
    }
}

fn crafting_system(
    mut query: Query<&mut Inventory>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        if let Ok(mut inv) = query.get_single_mut() {
            if inv.items.len() >= 2 {
                let item1 = inv.items.remove(0);
                let item2 = inv.items.remove(0);
                let crafted = Item {
                    id: rand::thread_rng().gen(),
                    name: "Mercy Sword".to_string(),
                    rarity: Rarity::Epic,
                    mercy_value: item1.mercy_value + item2.mercy_value * 1.5,
                };
                inv.items.push(crafted);
                info!("Crafted mercy sword");
            }
        }
    }
}
