use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Replicated)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

#[derive(Clone, Replicated)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub rarity: Rarity,
    pub mercy_value: f32,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Default for Inventory {
    fn default() -> Self {
        Self { items: vec![], capacity: 10 }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (inventory_capacity_system, item_decay_system));
    }
}

fn inventory_capacity_system(
    mut query: Query<(&TrustCredits, &mut Inventory)>,
) {
    for (trust, mut inv) in &mut query {
        inv.capacity = (trust.0 * 10.0) as usize;
    }
}

fn item_decay_system(
    mut query: Query<&mut Inventory>,
    time: Res<Time>,
) {
    for mut inv in &mut query {
        inv.items.retain_mut(|item| {
            item.mercy_value *= 0.99;  // Gentle decay if hoarded
            item.mercy_value > 0.1
        });
    }
}
