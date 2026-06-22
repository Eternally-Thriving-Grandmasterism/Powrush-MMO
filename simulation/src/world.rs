/*!
 * Boost zone visualization when spatially targeted policies are applied.
 */

if let Some(zone_id) = target_zone {
    if let Some(zone) = self.interest_zones.get_mut(&zone_id) {
        // Increase visual highlight when policy affects this zone
        zone.visual_highlight = (zone.visual_highlight + strength * 0.8).min(1.0);

        // Optional: tint based on policy type
        match policy_type {
            PolicyType::AbundanceBoost => zone.visual_tint = [0.2, 0.9, 0.4],
            PolicyType::SustainabilityFocus => zone.visual_tint = [0.3, 0.8, 0.9],
            _ => {}
        }
    }
}
