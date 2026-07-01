/*!
 * server/src/inventory_replication.rs
 * Full server-side handling for InventoryMove (40-slot general) + hotbar (8-slot).
 * MercyAnomalyDetector fully wired as Bevy Resource (report_anomaly called on high anomaly rejections).
 * SafetyNet integration point documented for EmitSafetyNetBroadcast in calling systems (process_inventory_messages / lib.rs).
 * All prior logic (swap, persistence load/save, replication) preserved exactly. AG-SML v1.0 | TOLC 8 + RBE + PATSAGi
 */

use shared::protocol::{ClientMessage, ServerMessage, HotbarSlot};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use tracing::{info, warn};
use crate::mercy_anomaly_detector::{AnomalyType, MercyAnomalyDetector};

/// Server authoritative validation result (stricter than client).
pub struct AuthoritativeMoveValidity {
    pub allowed: bool,
    pub reason: Option<String>,
    pub mercy_resonance: f32,
    pub abundance_score: f32,
    pub anomaly_score: f32,
}

/// Authoritative server mirror of client validate_move.
/// Uses real inventory[40] / hotbar[8] data from PlayerSaveData. Full TOLC 8 Mercy + RBE abundance/anti-tyranny + anomaly scoring.
pub fn validate_inventory_move_authoritative(
    player_data: &PlayerSaveData,
    from: usize,
    to: usize,
    is_hotbar: bool,
) -> AuthoritativeMoveValidity {
    if from == to {
        return AuthoritativeMoveValidity {
            allowed: false,
            reason: Some("Same slot".to_string()),
            mercy_resonance: 0.5,
            abundance_score: 0.5,
            anomaly_score: 0.0,
        };
    }

    if is_hotbar {
        if from >= 8 || to >= 8 {
            return AuthoritativeMoveValidity { allowed: false, reason: Some("Invalid hotbar slot".to_string()), mercy_resonance: 0.2, abundance_score: 0.3, anomaly_score: 0.8 };
        }
    } else {
        if from >= 40 || to >= 40 {
            return AuthoritativeMoveValidity { allowed: false, reason: Some("Invalid inventory slot".to_string()), mercy_resonance: 0.2, abundance_score: 0.3, anomaly_score: 0.8 };
        }
    }

    // === Real data lookup (wired from PlayerSaveData) ===
    let src_slot: &HotbarSlot = if is_hotbar {
        &player_data.hotbar[from]
    } else {
        &player_data.inventory[from]
    };
    let _tgt_slot: &HotbarSlot = if is_hotbar {
        &player_data.hotbar[to]
    } else {
        &player_data.inventory[to]
    };

    let src_valence = src_slot.valence;

    // Mercy gate (stricter server-side)
    let mercy_gate = if src_valence < -0.5 {
        0.25
    } else {
        0.92
    };

    // RBE Abundance + anti-tyranny gate
    let abundance_impact = if src_slot.count > 15 { 0.55 } else { 0.88 };

    // Anomaly scoring (feeds MercyAnomalyDetector)
    let anomaly = if mercy_gate < 0.4 || abundance_impact < 0.5 { 0.65 } else { 0.15 };

    let allowed = mercy_gate > 0.4 && abundance_impact > 0.4 && anomaly < 0.7;
    let final_mercy = (mercy_gate * 0.5 + abundance_impact * 0.3 + (1.0 - anomaly) * 0.2).clamp(0.0, 1.0);

    AuthoritativeMoveValidity {
        allowed,
        reason: if allowed { None } else { Some("Authoritative rejection: Mercy/RBE/anomaly violation".to_string()) },
        mercy_resonance: final_mercy,
        abundance_score: abundance_impact,
        anomaly_score: anomaly,
    }
}

pub fn handle_inventory_action(
    player_id: u64,
    message: &ClientMessage,
    persistence: &mut PersistenceManager,
    detector: &mut MercyAnomalyDetector,
) -> Option<ServerMessage> {
    match message {
        ClientMessage::InventoryHotbarMove { from_slot, to_slot } => {
            handle_hotbar_move(player_id, *from_slot, *to_slot, persistence, detector)
        }
        ClientMessage::InventoryMove { from, to } => {
            handle_general_inventory_move(player_id, *from, *to, persistence, detector)
        }
        _ => None,
    }
}

fn handle_hotbar_move(
    player_id: u64,
    from_slot: u8,
    to_slot: u8,
    persistence: &mut PersistenceManager,
    detector: &mut MercyAnomalyDetector,
) -> Option<ServerMessage> {
    if from_slot == to_slot || from_slot >= 8 || to_slot >= 8 {
        return None;
    }

    let mut player_data = persistence.load_player(player_id)
        .unwrap_or_else(|| PlayerSaveData::new(player_id));

    let validity = validate_inventory_move_authoritative(&player_data, from_slot as usize, to_slot as usize, true);
    if !validity.allowed {
        warn!("[Inventory] Authoritative hotbar move rejected for player {}: {:?} (anomaly={:.2})", player_id, validity.reason, validity.anomaly_score);

        if validity.anomaly_score > 0.5 {
            // Feed to MercyAnomalyDetector (Bevy Resource wired in ServerCorePlugin)
            let _action = detector.report_anomaly(
                player_id,
                AnomalyType::Custom("InventoryHotbarViolation".to_string()),
                validity.anomaly_score,
                format!("Rejected hotbar move {} -> {} | {}", from_slot, to_slot, validity.reason.as_deref().unwrap_or("")),
            );
            // SafetyNet: EmitSafetyNetBroadcast event recommended here for high-anomaly (see safety_net_broadcast.rs + process_inventory_messages system for EventWriter integration). Current: anomaly fed to detector for monitoring/moderation.
            warn!("[Inventory] High anomaly ({:.2}) → MercyAnomalyDetector triggered | SafetyNet emission point ready in broader server loop", validity.anomaly_score);
        }
        return None;
    }

    // Existing swap logic (100% preserved from prior iterations)
    if (from_slot as usize) < 8 && (to_slot as usize) < 8 {
        let tmp = player_data.hotbar[from_slot as usize].clone();
        player_data.hotbar[from_slot as usize] = player_data.hotbar[to_slot as usize].clone();
        player_data.hotbar[to_slot as usize] = tmp;
    }

    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to persist hotbar move: {}", e);
    }

    info!("[Inventory] Hotbar move persisted (mercy={:.2} abundance={:.2})", validity.mercy_resonance, validity.abundance_score);

    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar.clone(),
        inventory: player_data.inventory.clone(),
        abundance_score: player_data.abundance as f32,
    })
}

fn handle_general_inventory_move(
    player_id: u64,
    from: u32,
    to: u32,
    persistence: &mut PersistenceManager,
    detector: &mut MercyAnomalyDetector,
) -> Option<ServerMessage> {
    if from == to || from >= 40 || to >= 40 {
        return None;
    }

    let mut player_data = persistence.load_player(player_id)
        .unwrap_or_else(|| PlayerSaveData::new(player_id));

    let validity = validate_inventory_move_authoritative(&player_data, from as usize, to as usize, false);
    if !validity.allowed {
        warn!("[Inventory] Authoritative general move rejected for player {}: {:?} (anomaly={:.2})", player_id, validity.reason, validity.anomaly_score);

        if validity.anomaly_score > 0.5 {
            // Feed to MercyAnomalyDetector (Bevy Resource wired)
            let _action = detector.report_anomaly(
                player_id,
                AnomalyType::Custom("InventoryGeneralViolation".to_string()),
                validity.anomaly_score,
                format!("Rejected general inventory move {} -> {} | {}", from, to, validity.reason.as_deref().unwrap_or("")),
            );
            // SafetyNet emission point ready (see safety_net_broadcast.rs). Anomaly scoring active for monitoring.
            warn!("[Inventory] High anomaly ({:.2}) → MercyAnomalyDetector triggered", validity.anomaly_score);
        }
        return None;
    }

    // Existing authoritative swap (100% preserved)
    player_data.swap_inventory_slots(from as usize, to as usize);

    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to persist general inventory move: {}", e);
    }

    info!("[Inventory] General inventory move persisted: {} -> {} (mercy={:.2})", from, to, validity.mercy_resonance);

    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar.clone(),
        inventory: player_data.inventory.clone(),
        abundance_score: player_data.abundance as f32,
    })
}

// End of inventory_replication.rs v19.3 — Full authoritative inventory with TOLC 8, RBE, MercyAnomalyDetector. Thunder locked in. Yoi ⚡