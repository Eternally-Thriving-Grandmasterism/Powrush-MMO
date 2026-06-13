/*!
 * Data Collection Hooks for Mirror Score
 * Listens to events from Council, Epiphany, and RBE systems
 */ 

use bevy::prelude::*;
use super::WeeklyServerMetrics;

#[derive(Event)]
pub struct CouncilParticipationEvent { pub quality: f32 }

#[derive(Event)]
pub struct EpiphanyQualityEvent { pub intensity: f32 }

#[derive(Event)]
pub struct RbeContributionEvent { pub amount: f32, pub mercy_aligned: bool }

pub fn council_participation_hook(
    mut events: EventReader<CouncilParticipationEvent>,
    mut metrics: ResMut<WeeklyServerMetrics>,
) {
    for ev in events.read() {
        metrics.council_participation = (metrics.council_participation + ev.quality * 10.0).min(100.0);
    }
}

pub fn epiphany_quality_hook(
    mut events: EventReader<EpiphanyQualityEvent>,
    mut metrics: ResMut<WeeklyServerMetrics>,
) {
    for ev in events.read() {
        metrics.epiphany_quality = (metrics.epiphany_quality + ev.intensity * 5.0).min(100.0);
    }
}

pub fn rbe_contribution_hook(
    mut events: EventReader<RbeContributionEvent>,
    mut metrics: ResMut<WeeklyServerMetrics>,
) {
    for ev in events.read() {
        let score = if ev.mercy_aligned { ev.amount * 1.5 } else { ev.amount * 0.8 };
        metrics.rbe_contribution = (metrics.rbe_contribution + score).min(100.0);
    }
}

pub fn register_data_collection_hooks(app: &mut App) {
    app.add_event::<CouncilParticipationEvent>()
        .add_event::<EpiphanyQualityEvent>()
        .add_event::<RbeContributionEvent>()
        .add_systems(Update, (
            council_participation_hook,
            epiphany_quality_hook,
            rbe_contribution_hook,
        ));
}
