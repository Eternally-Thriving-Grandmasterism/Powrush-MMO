//! Lived-hour infra witness — Slice 5 (v23.2.9) + pack (v23.2.29)
//!
//! Offline extractor + spill as readable evidence. Dies in Peace.
//! Does not teach attack.
//!
//! CARD FLESH-INFRA-SPILL — the existing spill slab may name the Place
//! (`HexTravelState::chip_name`) while charter skin is live. Absent travel
//! keeps the slab byte for byte. No second HUD. No gold.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//!
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::infra_spill::InfraWitness;

use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

/// CARD FLESH-INFRA-SPILL — `{place} · {slab}` when a chip is present.
/// `None` returns the bare slab. One string. No second widget.
fn spill_slab_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

#[derive(Resource, Debug, Clone)]
pub struct EvidenceYard {
    pub witness: InfraWitness,
}

impl Default for EvidenceYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                witness: HourTwoPack::from_json(&raw).witness,
            };
        }
        Self {
            witness: InfraWitness::default(),
        }
    }
}

#[derive(Component)]
struct SpillSlabRoot;
#[derive(Component)]
struct SpillSlabText;

pub struct InfraSpillPlugin;

impl Plugin for InfraSpillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EvidenceYard>()
            .add_systems(Startup, spawn_spill_slab)
            .add_systems(Update, (witness_offline, update_spill_slab));
    }
}

fn spawn_spill_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(52.0),
                    left: Val::Px(16.0),
                    width: Val::Px(520.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.10, 0.07, 0.04, 0.92).into(),
                border_color: Color::srgba(0.78, 0.62, 0.32, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            SpillSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.98, 0.90, 0.72),
                        ..default()
                    },
                ),
                SpillSlabText,
            ));
        });
}

/// After Q founds the House, I2 / Offline extractor spill is the witness.
/// Peace and visitor-without-charter stay dark.
pub fn try_witness_house_spill(hour: &HourSacred, yard: &mut EvidenceYard) -> bool {
    if !hour.charter_skin_live() {
        return false;
    }
    yard.witness.ensure_offline_extractor();
    if yard.witness.seen {
        return false;
    }
    yard.witness.seen = true;
    true
}

fn witness_offline(
    hour: Res<HourSacred>,
    mut yard: ResMut<EvidenceYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if try_witness_house_spill(&hour, &mut yard) {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstSpillWitness,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_spill_slab(
    hour: Res<HourSacred>,
    yard: Res<EvidenceYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<SpillSlabRoot>>,
    mut text_q: Query<&mut Text, With<SpillSlabText>>,
) {
    let show = yard.witness.visible_on(hour.hex()) && hour.charter_skin_live();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !show {
        return;
    }
    let bare = yard.witness.slab_line();
    let line = spill_slab_line(&bare, travel.as_ref().map(|state| state.chip_name()));
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    #[test]
    fn peace_hides_spill() {
        let hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert_eq!(hour.hex(), HexFlag::Peace);
        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        yard.witness.ensure_offline_extractor();
        assert!(!yard.witness.visible_on(hour.hex()));
        assert!(!try_witness_house_spill(&hour, &mut yard));
    }

    /// Playtest H2-Q: Q off Peace plants House then I2 / Offline extractor spill.
    #[test]
    fn q_off_peace_plants_house_and_i2_spill() {
        let mut hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(crate::hour_sacred::try_ridge_tab(&mut hour, true));
        let mut factory = shared::vertical_factory::VerticalFactory::default();
        assert!(crate::hour_sacred::try_plant_house(&mut hour, &mut factory));
        assert!(hour.charter_skin_live());

        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        assert!(try_witness_house_spill(&hour, &mut yard));
        assert!(yard.witness.seen);
        let pack = yard.witness.pack.as_ref().expect("I2 pack");
        assert_eq!(pack.code, shared::infra_spill::OffenseCode::I2);
        assert!(pack.spill);
        assert!(yard.witness.visible_on(hour.hex()));
        let line = yard.witness.slab_line();
        assert!(line.contains("Extractor"));
        assert!(line.contains("I2") || line.contains("spill"));
        assert!(!try_witness_house_spill(&hour, &mut yard), "spill is once");
    }

    /// CARD FLESH-INFRA-SPILL — Place prefixes the spill slab; no travel keeps it exact.
    #[test]
    fn flesh_infra_spill_names_chip_or_keeps_slab() {
        use shared::hex_travel::PlaceId;

        let mut yard = EvidenceYard {
            witness: InfraWitness::default(),
        };
        yard.witness.ensure_offline_extractor();
        let bare = yard.witness.slab_line();
        assert!(bare.starts_with("Extractor Offline · spill on the ground · "));
        assert_eq!(spill_slab_line(&bare, None), bare);

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(travel.chip_name(), id.chip_name());
            assert_eq!(
                spill_slab_line(&bare, Some(travel.chip_name())),
                format!("{name} · {bare}")
            );
        }

        let mut peace = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(!peace.charter_skin_live());
        assert!(crate::hour_sacred::try_ridge_tab(&mut peace, true));
        assert!(crate::hour_sacred::try_plant_house(
            &mut peace,
            &mut shared::vertical_factory::VerticalFactory::default(),
        ));
        assert!(peace.charter_skin_live());

        for sample in [
            spill_slab_line(&bare, Some(PlaceId::Sanctuary.chip_name())),
            spill_slab_line(&bare, Some(PlaceId::Heartwood.chip_name())),
            spill_slab_line(&bare, Some(PlaceId::Depths.chip_name())),
            spill_slab_line(&bare, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
