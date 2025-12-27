use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

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

#[derive(Event, Replicated)]
pub struct AuctionList {
    pub item: Item,
    pub starting_price: f32,
    pub duration: Timer,
}

#[derive(Component)]
pub struct PendingTrade {
    pub trade_id: u64,
    pub other_player: Entity,
    pub offered_item: Item,
}

#[derive(Component)]
pub struct AuctionHouse;

pub struct TradingPlugin;

impl Plugin for TradingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TradeRequest>()
           .add_event::<TradeAccept>()
           .add_event::<AuctionList>()
           .add_systems(Update, (
               trade_request_system,
               trade_accept_system,
               trade_complete_system,
               auction_house_system,
               trading_ui_system,
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
        info!("Trade request sent");
    }
}

fn trade_accept_system(
    mut commands: Commands,
    mut events: EventReader<TradeAccept>,
    pending: Query<(Entity, &PendingTrade)>,
    mut inventories: Query<&mut Inventory>,
    mut trust: Query<&mut TrustCredits>,
) {
    for event in events.read() {
        if let Ok((to_entity, pending)) = pending.iter().find(|p| p.1.trade_id == event.trade_id) {
            if let Ok(mut inv_from) = inventories.get_mut(pending.other_player) {
                if let Ok(mut inv_to) = inventories.get_mut(to_entity) {
                    // Robust swap with validation
                    if let Some(idx) = inv_from.items.iter().position(|i| i.id == pending.offered_item.id) {
                        let item_from = inv_from.items.swap_remove(idx);
                        inv_to.items.push(item_from);
                        inv_to.items.push(event.item_to.clone());

                        // Trust boost
                        if let Ok(mut trust_from) = trust.get_mut(pending.other_player) {
                            trust_from.0 *= 1.1;
                        }
                        if let Ok(mut trust_to) = trust.get_mut(to_entity) {
                            trust_to.0 *= 1.1;
                        }

                        info!("Trade complete — mercy flows");
                        commands.entity(to_entity).remove::<PendingTrade>();
                    }
                }
            }
        }
    }
}

fn auction_house_system(
    mut events: EventReader<AuctionList>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.spawn((
            AuctionHouse,
            event.item.clone(),
            Timer::from_seconds(event.duration.time().as_secs_f32(), TimerMode::Once),
        ));
        info!("Auction listed — {} for {:.2} MP", event.item.name, event.starting_price);
    }
}

fn trading_ui_system(
    mut commands: Commands,
    inventories: Query<&Inventory>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.2, 0.9)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Trading Hub",
            TextStyle { font_size: 36.0, color: Color::GOLD, ..default() },
        ));

        if let Ok(inv) = inventories.get_single() {
            for item in &inv.items {
                parent.spawn(ButtonBundle {
                    style: Style { padding: UiRect::all(Val::Px(10.0)), ..default() },
                    background_color: BackgroundColor(Color::CYAN),
                    ..default()
                }).with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        &item.name,
                        TextStyle { font_size: 24.0, color: Color::WHITE, ..default() },
                    ));
                });
            }
        }
    });
}
