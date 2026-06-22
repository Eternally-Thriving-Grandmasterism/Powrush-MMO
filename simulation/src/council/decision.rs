/*!
 * Fully wired create_active_policy in apply_council_decision_effects.
 */

// Updated structure inside the inner loop (after applying effects):

// Calculate enriched score first (post-effect)
let avg_sustainability: f32 = world.rbe_pools.values()
    .map(|p| p.sustainability_score)
    .sum::<f32>() / world.rbe_pools.len().max(1) as f32;

let avg_abundance: f32 = world.rbe_pools.values()
    .map(|p| p.abundance_flow)
    .sum::<f32>() / world.rbe_pools.len().max(1) as f32;

let base = mercy.clamp(0.35, 1.0);
let archetype_bonus: f32 = match effect { /* ... */ };
let delta_component = (avg_sustainability * 0.55 + avg_abundance * 0.45).clamp(0.4, 1.0);

decision.final_mercy_alignment_score =
    (base * 0.50 + archetype_bonus * 0.25 + delta_component * 0.25).clamp(0.0, 1.0);

// Then create policies using the helper
match effect {
    "ResourcePolicy" | "resource_policy" => {
        // immediate effects...
        world.active_policies.push(
            decision.create_active_policy(PolicyType::AbundanceBoost, 0.15 * mag, 120)
        );
    }
    "HarmonyBoost" | "harmony_boost" => {
        // immediate effects...
        world.active_policies.push(
            decision.create_active_policy(PolicyType::HarmonyStabilization, 0.12 * mag, 90)
        );
    }
    "EpiphanyEvent" | "epiphany_event" => {
        // immediate effects...
        world.active_policies.push(
            decision.create_active_policy(PolicyType::GeneralProsperity, 0.10 * mag, 60)
        );
    }
    _ => {}
}
