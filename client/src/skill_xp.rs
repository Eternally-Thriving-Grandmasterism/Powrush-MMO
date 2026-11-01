//! SKILL-XP-1 — per-skill XP and levels for gather, tend, walk and sprint.
//! Offline only. Saves ride the existing SessionBlob path.

use bevy::prelude::*;

use crate::difficulty::{clamp_xp_pct, combat_xp, AreaDifficulty, DifficultyTier, COMBAT_XP_PCT};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hex_travel::HexTravelState;
use crate::first_session_guidance::body_step_xz;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use crate::living_body::LivingBody;

pub const XP_PER_GATHER: u32 = 10; // playtest placeholder
pub const XP_PER_TEND: u32 = 12; // playtest placeholder
pub const XP_PER_METRE: u32 = 1; // playtest placeholder (walk and sprint, whole metres)
pub const XP_CURVE_STEP: u32 = 100; // playtest placeholder: level n -> n+1 costs step * n
pub const LEVEL_CAP: u32 = 50; // playtest placeholder
pub const XP_CAP: u32 = XP_CURVE_STEP * (LEVEL_CAP - 1) * LEVEL_CAP / 2;

/// Player-facing names, one per skill. Internal ids: Gather, Tend, Walk, Sprint.
pub const SKILL_NAMES: [&str; 5] = ["[NAME TBD — Sherif's words]", "[NAME TBD — Sherif's words]", "[NAME TBD — Sherif's words]", "[NAME TBD — Sherif's words]", "[NAME TBD]"];
pub const GATHER: usize = 0;
pub const TEND: usize = 1;
pub const WALK: usize = 2;
pub const SPRINT: usize = 3;
/// COMBAT-CORE-1: landed hits only (`combat_core.rs`).
pub const COMBAT: usize = 4;

#[derive(Resource, Debug, Default)]
pub struct SkillXp {
    pub xp: [u32; 5],
    metres: [f32; 5],
    /// COMBAT-XP-PACE-1 — UTC day key of `combat_day_hits`. Saved because every paying hit
    /// rewrites the pulse on `FirstHarvestEpiphany` (combat_core :399-400), which `mark_dirty` watches.
    pub combat_day: u32,
    /// COMBAT-XP-PACE-1 — paying combat hits on `combat_day`.
    pub combat_day_hits: u32,
}

#[derive(Component)]
pub struct SkillsPanelText;

/// Total XP to stand at `level` (level 1 = 0). Saturating.
pub fn xp_for_level(level: u32) -> u32 {
    let n = level.clamp(1, LEVEL_CAP) - 1;
    XP_CURVE_STEP.saturating_mul(n.saturating_mul(n + 1) / 2)
}

pub fn level_for_xp(xp: u32) -> u32 {
    (2..=LEVEL_CAP).filter(|&l| xp >= xp_for_level(l)).count() as u32 + 1
}

/// XP gained from a success counter (only :636 / :704 move it).
pub fn success_gain(prev: u32, now: u32, per: u32) -> u32 {
    now.saturating_sub(prev).saturating_mul(per)
}

/// Same rule as `living_body.rs:146` `want_sprint`.
pub fn want_sprint(sprint: bool, movement: Vec2, can_sprint: bool) -> bool {
    sprint && movement.length_squared() > 0.04 && can_sprint
}

/// (walk, sprint) metres for one capped step. Sprint distance never also walks.
pub fn split_step(step: f32, sprinting: bool) -> (f32, f32) {
    match (step > 0.0 && step.is_finite(), sprinting) {
        (false, _) => (0.0, 0.0),
        (true, true) => (0.0, step),
        (true, false) => (step, 0.0),
    }
}

/// Seed gather/tend from lifetime counters only when the field is absent.
pub fn skills_on_load(saved: Option<[u32; 4]>, harvests: u32, tends: u32) -> [u32; 4] {
    let gather = harvests.saturating_mul(XP_PER_GATHER);
    let tend = tends.saturating_mul(XP_PER_TEND);
    saved
        .unwrap_or([gather, tend, 0, 0])
        .map(|xp| xp.min(XP_CAP))
}

/// COMBAT-CORE-1: the 4-wide save plus `combat_xp` (absent = 0).
pub fn skills_with_combat_on_load(
    saved: Option<[u32; 4]>,
    combat_xp: Option<u32>,
    harvests: u32,
    tends: u32,
) -> [u32; 5] {
    let [g, t, w, s] = skills_on_load(saved, harvests, tends);
    [g, t, w, s, combat_xp.unwrap_or(0).min(XP_CAP)]
}

/// COMBAT-XP-PACE-1 — paying combat hits per UTC day (`epoch_secs / 86400`; rolls 8 PM ET
/// in summer, 7 PM ET in winter). Council reading of COMBAT_AND_PVP.md:73. Playtest-tunable;
/// Sherif may veto.
pub const COMBAT_PAYING_HITS_PER_DAY: u32 = 20;

/// COMBAT-XP-PACE-1 — UTC day key: `epoch_secs / 86400`.
pub fn day_key(epoch_secs: u64) -> u32 {
    (epoch_secs / 86_400).min(u64::from(u32::MAX)) as u32
}

/// COMBAT-XP-PACE-1 — a later day starts at 0 hits; the same or an earlier day keeps the count.
pub fn roll_day(saved_day: u32, saved_hits: u32, today: u32) -> (u32, u32) {
    if today > saved_day {
        (today, 0)
    } else {
        (saved_day, saved_hits)
    }
}

/// TOTAL-LEVEL-2 — the one level-up line, shared by every skill.
pub fn level_up_line(skill: usize, level: u32) -> String {
    format!("{} · level {level}", SKILL_NAMES[skill])
}

impl SkillXp {
    /// Saturating add, clamped at the cap. Returns the new level on a level-up.
    pub fn gain(&mut self, skill: usize, amount: u32) -> Option<u32> {
        let before = level_for_xp(self.xp[skill]);
        self.xp[skill] = self.xp[skill].saturating_add(amount).min(XP_CAP);
        let after = level_for_xp(self.xp[skill]);
        (after > before).then_some(after)
    }

    /// Bank metres; whole metres become XP.
    pub fn gain_metres(&mut self, skill: usize, metres: f32) -> Option<u32> {
        self.metres[skill] += metres;
        let whole = self.metres[skill].floor();
        self.metres[skill] -= whole;
        (whole >= 1.0).then(|| self.gain(skill, (whole as u32).saturating_mul(XP_PER_METRE)))?
    }
}

/// COMBAT-XP-PACE-1 — the only wall-clock read for the cap. Reads through `Deref` and
/// writes `SkillXp` only when the UTC day moves forward, so a no-op frame stays unchanged.
fn roll_combat_day(skills: Option<ResMut<SkillXp>>) {
    let Some(mut skills) = skills else {
        return;
    };
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    let today = day_key(now.map_or(0, |d| d.as_secs()));
    let (day, hits) = roll_day(skills.combat_day, skills.combat_day_hits, today);
    if day != skills.combat_day {
        (skills.combat_day, skills.combat_day_hits) = (day, hits);
    }
}

pub struct SkillXpPlugin;

impl Plugin for SkillXpPlugin {
    fn build(&self, app: &mut App) {
        let panel = update_skills_panel.run_if(resource_changed::<SkillXp>);
        app.init_resource::<SkillXp>()
            .add_systems(PreUpdate, roll_combat_day)
            .add_systems(PostUpdate, (award_skill_xp, panel).chain());
    }
}

/// DIFFICULTY-REWARDS-2. Walk metres scale by the combat table, exactly 1.0 on Sheltered.
pub fn walk_factor(tier: DifficultyTier) -> f32 { clamp_xp_pct(COMBAT_XP_PCT[tier as usize]) as f32 / 100.0 }

/// Own system: runs even when first-session guidance is dismissed.
#[allow(clippy::too_many_arguments)]
fn award_skill_xp(
    time: Res<Time>,
    fixed_time: Option<Res<Time<Fixed>>>,
    presence: Option<Res<SoftPresence>>,
    input: Option<Res<PlayerInput>>,
    body: Option<Res<LivingBody>>,
    mut skills: ResMut<SkillXp>,
    mut state: ResMut<FirstHarvestEpiphany>,
    mut last: Local<(u32, u32, Option<Vec2>)>,
    (travel, diff): (Option<Res<HexTravelState>>, Option<Res<AreaDifficulty>>),
) {
    let tier = match (travel, diff) { (Some(t), Some(d)) => d.tier(t.current), _ => DifficultyTier::Sheltered };
    let (h, t) = (state.harvests_this_session, state.tends_this_session);
    let gather = success_gain(last.0, h, XP_PER_GATHER);
    let tend = success_gain(last.1, t, XP_PER_TEND);
    let mut ups = vec![
        (GATHER, skills.gain(GATHER, combat_xp(tier, gather))),
        (TEND, skills.gain(TEND, tend)),
    ];
    (last.0, last.1) = (h, t);
    if let Some(p) = presence {
        let xz = Vec2::new(p.position.x, p.position.z);
        let fixed_dt = fixed_time.map_or(0.0, |c| c.timestep().as_secs_f32());
        let step = body_step_xz(last.2, xz, time.delta_secs(), fixed_dt);
        last.2 = Some(xz);
        let sprinting = matches!((input, body), (Some(i), Some(b))
            if want_sprint(i.sprint, i.movement, b.can_sprint()));
        let (walk, sprint) = split_step(step, sprinting);
        ups.push((WALK, skills.gain_metres(WALK, walk * walk_factor(tier))));
        ups.push((SPRINT, skills.gain_metres(SPRINT, sprint)));
    }
    let now = time.elapsed_secs_f64();
    for (skill, level) in ups.into_iter().filter_map(|(s, l)| Some((s, l?))) {
        state.pulse_line = level_up_line(skill, level);
        state.pulse_until = now + 4.2;
    }
}

fn update_skills_panel(skills: Res<SkillXp>, mut texts: Query<&mut Text, With<SkillsPanelText>>) {
    let lines: Vec<String> = (0..SKILL_NAMES.len())
        .map(|i| format!("{} · Lv {}", SKILL_NAMES[i], level_for_xp(skills.xp[i])))
        .collect();
    let lines = with_total_line(lines, &skills.xp);
    for mut text in &mut texts {
        **text = format!("SKILLS\n{}", lines.join("\n"));
    }
}

/// Sum of every skill level; only used to clamp the total, never shown.
pub const TOTAL_CAP: u32 = SKILL_NAMES.len() as u32 * LEVEL_CAP;

/// Saturating sum of `level_for_xp` over the skill array, clamped at `TOTAL_CAP`.
pub fn total_level(xp: &[u32]) -> u32 {
    xp.iter()
        .fold(0u32, |sum, &x| sum.saturating_add(level_for_xp(x)))
        .min(TOTAL_CAP)
}

/// Appends the `Total · Lv {n}` line as the panel's last line.
pub fn with_total_line(mut lines: Vec<String>, xp: &[u32]) -> Vec<String> {
    lines.push(format!("Total · Lv {}", total_level(xp)));
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::first_harvest_epiphany::hold_e_tend_blocked;

    fn step(prev: Vec2, now: Vec2, sprinting: bool) -> (f32, f32) {
        split_step(
            body_step_xz(Some(prev), now, 1.0 / 60.0, 1.0 / 60.0),
            sprinting,
        )
    }

    #[test]
    fn idle_earns_zero() {
        assert_eq!(step(Vec2::ONE, Vec2::ONE, false), (0.0, 0.0));
        assert_eq!(success_gain(3, 3, XP_PER_GATHER), 0);
    }

    #[test]
    fn wall_push_earns_zero() {
        // Sprint held into a wall: input moves, position does not.
        let sprinting = want_sprint(true, Vec2::Y, true);
        assert_eq!(step(Vec2::ONE, Vec2::ONE, sprinting), (0.0, 0.0));
    }

    #[test]
    fn teleport_step_above_cap_earns_zero() {
        assert_eq!(step(Vec2::ZERO, Vec2::X * 50.0, false), (0.0, 0.0));
        assert_eq!(step(Vec2::ZERO, Vec2::X * 50.0, true), (0.0, 0.0));
    }

    #[test]
    fn sprint_distance_gives_no_walk_xp() {
        let (walk, sprint) = step(Vec2::ZERO, Vec2::X * 0.25, true);
        assert_eq!(walk, 0.0);
        let mut skills = SkillXp::default();
        for _ in 0..20 {
            skills.gain_metres(SPRINT, sprint);
            skills.gain_metres(WALK, walk);
        }
        assert_eq!((skills.xp[WALK], skills.xp[SPRINT]), (0, 5 * XP_PER_METRE));
        assert!(!want_sprint(true, Vec2::ZERO, true) && !want_sprint(true, Vec2::Y, false));
    }

    #[test]
    fn cooldown_blocked_tend_earns_zero() {
        // resolve_tend returns before :704 when blocked; the counter holds.
        assert!(hold_e_tend_blocked(1, 1, 10.0, 10.5));
        assert_eq!(success_gain(1, 1, XP_PER_TEND), 0);
    }

    #[test]
    fn out_of_range_gather_earns_zero() {
        // resolve_take returns before :636 when out of range; the counter holds.
        assert_eq!(success_gain(4, 4, XP_PER_GATHER), 0);
        assert_eq!(success_gain(4, 5, XP_PER_GATHER), XP_PER_GATHER);
    }

    #[test]
    fn cap_saturates_without_wrap() {
        let mut skills = SkillXp::default();
        assert_eq!(skills.gain(TEND, XP_CURVE_STEP), Some(2));
        skills.gain(GATHER, u32::MAX);
        assert_eq!(skills.gain(GATHER, u32::MAX), None);
        assert_eq!(skills.xp[GATHER], XP_CAP);
        assert_eq!((level_for_xp(u32::MAX), level_for_xp(0)), (LEVEL_CAP, 1));
        assert_eq!(skills_on_load(None, u32::MAX, u32::MAX)[TEND], XP_CAP);
    }

    #[test]
    fn total_level_zero_cap_and_saturating() {
        let count = SKILL_NAMES.len() as u32;
        assert_eq!(total_level(&[0; SKILL_NAMES.len()]), count * level_for_xp(0));
        assert_eq!(total_level(&[0; SKILL_NAMES.len()]), 5);
        assert_eq!(total_level(&[XP_CAP; SKILL_NAMES.len()]), TOTAL_CAP);
        assert_eq!(total_level(&[u32::MAX; SKILL_NAMES.len()]), TOTAL_CAP);
        assert_eq!(total_level(&vec![u32::MAX; 200]), TOTAL_CAP);
    }

    #[test]
    fn panel_last_line_is_total_at_zero_xp() {
        let lines = with_total_line(vec!["SKILLS".into()], &[0; SKILL_NAMES.len()]);
        assert_eq!(lines.last().map(String::as_str), Some("Total · Lv 5"));
    }

    #[test]
    fn combat_wrapper_absent_is_zero_and_some_round_trips() {
        let v1 = skills_with_combat_on_load(Some([1, 2, 3, 4]), None, 9, 9);
        assert_eq!(v1, [1, 2, 3, 4, 0]);
        assert_eq!(skills_with_combat_on_load(None, Some(77), 0, 0)[COMBAT], 77);
        assert_eq!(skills_with_combat_on_load(None, Some(u32::MAX), 0, 0)[COMBAT], XP_CAP);
    }

    #[test]
    fn day_key_rolls_at_utc_midnight() {
        assert_eq!((day_key(0), day_key(86_399), day_key(86_400)), (0, 0, 1));
    }

    #[test]
    fn roll_day_forward_resets_same_and_backwards_keep() {
        assert_eq!(roll_day(5, 20, 6), (6, 0), "forward");
        assert_eq!(roll_day(5, 20, 5), (5, 20), "same day");
        assert_eq!(roll_day(5, 20, 4), (5, 20), "backwards clock keeps the count");
    }
    #[test]
    fn rewards2_gather_scales_per_tier_through_the_system() {
        use bevy::ecs::system::RunSystemOnce;
        use crate::difficulty::TIERS;
        let got = TIERS.map(|t| {
            let mut world = World::new();
            world.insert_resource(Time::<()>::default());
            world.insert_resource(SkillXp::default());
            world.insert_resource(FirstHarvestEpiphany { harvests_this_session: 1, ..Default::default() });
            world.insert_resource(HexTravelState { current: shared::hex_travel::PlaceId::Heartwood });
            world.insert_resource(AreaDifficulty { default: t, ..Default::default() });
            world.run_system_once(award_skill_xp).unwrap();
            world.resource::<SkillXp>().xp[GATHER]
        });
        assert_eq!(got, [10, 12, 13, 15]);
        assert_eq!(TIERS.map(|t| combat_xp(t, XP_PER_GATHER)), [10, 12, 13, 15]);
    }

    #[test]
    fn rewards2_walk_1000m_per_tier_zero_idle_and_sprint_at_base() {
        use crate::difficulty::TIERS;
        assert_eq!(walk_factor(DifficultyTier::Sheltered), 1.0, "Sheltered is exactly 1.0");
        assert!(TIERS.iter().all(|&t| (1.0..=1.75).contains(&walk_factor(t))), "never over the hard max");
        for (t, want) in TIERS.into_iter().zip([1000u32, 1150, 1300, 1500]) {
            let mut skills = SkillXp::default();
            for _ in 0..4000 {
                skills.gain_metres(WALK, 0.25 * walk_factor(t));
                skills.gain_metres(SPRINT, 0.25);
            }
            let walk = skills.xp[WALK];
            assert!(walk.abs_diff(want) <= 1, "{t:?} walk {walk} vs {want}");
            if t == DifficultyTier::Sheltered { assert_eq!(walk, 1000); }
            assert_eq!(skills.xp[SPRINT], 1000, "{t:?} sprint stays at base");
        }
        let mut idle = SkillXp::default();
        for _ in 0..1000 { idle.gain_metres(WALK, 0.0 * walk_factor(DifficultyTier::Unbroken)); }
        assert_eq!(idle.xp[WALK], 0, "0 m on Unbroken pays 0");
    }
}
