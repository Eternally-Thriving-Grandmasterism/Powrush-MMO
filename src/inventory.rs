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
            item_ui_render_system,
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
        if inv.items.len() < inv.capacity && rng.gen_bool(0.05 * time.delta_seconds()) {
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
        }
    }
}

fn item_ui_render_system(
    mut commands: Commands,
    query: Query<&Inventory>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.2, 0.8)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Inventory",
            TextStyle { font_size: 32.0, color: Color::GOLD, ..default() },
        ));

        if let Ok(inv) = query.get_single() {
            for item in &inv.items {
                let color = match item.rarity {
                    Rarity::Common => Color::WHITE,
                    Rarity::Rare => Color::GREEN,
                    Rarity::Epic => Color::PURPLE,
                    Rarity::Legendary => Color::GOLD,
                };
                parent.spawn(TextBundle::from_section(
                    format!("{} ({:?}) — {:.1}", item.name, item.rarity, item.mercy_value),
                    TextStyle { font_size: 20.0, color, ..default() },
                ));
            }
        }
    });
}
