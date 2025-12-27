use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

#[derive(Event, Replicated)]
pub struct BidEvent {
    pub auction_id: u64,
    pub bidder: Entity,
    pub amount: f32,
}

#[derive(Component)]
pub struct Auction {
    pub id: u64,
    pub item: Item,
    pub current_bid: f32,
    pub highest_bidder: Option<Entity>,
    pub timer: Timer,
    pub min_increment: f32,
}

pub struct AuctionPlugin;

impl Plugin for AuctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BidEvent>()
           .add_systems(Update, (auction_bid_system, auction_timer_system, auction_ui_system));
    }
}

fn auction_bid_system(
    mut events: EventReader<BidEvent>,
    mut auctions: Query<&mut Auction>,
    mut trust: Query<&mut TrustCredits>,
) {
    for event in events.read() {
        if let Ok(mut auction) = auctions.get_mut(event.auction_id as Entity) {
            let required = auction.current_bid + auction.min_increment;
            if event.amount >= required {
                auction.current_bid = event.amount;
                auction.highest_bidder = Some(event.bidder);
                if let Ok(mut bidder_trust) = trust.get_mut(event.bidder) {
                    bidder_trust.0 *= 1.05;  // Mercy for fair bid
                }
                info!("Bid accepted — new high: {:.2} MP", event.amount);
            } else {
                info!("Bid too low — need {:.2}", required);
            }
        }
    }
}

fn auction_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut auctions: Query<(Entity, &mut Auction)>,
) {
    for (entity, mut auction) in auctions.iter_mut() {
        auction.timer.tick(time.delta());
        if auction.timer.finished() {
            if let Some(winner) = auction.highest_bidder {
                info!("Auction won — mercy flows to winner");
                // Award item + trust bonus
            }
            commands.entity(entity).despawn();
        }
    }
}

fn auction_ui_system(
    mut commands: Commands,
    auctions: Query<&Auction>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            bottom: Val::Px(20.0),
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.2, 0.9)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Auction House",
            TextStyle { font_size: 36.0, color: Color::GOLD, ..default() },
        ));

        for auction in auctions.iter() {
            parent.spawn(TextBundle::from_section(
                format!("{} — Current: {:.2}", auction.item.name, auction.current_bid),
                TextStyle { font_size: 24.0, color: Color::CYAN, ..default() },
            ));
        }
    });
}
