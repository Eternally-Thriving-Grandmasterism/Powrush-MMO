use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub rarity: Rarity,
    pub mercy_value: f32,  // Contrib to RBE
}

#[derive(Clone, Copy, PartialEq)]
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

pub fn inventory_system(
    mut query: Query<(&mut Inventory, &TrustCredits)>,
) {
    for (mut inv, trust) in &mut query {
        inv.capacity = (trust.0 * 10.0) as usize;

        // Decay hoarded items
        inv.items.retain_mut(|item| {
            item.mercy_value *= 0.99;  // Gentle decay
            item.mercy_value > 0.1
        });
    }
}

pub fn pickup_item_system(
    mut commands: Commands,
    mut query: Query<&mut Inventory>,
    events: EventReader<PickupEvent>,
) {
    let mut rng = rand::thread_rng();
    for event in events.read() {
        for mut inv in &mut query {
            if inv.items.len() < inv.capacity {
                let item = Item {
                    id: rng.gen(),
                    name: "Mercy Crystal".to_string(),
                    rarity: match rng.gen_range(0..100) {
                        0..80 => Rarity::Common,
                        80..95 => Rarity::Rare,
                        95..99 => Rarity::Epic,
                        _ => Rarity::Legendary,
                    },
                    mercy_value: rng.gen_range(5.0..50.0),
                };
                inv.items.push(item.clone());
                info!("Picked up: {} ({:?})", item.name, item.rarity);
            }
        }
    }
}

pub fn share_item_system(
    mut query: Query<&mut Inventory>,
    mut guild_query: Query<&mut GuildResources>,
    events: EventReader<ShareItemEvent>,
) {
    for event in events.read() {
        if let Ok(mut inv) = query.get_mut(event.player) {
            if let Some(item) = inv.items.pop() {
                if let Ok(mut guild_res) = guild_query.get_mut(event.guild) {
                    guild_res.0 += item.mercy_value * 1.2;  // Guild bonus
                    info!("Shared {} — guild +{:.2} MP", item.name, item.mercy_value * 1.2);
                }
            }
        }
    }
}

#[derive(Event)]
pub struct PickupEvent(pub Entity);  // Player picking up

#[derive(Event)]
pub struct ShareItemEvent {
    pub player: Entity,
    pub guild: Entity,
}

#[derive(Component)]
pub struct Player;  // Marker for pickup
