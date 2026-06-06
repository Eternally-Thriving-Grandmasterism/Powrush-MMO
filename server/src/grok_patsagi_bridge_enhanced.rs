 // server/src/grok_patsagi_bridge.rs
// Enhanced to pass richer EnrichedNpcState context to PATSAGi queries

use crate::enriched_npc::EnrichedNpcState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NpcContextSummary {
    pub nearby_npcs: Vec<NpcSummary>,
    pub global_mercy_pressure: f32,
    pub dominant_faction_influence: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NpcSummary {
    pub faction: String,
    pub valence: f32,
    pub mercy_alignment: f32,
    pub rbe_contribution: f32,
    pub air_foundation_impact: f32,
}

impl GrokPATSAGiBridge {
    /// Enhanced query that includes living NPC context
    pub async fn query_patsagi_council_with_npc_context(
        &self,
        player_id: u64,
        query: &str,
        context: Option<String>,
        nearby_npcs: Vec<EnrichedNpcState>,
    ) -> Result<String, String> {
        let npc_summary = self.build_npc_context_summary(&nearby_npcs);

        let enriched_context = format!(
            "{}\n\nLiving World Context:\n- Nearby NPCs: {}\n- Global Mercy Pressure: {:.2}\n- Dominant Influence: {}",
            context.unwrap_or_default(),
            serde_json::to_string(&npc_summary.nearby_npcs).unwrap_or_default(),
            npc_summary.global_mercy_pressure,
            npc_summary.dominant_faction_influence.unwrap_or_default()
        );

        self.query_patsagi_council(player_id, Some(enriched_context), query.to_string()).await
    }

    fn build_npc_context_summary(&self, npcs: &[EnrichedNpcState]) -> NpcContextSummary {
        if npcs.is_empty() {
            return NpcContextSummary { nearby_npcs: vec![], global_mercy_pressure: 0.0, dominant_faction_influence: None };
        }

        let mut total_mercy = 0.0;
        let mut faction_counts: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        let summaries: Vec<NpcSummary> = npcs.iter().map(|npc| {
            total_mercy += npc.mercy_alignment;
            *faction_counts.entry(npc.faction.clone()).or_insert(0) += 1;
            NpcSummary {
                faction: npc.faction.clone(),
                valence: npc.valence,
                mercy_alignment: npc.mercy_alignment,
                rbe_contribution: npc.rbe_contribution,
                air_foundation_impact: npc.air_foundation_impact,
            }
        }).collect();

        let avg_mercy = total_mercy / npcs.len() as f32;
        let dominant = faction_counts.iter().max_by_key(|(_, count)| *count).map(|(f, _)| f.clone());

        NpcContextSummary {
            nearby_npcs: summaries,
            global_mercy_pressure: avg_mercy,
            dominant_faction_influence: dominant,
        }
    }
}