use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Event, Replicated)]
pub struct TradeRequest {
    pub from: Entity,
    pub to: Entity,
    pub item_from: Item,
}

#[derive(Event, Replicated)]
pub struct TradeAccept {
    pub trade_id: u64,
    pub item_to: Item,
}

#[derive(Component)]
pub struct PendingTrade {
    pub trade_id: u64,
    pub other_player: Entity,
    pub offered_item: Item,
}

pub struct TradingPlugin;

impl Plugin for TradingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TradeRequest>()
           .add_event::<TradeAccept>()
           .add_systems(Update, (
               trade_request_system,
               trade_accept_system,
               trade_complete_system,
           ));
    }
}

fn trade_request_system(
    mut events: EventReader<TradeRequest>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.entity(event.to).insert(PendingTrade {
            trade_id: rand::thread_rng().gen(),
            other_player: event.from,
            offered_item: event.item_from.clone(),
        });
        info!("Trade request sent — {} offering {}", event.from, event.item_from.name);
    }
}

fn trade_accept_system(
    mut commands: Commands,
    mut events: EventReader<TradeAccept>,
    pending: Query<(Entity, &PendingTrade)>,
    mut inventories: Query<&mut Inventory>,
) {
    for event in events.read() {
        if let Ok((to_entity, pending)) = pending.iter().find(|p| p.1.trade_id == event.trade_id) {
            if let Ok(mut inv_from) = inventories.get_mut(pending.other_player) {
                if let Ok(mut inv_to) = inventories.get_mut(to_entity) {
                    // Swap items
                    inv_from.items.retain(|i| i.id != pending.offered_item.id);
                    inv_to.items.push(pending.offered_item.clone());
                    inv_to.items.push(event.item_to.clone());

                    // Mercy bonus
                    if let Ok(mut trust_from) = inv_from.get_mut::<&mut TrustCredits>() {
                        trust_from.0 *= 1.1;
                    }
                    if let Ok(mut trust_to) = inv_to.get_mut::<&mut TrustCredits>() {
                        trust_to.0 *= 1.1;
                    }

                    info!("Trade complete — mercy flows");
                    commands.entity(to_entity).remove::<PendingTrade>();
                }
            }
        }
    }
}

fn trade_complete_system(
    mut commands: Commands,
    completed: Query<Entity, With<PendingTrade>>,  // Cleanup if needed
) {
    // Visual mercy wave on trade
    for _ in completed.iter() {
        // Spawn gold particle burst + lattice glow
        info!("Mercy wave — trade thriving");
    }
}
