use bevy::prelude::*;
use bevy_replicon::prelude::*;

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
}

pub struct AuctionPlugin;

impl Plugin for AuctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BidEvent>()
           .add_systems(Update, (auction_bid_system, auction_timer_system));
    }
}

fn auction_bid_system(
    mut events: EventReader<BidEvent>,
    mut auctions: Query<&mut Auction>,
    mut trust: Query<&mut TrustCredits>,
) {
    for event in events.read() {
        if let Ok(mut auction) = auctions.get_mut(event.auction_id as Entity) {
            if event.amount > auction.current_bid {
                auction.current_bid = event.amount;
                auction.highest_bidder = Some(event.bidder);
                if let Ok(mut bidder_trust) = trust.get_mut(event.bidder) {
                    bidder_trust.0 *= 1.05;  // Mercy for fair bid
                }
                info!("New bid: {:.2} MP — auction thriving", event.amount);
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
