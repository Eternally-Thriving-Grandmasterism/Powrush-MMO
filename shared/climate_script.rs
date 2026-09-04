//! shared/climate_script.rs
//! Two first-hour teaching climates. Hands, not a manifesto.
//! Mercy restore vs extract-only. Contact: info@Rathor.ai

use crate::climate_node::{AllocKind, LivedHour, NodeState};

/// Tend twice, then flow. The tired node breathes.
pub fn run_mercy_restore() -> LivedHour {
    let mut hour = LivedHour::new_demo();
    let _ = hour.tend(1);
    let _ = hour.tend(1);
    let _ = hour.allocate(AllocKind::Flow);
    hour
}

/// Tend twice. No flow. The node stays tired.
pub fn run_extract_only() -> LivedHour {
    let mut hour = LivedHour::new_demo();
    let _ = hour.tend(1);
    let _ = hour.tend(1);
    hour
}

pub fn mercy_restore_holds(hour: &LivedHour, node_id: u32) -> bool {
    if hour.allocation.flow == 0 {
        return false;
    }
    hour.nodes.iter().any(|n| {
        n.id == node_id && matches!(n.state, NodeState::Glowing | NodeState::Resting)
    })
}

pub fn extract_only_holds(hour: &LivedHour, node_id: u32) -> bool {
    if hour.allocation.flow > 0 {
        return false;
    }
    hour.nodes.iter().any(|n| {
        n.id == node_id && matches!(n.state, NodeState::Resting | NodeState::Stressed)
    })
}

pub fn mercy_line() -> &'static str {
    "flow restored the well"
}

pub fn extract_line() -> &'static str {
    "extract left it tired"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mercy_restore_script_breathes() {
        let hour = run_mercy_restore();
        assert!(mercy_restore_holds(&hour, 1));
        assert!(!extract_only_holds(&hour, 1));
        assert_eq!(hour.allocation.flow, 1);
        assert!(matches!(
            hour.nodes[0].state,
            NodeState::Glowing | NodeState::Resting
        ));
    }

    #[test]
    fn extract_only_script_tires() {
        let hour = run_extract_only();
        assert!(extract_only_holds(&hour, 1));
        assert!(!mercy_restore_holds(&hour, 1));
        assert_eq!(hour.allocation.flow, 0);
        assert!(matches!(
            hour.nodes[0].state,
            NodeState::Resting | NodeState::Stressed
        ));
    }

    #[test]
    fn lines_are_sentences() {
        assert!(mercy_line().len() < 32);
        assert!(extract_line().len() < 32);
    }
}
