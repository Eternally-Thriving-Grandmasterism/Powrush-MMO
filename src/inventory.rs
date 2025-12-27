use bevy::prelude::*;
use bevy_replicon::prelude::*;
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
        app.add_systems(Update, (
            inventory_capacity_system,
            item_decay_system,
            item_generation_system,
            trading_system,
        ));
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
            item.mercy_value *= 0.99;
            item.mercy_value > 0.1
        });
    }
}

fn item_generation_system(
    mut query: Query<&mut Inventory>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    for mut inv in &mut query {
        if inv.items.len() < inv.capacity && rng.gen_bool(0.1 * time.delta_seconds()) {
            let item = Item {
                id: rng.gen(),
                name: format!("Mercy Crystal {}", rng.gen_range(1..100)),
                rarity: match rng.gen_range(0..100) {
                    0..80 => Rarity::Common,
                    80..95 => Rarity::Rare,
                    95..99 => Rarity::Epic,
                    _ => Rarity::Legendary,
                },
                mercy_value: rng.gen_range(5.0..50.0),
            };
            inv.items.push(item);
            info!("Generated item: {:?} ({:?})", item.name, item.rarity);
        }
    }
}

fn trading_system(
    mut query: Query<&mut Inventory>,
    events: EventReader<TradeEvent>,
) {
    for event in events.read() {
        if let Ok(mut inv1) = query.get_mut(event.player1) {
            if let Ok(mut inv2) = query.get_mut(event.player2) {
                if let Some(item) = inv1.items.pop() {
                    inv2.items.push(item.clone());
                    info!("Trade complete — item moved");
                }
            }
        }
    }
}

#[derive(Event)]
pub struct TradeEvent {
    pub player1: Entity,
    pub player2: Entity,
}
