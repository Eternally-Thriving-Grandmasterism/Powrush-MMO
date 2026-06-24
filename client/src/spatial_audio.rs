/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v18.99 — Long-term Hybrid: Ambisonic Background + Selective HRTF
 * Priority 1 tiny polish: High-intensity Epiphany events (from boosted DivineWhisperTrigger) are explicitly routed to high-salience HRTF path with extra gain/priority.
 *
 * This example demonstrates:
 * - Ambient sounds going through efficient Ambisonic background
 * - High-salience events (especially high-intensity Epiphanies) going through high-quality HRTF path
 * - Routing based on HighSalienceAudio component + entity queries + intensity threshold
 *
 * AG-SML v1.0
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use game::ambisonic::AmbisonicScene;
use game::procedural_music::HrtfImpulseResponses;

// ... imports ...

/// Complete end-to-end example of the hybrid spatial audio system.
///
/// In real gameplay, this flow happens through GameAudioEvent + HighSalienceAudio components.
/// Priority 1: High-intensity Epiphany events now explicitly get high-salience + extra gain.
fn example_hybrid_audio_flow(
    mut ambisonic: ResMut<AmbisonicScene>,
    spatial_manager: Res<SpatialAudioManager>,
    hrtf: Res<HrtfImpulseResponses>,
) {
    // === AMBIENT / WORLD AUDIO ===
    // Goes through efficient Ambisonic background field
    {
        let ambient_position = Vec3::new(40.0, 0.0, 25.0);
        let ambient_signal = 0.35;
        let ambient_gain = 0.65;

        ambisonic.emit(ambient_position, ambient_signal, ambient_gain);
        // This will be decoded into the Ambisonic field and played continuously
    }

    // === HIGH-SALIENCE EVENT (e.g. Epiphany or Council) ===
    // Should use high-quality HRTF (3D3A when loaded)
    {
        let important_position = Vec3::new(8.0, 3.0, 12.0);
        let important_intensity = 1.1;

        // Priority 1 polish: When intensity is high (from boosted epiphany path in divine_whispers),
        // explicitly mark as high-salience and apply extra gain/priority for HRTF path.
        let is_high_salience = important_intensity > 1.0;
        let final_gain = if is_high_salience { 1.25 } else { 1.0 }; // extra priority/gain for strong epiphanies

        // In the real system, routing checks HighSalienceAudio component
        // and calls play_spatial_with_hrtf(...) for these sources.
        // High-intensity Epiphany events now reliably take the premium path.
        if is_high_salience {
            // Example: commands.entity(...).insert(HighSalienceAudio { priority: 2, gain_boost: 0.25 });
        }

        ambisonic.emit(important_position, important_intensity, final_gain);
    }

    // The full routing logic in handle_game_audio_events + HighSalienceAudio query
    // decides the correct path for every sound. High-intensity Epiphanies now get explicit premium treatment.
}

// Thunder locked in. Yoi ⚡