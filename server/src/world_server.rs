use reqwest;
use std::time::Duration;
use tokio::time::sleep;
use tracing;

/// Loads fresh procedural NPC state from the public `artifacts` branch (zero authentication required).
/// Includes production-grade retry logic with exponential backoff.
/// Fully compliant with the sealed Nanotech & Harvest Canon + Ambrosians as the 5th divine race.
pub async fn load_fresh_npc_snapshots(&mut self) -> Result<(), String> {
    // Public zero-auth URL
    let url = std::env::var("POWRUSH_NPC_ARTIFACT_URL").unwrap_or_else(|_| {
        "https://raw.githubusercontent.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/artifacts/artifacts/latest_npc_snapshots.json".to_string()
    });

    // Retry configuration (production tunable)
    let max_retries: u32 = std::env::var("POWRUSH_ARTIFACT_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    let base_delay_ms: u64 = std::env::var("POWRUSH_ARTIFACT_RETRY_DELAY_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(25))
        .user_agent("Powrush-MMO-Server/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let mut last_error: Option<String> = None;

    for attempt in 0..=max_retries {
        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                let json_text = response.text().await.map_err(|e| e.to_string())?;

                let snapshots: Vec<EnrichedNpcState> =
                    serde_json::from_str(&json_text)
                        .map_err(|e| format!("Failed to parse NPC snapshots JSON: {}", e))?;

                for snapshot in snapshots {
                    let zone = self.get_or_create_zone_for_faction(&snapshot.faction);
                    let npc_id = self.spawn_or_update_npc_from_snapshot(snapshot, zone);
                    self.apply_lore_valence_modifiers(npc_id);
                }

                tracing::info!(
                    "Successfully loaded fresh NPC snapshots from public artifacts (attempt {}/{})",
                    attempt + 1,
                    max_retries + 1
                );
                return Ok(());
            }

            Ok(response) if response.status().as_u16() == 429 => {
                last_error = Some(format!("Rate limited (429) on attempt {}", attempt + 1));
            }

            Ok(response) if response.status().is_server_error() => {
                last_error = Some(format!(
                    "Server error {} on attempt {}",
                    response.status(),
                    attempt + 1
                ));
            }

            Ok(response) => {
                tracing::warn!(
                    "Public NPC artifact request returned non-retryable status {}. Falling back to deterministic generation.",
                    response.status()
                );
                self.spawn_default_lore_npcs().await;
                return Ok(());
            }

            Err(e) => {
                last_error = Some(format!("Network error on attempt {}: {}", attempt + 1, e));
            }
        }

        if attempt < max_retries {
            let delay = base_delay_ms * (1 << attempt);
            tracing::warn!(
                "Artifact fetch failed (attempt {}/{}). Retrying in {}ms...",
                attempt + 1,
                max_retries + 1,
                delay
            );
            sleep(Duration::from_millis(delay)).await;
        }
    }

    tracing::warn!(
        "Failed to fetch public NPC artifacts after {} attempts ({}). Falling back to high-quality deterministic lore generation.",
        max_retries + 1,
        last_error.unwrap_or_default()
    );

    self.spawn_default_lore_npcs().await;
    Ok(())
}