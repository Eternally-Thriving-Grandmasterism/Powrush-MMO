# Voice Acting Direction — Powrush-MMO

**Version:** 1.0  
**Last Updated:** June 13, 2026  
**Status:** Active Development

---

## 1. Core Principles

- **Asymmetric Vocal Identity**: Quellorian voices embody harmony, clarity, resonance, and hopeful elegance. Draek voices embody layered hivemind dissonance, oppression, consumption, and predatory menace.

- **Moral & Systemic Reactivity**: Voices dynamically shift based on Crownstone state, corruption levels, Resonance Burst outcomes, redemption progress, and RBE moral alignment.

- **Cinematic Integration**: Voice lines are triggered by and synchronized with major simulation events, VFX, and UI moments for maximum emotional impact.

- **Mercy-Aligned Depth**: Even in darkness there is the possibility of redemption; voices can show cracks of doubt, pain, or emerging harmony.

- **Performance at Scale**: Prioritized, spatialized, event-driven voice system suitable for large-scale MMORPG battles and mothership operations.

---

## 2. Faction Vocal Identities

### Quellorian / Aetherion Luminari

- **Overall Palette**: Clear, resonant, melodic, with subtle harmonic layering (choir-like overtones). Warm yet authoritative. Rising in power and beauty during Resonance events.

- **Emotional Range**: Calm wisdom → inspiring leadership → fierce protective resolve → transcendent hope during redemption moments.

- **Technical Notes**: Slight reverb + harmonic filter when near Ambrosian attunement or during Resonance Burst. Clean comms with elegant UI beeps.

### Draek Dominion

- **Overall Palette**: Multi-layered, distorted, guttural, with dissonant chorus effects. Many voices speaking slightly out of sync. Cold, commanding, increasingly unhinged as corruption grows.

- **Emotional Range**: Cold domination → ravenous hunger → psionic rage → rare moments of fractured doubt (especially during Crownstone sabotage or failed purifications).

- **Technical Notes**: Heavy distortion + pitch shifting when Crownstone integrity is low. Hivemind chorus swells during successful consumption or boarding.

---

## 3. Key Character Voice Profiles

### The Hivelord (Draek Supreme Leader)

- **Voice Type**: Deep male with heavy multi-voice layering (3–5 overlapping voices). Base voice is cold, precise, aristocratic menace. Crownstone influence adds echoing psionic reverb and increasing distortion.

- **Delivery**: Slow, deliberate, each word carrying weight. When enraged or using Crownstone powers, voices fracture and overlap chaotically.

- **Key Lines Examples**:
  - Idle / Command: "All will be consumed. All will serve the Dominion."
  - During Crownstone Trilemma (if player attempts Capture): "You dare touch what is mine? The Crownstone hungers for your defiance."
  - During failed purification: "Your harmony is a lie. Watch as your precious crystal breaks."

- **Technical Variables**: `hivelord_voice_distortion_level` (0.0–1.0) driven by `crownstone_integrity` and `hivelord_corruption_level`.

### The Auroral Sovereign (Elyndor the Harmonic)

- **Voice Type**: Warm, resonant baritone with subtle layered harmonic overtones (like a single voice + distant choir). Clear, inspiring, with rising power during Resonance events.

- **Delivery**: Calm authority that builds into passionate, transcendent calls during Resonance Burst or major redemption moments.

- **Key Lines**:
  - Resonance Burst activation: "Let the aurora rise! Harmony will not be silenced!"
  - During successful Crownstone Capture & Repurpose: "Even the darkest stone can be returned to the light. We choose mercy."

### The High Resonance Keeper (Veyra of the Crystal Choir)

- **Voice Type**: Ethereal female voice with crystalline, choir-backed quality. Very melodic, almost singing in stressful moments.

- **Role**: Explains attunement, warns of Discordant corruption, guides redemption paths.

### The Grand Fleet Warden (Kaelith Starweaver)

- **Voice Type**: Strong, clear, tactical female/male voice with military precision. Less harmonic layering than Elyndor, more focused command tone.

### The Nexus Archivist (Scholar-Lord Thalorien)

- **Voice Type**: Measured, slightly ancient, wise male voice. Calm even in crisis. Provides lore and strategic insight on Crownstone history.

### Generic / Unit Voices

- **Quellorian Pilots & Crew**: Clean, professional, with slight harmonic filter near Ambrosians. "Resonance aligned. Engaging."

- **Draek Units**: Layered, aggressive, short commands mixed with hunger sounds. "Consume. Obey. Spread."

---

## 4. Contextual Voice Systems

### Major Event Triggers

- **Resonance Burst**: Massive harmonic swell + Elyndor’s empowered call. Nearby Quellorian units cheer with rising harmony.

- **Crownstone Trilemma Decision Screen**: Special voice lines from both Elyndor and The Hivelord reacting to player choice (Destroy / Capture / Sabotage).

- **Successful Boarding on TAUN**: Hivelord taunts with growing distortion.

- **Successful Redemption (any path)**: Ambrosian crystalline chimes + Veyra’s hopeful guidance + possible Elyndor blessing.

- **Discordant Outbreak**: Terrifying dissonant shrieks + Hivelord’s mocking laughter layered in.

- **Hivelord Suit Activation / High Corruption**: Increasing vocal fracture and multiple overlapping voices.

### Dynamic Moral Reactivity

Voice lines and delivery change based on:
- Current `crownstone_owner` and `crownstone_corruption_level`
- Player’s RBE moral standing (high harmony = warmer Quellorian voices; high consumption = more aggressive Draek voices)
- Whether major redemption events have occurred

---

## 5. Technical Implementation Notes

### Recommended Architecture (Bevy + Kira Audio)

- Use `bevy_kira_audio` for advanced control (spatial audio, dynamic mixing, pitch/timbre modulation).
- Central `VoiceDirector` resource that listens to events from `WorldSimulationState`, `CrownstoneState`, `ResonanceNetworkState`, `DraekHivemindState`, boarding events, etc.
- Event-driven: `VoiceEvent { character: VoiceCharacter, context: VoiceContext, intensity: f32 }`
- Priority system: Mothership leaders > Key characters > Generic units. Interrupt lower priority lines when higher ones trigger.
- Spatial audio: 3D positioned voices for boarding parties, dogfights, and mothership interiors.
- Dynamic DSP: Real-time low-pass / distortion filters driven by corruption or resonance levels (can be done via Kira or custom DSP chain).

### Data Structure Recommendations

```rust
#[derive(Resource)]
pub struct VoiceBank {
    pub hivelord_lines: HashMap<VoiceContext, Vec<String>>,
    pub auroral_sovereign_lines: HashMap<VoiceContext, Vec<String>>,
    // ... other characters
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoiceContext {
    IdleCommand,
    ResonanceBurstActivation,
    CrownstoneTrilemmaChoice { path: TrilemmaPath },
    BoardingSuccess { target: BoardingTarget },
    RedemptionSuccess { path: RedemptionPath },
    CorruptionWarning,
    HivelordRage,
    // etc.
}
```

### Integration Hooks

- `simulation_integration.rs`: Fire `VoiceEvent` on state changes (Crownstone integrity shifts, Resonance Burst completion, Hivelord suit damage, Ambrosian corruption spread, etc.).
- `rbe_engine.rs`: Moral standing influences which voice lines are available or how they are delivered (warmer vs colder tone).
- UI/VFX sync: Voice lines can trigger or be triggered by specific VFX (auroral waves, Crownstone glow changes, Discordant crystal cracking).

### Localization & Accessibility

- Full support for 11+ languages (consistent with Ra-Thor multilingual approach).
- Subtitle system tightly synced with voice timing.
- Option for reduced voice layering / distortion for accessibility.

---

## 6. Development Priorities

1. Define full voice line database structure and initial recording script for key characters + generic units.
2. Implement `VoiceDirector` resource + event system in `simulation_integration.rs`.
3. Create dynamic DSP chain for real-time voice modulation (distortion, harmonic layering, pitch shift).
4. Record / generate placeholder voice lines for major events (Resonance Burst, Crownstone Trilemma, Hivelord taunts).
5. Integrate spatial audio positioning for boarding and dogfight scenarios.
6. Add moral reactivity layer (voice tone shifts based on RBE standing and Crownstone state).
7. Full localization pipeline and subtitle synchronization.

**End of Document**