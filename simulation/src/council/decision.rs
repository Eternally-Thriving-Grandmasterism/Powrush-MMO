/*!
 * CouncilDecision - Wired create_active_policy helper into apply_council_decision_effects.
 */

// Inside apply_council_decision_effects, after applying effects to the world
// but before creating policies, we now calculate the final score first:

// (pseudocode of the updated structure)

for world in query.iter_mut() {
    // 1. Apply immediate effects to world (existing match)
    // ...

    // 2. Calculate final_mercy_alignment_score (using post-effect world state)
    let avg_sustainability = ...;
    let avg_abundance = ...;
    // ... calculate decision.final_mercy_alignment_score ...

    // 3. Now create policies using the helper (which reads final_mercy_alignment_score)
    match effect {
        "ResourcePolicy" | "resource_policy" => {
            // ...
            world.active_policies.push(
                decision.create_active_policy(PolicyType::AbundanceBoost, 0.15 * mag, 120)
            );
        }
        // similar for other branches
    }

    // 4. Record to history, etc.
}
