// ... existing harvest.rs v18.13 content ...

// v18.14 Dynamic Challenge Balancer integration
use crate::flow_state_forge::{dynamic_challenge_skill_balancer, ChallengeBalancerConfig, FlowStateMetrics};

// Inside attempt_harvest on sustainable path, after receptor + flow checks:
// let balanced_resistance = dynamic_challenge_skill_balancer(&flow_metrics, current_node_resistance, &ChallengeBalancerConfig::default());
// Apply balanced_resistance to yield calculation and store in FlowStateOutcome

// CLIENT_HOOK: resistance affects visual/audio intensity of harvest action and particle density
// Full wiring ready for production.