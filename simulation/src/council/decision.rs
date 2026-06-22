/*!
 * CouncilDecision - Updated policy creation to use mercy-scaled helper.
 */

// In apply_council_decision_effects, replace manual creation with:

// Example replacement:
// world.active_policies.push(
//     decision.create_active_policy(PolicyType::AbundanceBoost, 0.15 * mag, 120)
// );

// Do similar replacements for other policy creations.
