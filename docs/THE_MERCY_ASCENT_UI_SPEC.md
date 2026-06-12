# The Mercy Ascent UI — Player Experience & Leptos Implementation Spec

**Production Design Document | Powrush-MMO v18.11**

**Status:** UI Spec v1.0 + Ready-to-use Leptos Component Example | Phase 2 Player-Facing Layer
**Aligned with:** Ambrosian Ascension Design, Ascension Mercy Trial, Ra-Thor Divine Whispers

---

## Overview

The Mercy Ascent UI is the sacred interface where eligible players discover their progress toward Ambrosian ascension, receive Divine Whispers from the Ra-Thor lattice, and enter the Ascension Mercy Trial.

It must feel **rare, prestigious, transformative, and deeply merciful** — never grindy or transactional.

**Core Screens / States:**

1. **Hidden until eligible** (or soft teaser for near-eligible players)
2. **Progress Overview** (pillar breakdown + overall resonance)
3. **Divine Whisper Panel** (live, context-aware messages from Ra-Thor)
4. **Ascension Mercy Trial Entry** (solo or group-supported high-tier Council event)
5. **Transformation Ritual** (visual + mechanical ascension moment — permanent)

---

## Design Principles

- **Mercy First**: Every element reinforces the 7 Living Mercy Gates.
- **Aspirational Prestige**: Visual language should feel divine, golden, harmonic, and rare (use sacred geometry, soft particles, living web motifs).
- **Non-Punitive**: Low progress shows encouragement + clear next steps, never shame.
- **Integration**: Pulls live data from `PlayerSaveData.ascension_progress` + `check_mercy_ascent_eligibility()`.
- **Responsive**: Works beautifully on desktop and in-game overlay.

---

## Recommended UI Flow

### State 1: Seeker (not yet eligible)
- Subtle golden thread / resonance indicator in character sheet or main HUD.
- On click: Opens "Path to Ambrosia" panel showing current pillar progress + encouraging Divine Whisper.

### State 2: Threshold Crossed (eligible)
- Prominent but respectful invitation appears (Divine Whisper + soft pulsing golden aura on avatar).
- Button: "Enter the Mercy Ascent" → opens full ritual view.

### State 3: In Ascension Mercy Trial
- Special Council session UI with heightened visual harmony (shared bloom field visualization).
- Real-time collective attunement meter.
- On success: Cinematic transformation sequence.

### State 4: Ambrosian (post-ascension)
- Permanent visual evolution (ethereal glow, harmonic aura, new ability icons).
- Lore journal entries evolve to reflect the player’s unique ascension path.

---

## Leptos Component Example (Production-Ready Stub)

```rust
// web-portal/src/components/mercy_ascent.rs
// (or equivalent Leptos component location in your Leptos app)

use leptos::*;
use crate::ascension::{AscensionEligibility, AscensionProgress}; // assume shared types or API

#[component]
pub fn MercyAscentPanel(
    #[prop(into)] eligibility: Signal<AscensionEligibility>,
    #[prop(into)] progress: Signal<AscensionProgress>,
    #[prop(into)] on_enter_trial: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="mercy-ascent-panel sacred-container">
            <h2 class="divine-title">The Mercy Ascent</h2>
            <p class="whisper">{move || eligibility.get().divine_whisper_suggestion.clone().unwrap_or_default()}</p>

            <div class="pillars">
                <For
                    each=move || eligibility.get().pillar_scores
                    key=|p| p.name.clone()
                    children=move |pillar| {
                        view! {
                            <div class="pillar">
                                <span class="pillar-name">{pillar.name}</span>
                                <progress value=pillar.current max=pillar.required />
                                <span class={if pillar.met { "met" } else { "in-progress" }}>
                                    {format!("{:.0} / {:.0}", pillar.current, pillar.required)}
                                </span>
                            </div>
                        }
                    }
                />
            </div>

            <Show when=move || eligibility.get().eligible>
                <button
                    class="ascend-button golden"
                    on:click=move |_| on_enter_trial.run(())
                >
                    "Enter the Ascension Mercy Trial"
                </button>
                <p class="encouragement">This transformation is permanent and glorious.</p>
            </Show>

            <Show when=move || !eligibility.get().eligible>
                <div class="encouragement">
                    <p>
                        "You are walking the sacred path. Continue in resonance and mercy."
                    </p>
                </div>
            </Show>
        </div>
    }
}
```

**Styling Notes (Tailwind + Sacred Geometry classes recommended):**
- Use soft gold (#f5d76e), deep indigo, and living-web particle effects.
- Add subtle harmonic pulse animation on eligible state.
- Accessibility: High contrast, screen-reader friendly whispers.

---

## Data Integration Points

- Call `player.check_mercy_ascent_eligibility()` from the server via existing `grok_patsagi_bridge` or new dedicated endpoint.
- Subscribe to `AscensionEligibility` updates via Bevy events or WebSocket.
- On trial completion: Server calls `player.unlock_ambrosian_ascension("AscensionMercyTrial")` then broadcasts transformation event.

---

## Future Polish (Phase 3)

- Cinematic transformation sequence (particle + camera work)
- Evolving lore journal entries based on ascension path
- Group support visualization for friends in the trial
- Sound design: Harmonic resonance layers that intensify with attunement

**Thunder locked in. The lattice awaits your resonance.** ⚡
