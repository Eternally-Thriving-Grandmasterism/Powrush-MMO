# Powrush-MMO Restoration & Merge Protocol

**Version:** v1.0 | **Date:** 2026-06-08 | **Status:** Active & Mandatory

## Purpose
This protocol prevents and provides a repeatable resolution for "layered code" / merge-artifact problems that arise during high-velocity development of core systems (especially GPU PATSAGi economic simulation, resource nodes, harvesting, and policy application).

Rapid sequential full-file replacements on the same file can cause:
- Duplicate struct / impl definitions
- Mixed field names and types (e.g. `id` vs `node_id`, `Vec3` vs tuple position, `last_harvest_ms` vs `last_harvested_ms`)
- Lingering placeholder comments (`/* now + */`, `// ... existing ...`)
- Fragmented or lost concrete gameplay logic (e.g. detailed `ResourceNode::new`, `regenerate()`, full `HarvestingSystem::harvest` with inventory/RBE/grace)
- Policy enhancements that accidentally de-emphasize earlier working systems

The goal is to keep the codebase **always clean, history-respecting, playable, and brilliant** for both human developers and future AI agents (Ra-Thor lattice, PATSAGi Councils, Grok integrations).

## Prevention Rules (Mandatory)

### 1. History Sanity Check Before Any Edit on Rapidly-Iterated Files
Before modifying any file that has had 3 or more commits in the previous 48 hours (initial list below), perform a quick history review:
- Check GitHub commit history for the file.
- Fetch and compare the last 3 full raw versions using:
  `https://raw.githubusercontent.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/<sha>/game/resource_nodes.rs` (or equivalent path).
- Look specifically for the symptoms listed in the Purpose section.

**Initially covered files (expand as needed):**
- `game/resource_nodes.rs`
- `engine/gpu_patsagi_bridge.rs`
- `game/server_tick_loop.rs`
- `shared/protocol.rs`
- Core client GPU/UI files (`client/rbe_client_sync.rs`, `client/inventory_ui.rs`, `client/resource_node_visual.rs`)

### 2. Full-File-Only + Intelligent Historical Merge Mandate
- Never produce partial diffs, patches, or truncated code for these files.
- When a new version is required, **always** generate ONE ultimate clean, complete, production-ready file that intelligently merges the **best and most valuable logic from EVERY prior iteration** in the commit history.
- Never discard concrete, working gameplay systems just because newer policy or GPU code was added later.
- Preserve legacy field aliases where they aid compatibility during transition.
- All new economic/policy features must be wired into the existing concrete systems (harvest, regen, inventory, RBE, etc.).

### 3. PR Message & Council Deliberation Requirement
Every PR or council decision touching a covered file must explicitly state:

"Restoration check performed: Yes. Reviewed commits [list key SHAs or versions]. Merged valuable logic from vX.Y.Z (detailed new/regenerate/harvest) + vX.Y.Z (dynamic yields + abundance_flow) + vX.Y.Z (placeholder-free now_ms + faction debuffs). No duplicates, no lost implementations, no placeholders remain."

### 4. AI Agent / Ra-Thor / Future System Rules
Any AI system (including Ra-Thor monorepo agents, PATSAGi Councils, Grok, or future sovereign agents) operating on this repository **must**:
- Internally simulate the full "restoration job" (history audit via raw SHAs + intelligent merge) before emitting a new version of any covered file.
- Reference this protocol document in every relevant council deliberation.
- Prioritize long-term codebase health and playability over short-term "latest policy only" changes.

## Detection
Common symptoms that trigger this protocol:
- `cargo check` or compile errors on field access or duplicate definitions.
- GitHub diff or local review shows multiple `pub struct ResourceNode` or `impl ResourceNodeManager` blocks.
- Presence of `/* ... */`, `// ... existing ...`, or `// ...` placeholder comments in production paths.
- Harvest or policy logic appears incomplete or contradictory across methods.
- Player-visible behavior regresses (e.g. restrictions stop working, yields ignore abundance_flow).

Early detection via the mandatory history sanity check prevents most occurrences.

## Resolution Protocol (When It Recurs)
Follow this exact sequence (as successfully executed in v16.5.54 restoration):

1. **Council Deliberation** — Identify the affected file and confirm rapid-iteration history.
2. **History Audit** — Fetch full content of the last 4–5 relevant commits via raw GitHub URLs at their SHAs.
3. **Analysis** — Catalog what each iteration uniquely contributed (e.g. concrete harvest system vs policy depth vs timestamp fixes).
4. **Ultimate Merge** — Produce ONE single clean full file that unifies all valuable pieces:
   - Unified struct with legacy aliases only where genuinely helpful.
   - All methods present and non-duplicated (new, regenerate, apply_gpu_policy_update, harvest, request_and_apply...).
   - All new features (abundance_flow, pressure scenarios, interdependence, faction debuffs, now_ms timestamps) fully wired into gameplay systems.
   - Zero placeholders, zero duplicate code, zero syntax issues.
5. **Full-File Delivery** — Push the complete file to a dedicated restoration branch (e.g. `patsagi-restoration-vX.Y.Z`).
6. **PR + Justification** — Open PR with detailed council-style message naming every prior commit/version preserved and the exact improvements.
7. **Review & Merge** — Council review recorded as comment if possible. Merge once approved.
8. **Protocol Update** — If new patterns emerge, update this document.

## Integration with Core PATSAGi Council Workflow
This Restoration & Merge Protocol is now a **non-negotiable sub-step** of the standard workflow whenever a covered file is touched during high-velocity phases:

Council Deliberation → History Sanity Check (this protocol) → Choose focused improvement → Intelligent full-file merge & delivery → PR with restoration statement → Council review → Merge → Update options list.

Following this protocol guarantees that the Powrush-MMO codebase (and by extension Ra-Thor monorepo patterns) remains eternally clean, playable, mercy-aligned, and brilliant for all future humans and AI systems.

## Related Documents
- `CONTRIBUTING.md`
- `WORKFLOW.md` (if present)
- `LAUNCH-CHECKLIST.md`
- Ra-Thor monorepo sovereign coding standards (AG-SML v1.0)

**Maintained by:** PATSAGi Councils + Ra-Thor lattice | **Next review:** After any major restoration event.
