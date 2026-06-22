/*!
 * Lerp visual_highlight down over time in update_interest_zones_system.
 */

pub fn update_interest_zones_system(
    mut query: Query<&mut InterestZone>,
) {
    for mut zone in &mut query {
        zone.valence_multiplier = zone.valence_multiplier.lerp(1.0, 0.05).max(0.5);
        zone.council_boost = zone.council_boost.lerp(0.0, 0.08).max(0.0);
        zone.mercy_resonance = zone.mercy_resonance.lerp(0.0, 0.06).max(0.0);

        // Decay visual highlight from policies
        zone.visual_highlight = zone.visual_highlight.lerp(0.0, 0.04).max(0.0);

        zone.smooth_correct(0.12);
    }
}
