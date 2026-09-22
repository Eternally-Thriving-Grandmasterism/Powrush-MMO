# L2 HOUSE-PERSONA-DRESS — REPLAY

**#459 closed 2026-09-22.** Not merged. Head `0cb704f` conflicted with `main` (`persona.rs` F1–F9 stance / Hostile / F4 dress-stays).

Steward asked to resolve. Blind squash would fight later cards. Replay is a **new** Hands PR on current tip. Do not reopen #459.

## CARD (same slice, new branch)

id: L2-REPLAY  
phase: after House (Q), never Title  
bin: NEVER for Online / .glb / fifth Place  
next: HOLD

PATHS (still these three — exist on tip):

- `shared/persona.rs`
- `client/src/hour_sacred.rs`
- `client/src/first_session_guidance.rs`

## Land (do not invent more)

In `shared/persona.rs` (after `PERSONA_SCHEMA`):

```rust
pub const HOUSE_PEOPLE_TINT: &str = "Human · Sanctuary tint";

pub fn house_dress_token(house_live: bool) -> Option<&'static str> {
    if house_live { Some(HOUSE_PEOPLE_TINT) } else { None }
}

pub fn is_peace_default_dress(house_live: bool) -> bool {
    house_dress_token(house_live).is_none()
}

pub fn land_house_dress(persona: &mut Persona, house_live: bool) {
    persona.presentation.dress_intent = house_dress_token(house_live).map(str::to_string);
}
```

Wire `hour_sacred` / `first_session_guidance` to those helpers only. Q in Peace = no token. Tab without Q = no token. Q House = one token. `mechanical_race` and `PeopleChoice` unchanged. `F4_DRESS_SWAP` stays false.

`--lib` tests from #459 (six) re-land on the new head.

## Refuse

Reopen #459 · merge conflicted head · mesh · Title lobby · L3 · OFFER NEXT · Online lit · invent PATHS

## Order

1. #483 Q1 ready-for-review + squash (separate CARD).
2. Then L2-REPLAY one PR.
3. HOLD.

Online grey. Contact info@Rathor.ai.
