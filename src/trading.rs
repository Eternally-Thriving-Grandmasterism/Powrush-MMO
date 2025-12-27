use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Event, Replicated)]
pub struct PlayerTradeEvent {
    pub from: Entity,
    pub to: Entity,
    pub item: Item,
}

pub struct TradingPlugin;

impl Plugin for TradingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTradeEvent>()
           .add_systems(Update, player_trade_system);
    }
}

fn player_trade_system(
    mut events: EventReader<PlayerTradeEvent>,
    mut inventories: Query<&mut Inventory>,
    mut trust: Query<&mut TrustCredits>,
) {
    for event in events.read() {
        if let Ok(mut from_inv) = inventories.get_mut(event.from) {
            if let Some(idx) = from_inv.items.iter().position(|i| i.id == event.item.id) {
                let item = from_inv.items.swap_remove(idx);
                if let Ok(mut to_inv) = inventories.get_mut(event.to) {
                    to_inv.items.push(item);
                    if let Ok(mut from_trust) = trust.get_mut(event.from) {
                        from_trust.0 *= 1.1;
                    }
                    if let Ok(mut to_trust) = trust.get_mut(event.to) {
                        to_trust.0 *= 1.1;
                    }
                    info!("Player trade — mercy flows");
                }
            }
        }
    }
}                        if let Ok(mut trust_from) = trust.get_mut(pending.other_player) {
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
