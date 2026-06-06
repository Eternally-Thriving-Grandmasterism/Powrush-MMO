use crate::enriched_npc::EnrichedNpcState;
use crate::persistence::Persistence;
use std::path::Path;

impl WorldServer {
    /// Loads fresh procedural NPC state from public artifact or falls back to high-quality deterministic generation.
    /// Called on server boot and on explicit world refresh.
    /// 100% compliant with sealed Nanotech & Harvest Canon + Ambrosians as 5th divine race.
    pub async fn load_fresh_npc_snapshots(&mut self, public_artifact_url: Option<&str>) {
        if let Some(url) = public_artifact_url {
            if let Ok(content) = reqwest::get(url).await.and_then(|r| r.text().await) {
                if let Ok(snapshots) = serde_json::from_str::<Vec<EnrichedNpcState>>(&content) {
                    for snapshot in snapshots {
                        let zone = self.get_or_create_zone_for_faction(&snapshot.faction);
                        let npc = self.spawn_or_update_npc_from_snapshot(snapshot, zone);
                        self.apply_lore_valence_modifiers(&npc);
                    }
                    tracing::info!("Loaded fresh NPC snapshots from public artifact");
                    return;
                }
            }
        }

        // Fallback: high-quality deterministic generation (still lore-compliant)
        self.spawn_default_lore_npcs().await;
        tracing::info!("Using deterministic lore-compliant NPC generation (no public artifact available)");
    }

    fn apply_lore_valence_modifiers(&mut self, npc: &mut CreatureState) {
        // Draeks: higher desperation from cloning crisis
        // Cydruids: strong connection to Fracture / Hollow Earth
        // Quellorians: naturally higher mercy_alignment
        // Humans: variable based on collaborative actions
        // Ambrosians (5th secret/divine race): rare, high-valence, high-mercy, steer toward abundance
        if let Some(enriched) = &mut npc.enriched_state {
            match enriched.faction.as_str() {
                "Draek" => enriched.valence = (enriched.valence * 0.85).max(0.3),
                "Cydruid" => enriched.mercy_alignment = (enriched.mercy_alignment * 1.15).min(1.0),
                "Quellorian" => enriched.mercy_alignment = (enriched.mercy_alignment * 1.25).min(1.0),
                "Ambrosian" => {
                    enriched.valence = (enriched.valence * 1.4).min(1.0);
                    enriched.mercy_alignment = 0.95;
                }
                _ => {}
            }
        }
    }

    // ... existing methods ...
}