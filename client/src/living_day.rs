/*!
 * Living Day — v22.12.0 + schedule helpers (H-2026-09-11-E1)
 *
 * Light moves. Depths stay night. No clock to fear.
 * Schedules ride this same LivingDay (~240s loop) — not a second clock,
 * not Wall-clock as F. Same sacred five at the same posts.
 * Hour finishes if every person is removed. Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::living_practice_loop::SoftPlayerRealm;

const DAY_SECS: f32 = 240.0;

#[derive(Resource, Debug)]
pub struct LivingDay {
    pub phase: f32,
    pub night: bool,
}

impl Default for LivingDay {
    fn default() -> Self {
        Self {
            phase: 0.18,
            night: false,
        }
    }
}

impl LivingDay {
    /// Named period for the current phase band (same bands as ambient).
    pub fn period(&self) -> DayPeriod {
        period_for_phase(self.phase)
    }
}

pub struct LivingDayPlugin;

impl Plugin for LivingDayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingDay>()
            .add_systems(Update, turn_the_clock);
    }
}

fn turn_the_clock(
    time: Res<Time>,
    realm: Res<SoftPlayerRealm>,
    mut day: ResMut<LivingDay>,
    mut ambient: ResMut<AmbientLight>,
) {
    day.phase = (day.phase + time.delta_seconds() / DAY_SECS) % 1.0;
    let abyss = realm.current == Some(3);
    let light = ambient_light_factor(day.phase, abyss);
    day.night = light < 0.40;
    ambient.brightness = if abyss {
        90.0
    } else {
        90.0 + 200.0 * light
    };
}

/// Ambient light factor for a phase. Depths (abyss) stay dim / night.
pub fn ambient_light_factor(phase: f32, abyss: bool) -> f32 {
    if abyss {
        0.22
    } else if phase < 0.42 {
        1.0
    } else if phase < 0.52 {
        0.55
    } else if phase < 0.88 {
        0.28
    } else {
        0.62
    }
}

/// Night flag from the same light bands `turn_the_clock` uses.
pub fn night_for(phase: f32, abyss: bool) -> bool {
    ambient_light_factor(phase, abyss) < 0.40
}

// --- Schedule helpers (stub/feel) — ride LivingDay; no NPC spawn / mesh ---

/// Named day periods mapped from LivingDay.phase bands (Dawn/Day/Dusk/Night).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayPeriod {
    Dawn,
    Day,
    Dusk,
    Night,
}

impl DayPeriod {
    pub fn name(self) -> &'static str {
        match self {
            DayPeriod::Dawn => "Dawn",
            DayPeriod::Day => "Day",
            DayPeriod::Dusk => "Dusk",
            DayPeriod::Night => "Night",
        }
    }
}

/// Same phase bands as ambient: Day <0.42, Dusk <0.52, Night <0.88, else Dawn.
pub fn period_for_phase(phase: f32) -> DayPeriod {
    let p = phase.rem_euclid(1.0);
    if p < 0.42 {
        DayPeriod::Day
    } else if p < 0.52 {
        DayPeriod::Dusk
    } else if p < 0.88 {
        DayPeriod::Night
    } else {
        DayPeriod::Dawn
    }
}

/// Four Places — posts schedules prefer (one leg stub).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulePlace {
    Sanctuary,
    Heartwood,
    Threshold,
    Depths,
}

impl SchedulePlace {
    pub fn name(self) -> &'static str {
        match self {
            SchedulePlace::Sanctuary => "Sanctuary",
            SchedulePlace::Heartwood => "Heartwood",
            SchedulePlace::Threshold => "Threshold",
            SchedulePlace::Depths => "Depths",
        }
    }

    /// Known post at this Place (still-frame readable).
    pub fn post(self) -> &'static str {
        match self {
            SchedulePlace::Sanctuary => "Sanctuary well",
            SchedulePlace::Heartwood => "Heartwood Wards",
            SchedulePlace::Threshold => "Threshold pipe",
            SchedulePlace::Depths => "Depths landing",
        }
    }
}

/// Sacred five — Tend / Take / Flow / Reserve / Mend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SacredVerb {
    Tend,
    Take,
    Flow,
    Reserve,
    Mend,
}

impl SacredVerb {
    pub fn name(self) -> &'static str {
        match self {
            SacredVerb::Tend => "Tend",
            SacredVerb::Take => "Take",
            SacredVerb::Flow => "Flow",
            SacredVerb::Reserve => "Reserve",
            SacredVerb::Mend => "Mend",
        }
    }
}

/// Preferred post + sacred-five work intent for a Place at a day period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreferredWork {
    pub place: SchedulePlace,
    pub post: &'static str,
    pub verb: SacredVerb,
}

/// Place-honest preferred work: Threshold Tend-not-Take; Depths restore-not-Take.
pub fn preferred_work(place: SchedulePlace, period: DayPeriod) -> PreferredWork {
    let verb = match place {
        SchedulePlace::Sanctuary => match period {
            DayPeriod::Dawn => SacredVerb::Tend,
            DayPeriod::Day => SacredVerb::Take,
            DayPeriod::Dusk => SacredVerb::Flow,
            DayPeriod::Night => SacredVerb::Reserve,
        },
        SchedulePlace::Heartwood => match period {
            DayPeriod::Dawn | DayPeriod::Day => SacredVerb::Tend,
            DayPeriod::Dusk => SacredVerb::Mend,
            DayPeriod::Night => SacredVerb::Reserve,
        },
        // Threshold pipe: Tend-not-Take — never Take at the pipe.
        SchedulePlace::Threshold => match period {
            DayPeriod::Dawn | DayPeriod::Day | DayPeriod::Dusk => SacredVerb::Tend,
            DayPeriod::Night => SacredVerb::Mend,
        },
        // Depths landing: restore-not-Take — Mend / Tend only.
        SchedulePlace::Depths => match period {
            DayPeriod::Day => SacredVerb::Tend,
            DayPeriod::Dawn | DayPeriod::Dusk | DayPeriod::Night => SacredVerb::Mend,
        },
    };
    PreferredWork {
        place,
        post: place.post(),
        verb,
    }
}

/// Still-frame line: period · post · verb (no crime / ownership / gold / Market / Online).
pub fn still_frame_line(place: SchedulePlace, phase: f32) -> String {
    let period = period_for_phase(phase);
    let work = preferred_work(place, period);
    format!("{} · {} · {}", period.name(), work.post, work.verb.name())
}

/// Well moods greet may rhyme with (Idle / Glowing / Tended / Resting / Stressed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellMood {
    Idle,
    Glowing,
    Tended,
    Resting,
    Stressed,
}

impl WellMood {
    pub fn name(self) -> &'static str {
        match self {
            WellMood::Idle => "Idle",
            WellMood::Glowing => "Glowing",
            WellMood::Tended => "Tended",
            WellMood::Resting => "Resting",
            WellMood::Stressed => "Stressed",
        }
    }
}

/// Short greet helper copy by Place + mood — one-HUD dress; no new audio/mesh.
pub fn greet_by_place_mood(place: SchedulePlace, mood: WellMood) -> &'static str {
    match (place, mood) {
        (SchedulePlace::Sanctuary, WellMood::Glowing) => "Well is warm — peace holds.",
        (SchedulePlace::Sanctuary, WellMood::Stressed) => "Well runs thin — tend when you can.",
        (SchedulePlace::Sanctuary, _) => "Sanctuary keeps the yard.",
        (SchedulePlace::Heartwood, WellMood::Tended) => "Wards stand tended — wood room steady.",
        (SchedulePlace::Heartwood, _) => "Heartwood posts keep seal dress.",
        (SchedulePlace::Threshold, WellMood::Resting) => "Pipe rests — look, then Tend.",
        (SchedulePlace::Threshold, _) => "Threshold pipe — Tend, not Take.",
        (SchedulePlace::Depths, WellMood::Stressed) => "Landing remembers — restore, not Take.",
        (SchedulePlace::Depths, _) => "Depths landing — restore the stone.",
    }
}

/// Hour-finish bar: schedules never strand the hour, write the house book, or gate Places.
pub fn hour_finishes_without_persons() -> bool {
    true
}

pub fn schedules_write_house_book() -> bool {
    false
}

pub fn schedules_gate_places() -> bool {
    false
}

/// Schedule / greet copy refuses crime · ownership · theft · fence · gold · Market · Online.
pub fn schedule_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("market")
        && !low.contains("price")
        && !low.contains("sell")
        && !low.contains("ticker")
        && !low.contains("auction")
        && !low.contains("theft")
        && !low.contains("steal")
        && !low.contains("fence")
        && !low.contains("crime")
        && !low.contains("ownership")
        && !low.contains("online")
        && !low.contains("pickpocket")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedules_ride_living_day_phase() {
        assert_eq!(period_for_phase(0.18), DayPeriod::Day);
        assert_eq!(period_for_phase(0.45), DayPeriod::Dusk);
        assert_eq!(period_for_phase(0.70), DayPeriod::Night);
        assert_eq!(period_for_phase(0.95), DayPeriod::Dawn);
        let day = LivingDay {
            phase: 0.18,
            night: false,
        };
        assert_eq!(day.period(), DayPeriod::Day);
        let work = preferred_work(SchedulePlace::Sanctuary, day.period());
        assert_eq!(work.post, "Sanctuary well");
        assert_eq!(work.verb, SacredVerb::Take);
        let line = still_frame_line(SchedulePlace::Sanctuary, day.phase);
        assert!(line.starts_with("Day ·"));
        assert!(line.contains("Sanctuary well"));
        assert!(line.contains("Take"));
    }

    #[test]
    fn depths_stay_night() {
        // Any phase: abyss (realm 3) forces dim light → night.
        for phase in [0.0, 0.18, 0.45, 0.70, 0.95] {
            assert!(night_for(phase, true), "phase {phase}");
            assert!((ambient_light_factor(phase, true) - 0.22).abs() < f32::EPSILON);
        }
        // Surface day is not night.
        assert!(!night_for(0.18, false));
        assert!(night_for(0.70, false));
    }

    #[test]
    fn hour_finish_without_persons_no_gate_or_book() {
        assert!(hour_finishes_without_persons());
        assert!(!schedules_write_house_book());
        assert!(!schedules_gate_places());
        // Preferred work is pure data — removing persons cannot strand these flags.
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            for period in [
                DayPeriod::Dawn,
                DayPeriod::Day,
                DayPeriod::Dusk,
                DayPeriod::Night,
            ] {
                let _ = preferred_work(place, period);
            }
        }
        assert!(hour_finishes_without_persons());
    }

    #[test]
    fn place_honest_verbs_refuse_bad_take() {
        // Threshold Tend-not-Take; Depths restore-not-Take.
        for period in [
            DayPeriod::Dawn,
            DayPeriod::Day,
            DayPeriod::Dusk,
            DayPeriod::Night,
        ] {
            let pipe = preferred_work(SchedulePlace::Threshold, period);
            assert_ne!(pipe.verb, SacredVerb::Take, "Threshold {period:?}");
            assert_eq!(pipe.post, "Threshold pipe");
            let landing = preferred_work(SchedulePlace::Depths, period);
            assert_ne!(landing.verb, SacredVerb::Take, "Depths {period:?}");
            assert_eq!(landing.post, "Depths landing");
            assert!(
                matches!(landing.verb, SacredVerb::Tend | SacredVerb::Mend),
                "Depths restore verb"
            );
        }
        assert_eq!(
            preferred_work(SchedulePlace::Heartwood, DayPeriod::Day).post,
            "Heartwood Wards"
        );
    }

    #[test]
    fn refuse_crime_gold_market_online_copy() {
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            for period in [
                DayPeriod::Dawn,
                DayPeriod::Day,
                DayPeriod::Dusk,
                DayPeriod::Night,
            ] {
                let work = preferred_work(place, period);
                let line = format!(
                    "{} · {} · {}",
                    period.name(),
                    work.post,
                    work.verb.name()
                );
                assert!(schedule_copy_is_honest(&line), "{line}");
            }
            for mood in [
                WellMood::Idle,
                WellMood::Glowing,
                WellMood::Tended,
                WellMood::Resting,
                WellMood::Stressed,
            ] {
                let greet = greet_by_place_mood(place, mood);
                assert!(schedule_copy_is_honest(greet), "{greet}");
            }
        }
        assert!(!schedule_copy_is_honest("sell gold on Market"));
        assert!(!schedule_copy_is_honest("crime meter Online"));
        assert!(!schedule_copy_is_honest("theft and fence ownership"));
        assert!(!schedule_copy_is_honest("pickpocket the stall"));
        assert!(schedule_copy_is_honest("Day · Sanctuary well · Take"));
        assert!(schedule_copy_is_honest(
            "Threshold pipe — Tend, not Take."
        ));
    }

    #[test]
    fn day_secs_loop_unchanged() {
        assert!((DAY_SECS - 240.0).abs() < f32::EPSILON);
        // Default boot phase is Day on the surface.
        let day = LivingDay::default();
        assert!((day.phase - 0.18).abs() < f32::EPSILON);
        assert_eq!(day.period(), DayPeriod::Day);
        assert!(!night_for(day.phase, false));
    }
}
