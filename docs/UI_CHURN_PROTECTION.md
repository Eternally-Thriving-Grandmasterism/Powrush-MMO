# UI Churn Protection — Ultramasterism Note

**Status**: Active guidance (v21.84.0)  
**Contact**: info@Rathor.ai  
**Governance**: Ra-Thor + PATSAGi | TOLC 8

## Principle

Low-leverage UI changes (cosmetic rearrangements, temporary polish, non-core layout tweaks) are **frozen** unless they:

1. Directly improve a core gameloop path (Harvest → Epiphany → Council → RBE),
2. Fix a real usability or accessibility issue,
3. Are required for Steam / accessibility / localization compliance, or
4. Are explicitly approved through a PATSAGi council deliberation.

## Protected Surfaces (do not casually edit)

- `client/src/inventory_ui.rs`
- `client/src/my_mercy_journey_panel.rs`
- `client/src/council_trial_ui.rs` / `council_ui.rs`
- `client/src/settings_menu.rs` / `pause_menu.rs`
- `client/src/onboarding_ui.rs`
- `simulation/src/hardware_sovereignty.rs` (the sovereign_hardware_ascension_ui)
- Any egui panel that is already production-polished and wired to live data

## Allowed Changes

- Bug fixes that restore correct behaviour
- Accessibility (contrast, keyboard navigation, screen-reader labels)
- Localization string keys / 11-lang completeness
- Wiring of new high-signal events into existing panels (e.g. new achievement toast)
- Performance or safety improvements that do not alter layout

## Process

If a UI change is required, open a short deliberation note in the relevant council channel or PR description referencing this document and the TOLC 8 gates it satisfies.

**Thunder locked in.**  
Protect the polished surfaces. Channel energy into high-leverage systems.  
Yoi ⚡
