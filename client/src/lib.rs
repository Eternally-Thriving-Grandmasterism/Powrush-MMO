//! Powrush-MMO lived first hour.
//!
//! One human, one machine. Network, simulation crate, GPU-test spheres, and
//! Steam-as-blocker stay off this graph. Contact: info@Rathor.ai

use bevy::prelude::*;

pub mod lived_hour_support;
pub mod lived_hour_bind;
pub mod hour_sacred;
pub mod hour_two_resume;
pub mod ui_above_world;
pub mod title_screen;
pub mod hex_travel;
pub mod depths_landing;
pub mod heartwood_lip;
pub mod heartwood_wards;
pub mod local_settings;
pub mod vertical_factory;
pub mod coop_voice;
pub mod infra_spill;
pub mod ledger_bind;
pub mod fabricator;
pub mod embassy;
pub mod war_week;
pub mod crownstone;
pub mod species_redemption;
pub mod hybrid_matrix;
pub mod compass;
pub mod skirmish_well;
pub mod input;
pub mod touch_controls;
pub mod soft_play_bindings;
pub mod first_session_guidance;
pub mod first_hour_camera;
pub mod harvest_feel;
pub mod peace_audio;
pub mod feel_move;
pub mod first_harvest_epiphany;
pub mod mercy_harvest_nodes;
pub mod climate_visible;
pub mod shard_climate;
pub mod net_mode;
pub mod climate_script;
pub mod human_inventory;
pub mod local_session_persist;
pub mod lived_sim_bridge;
pub mod local_sovereign_session;
pub mod living_practice_loop;
pub mod rbe_allocate_choice;
pub mod thriving_moments;
pub mod human_soft_panels;
pub mod human_presence;
pub mod first_whisper;
pub mod local_human_sim;
pub mod player_lineage;
pub mod living_ecology;
pub mod living_body;
pub mod living_freshness;
pub mod living_day;
pub mod companion_bond;
pub mod flow_weather;
pub mod abundance_journey_echo;
pub mod world_answer;
pub mod lattice_flow_share;
pub mod climate_plane;
pub mod light_gen;
pub mod hands_memory;
pub mod foundation_lattice;
pub mod resonance_flavors;
pub mod mercy_transporters;
pub mod steam_abundance_mirror;

pub use first_session_guidance::{FirstSessionGuidancePlugin, FirstSessionGuidance};
pub use input::InputPlugin;
pub use lived_hour_support::LivedHourEconomyPlugin;

/// Default lived-hour plugin graph. This is the player door.
pub struct PowrushClientBundle;

impl Default for PowrushClientBundle {
    fn default() -> Self {
        Self
    }
}

impl PowrushClientBundle {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for PowrushClientBundle {
    fn build(&self, app: &mut App) {
        app.add_plugins(hour_sacred::HourSacredPlugin);
        app.add_plugins(ui_above_world::UiAboveWorldPlugin);
        app.add_plugins(title_screen::TitleScreenPlugin);
        app.add_plugins(vertical_factory::VerticalFactoryPlugin);
        app.add_plugins(coop_voice::CoopVoicePlugin);
        app.add_plugins(infra_spill::InfraSpillPlugin);
        app.add_plugins(ledger_bind::LedgerBindPlugin);
        app.add_plugins(fabricator::FabricatorPlugin);
        app.add_plugins(embassy::EmbassyPlugin);
        app.add_plugins(war_week::WarWeekPlugin);
        app.add_plugins(crownstone::CrownstonePlugin);
        app.add_plugins(species_redemption::SpeciesRedemptionPlugin);
        app.add_plugins(hybrid_matrix::HybridMatrixPlugin);
        app.add_plugins(compass::CompassPlugin);
        app.add_plugins(skirmish_well::SkirmishWellPlugin);
        app.add_plugins(LivedHourEconomyPlugin);
        app.add_plugins(hex_travel::HexTravelPlugin);
        app.add_plugins(depths_landing::DepthsLandingPlugin);
        app.add_plugins(local_settings::LocalSettingsPlugin);
        app.add_plugins(InputPlugin);
        app.add_plugins(touch_controls::TouchControlsPlugin);
        app.add_plugins(first_session_guidance::FirstSessionGuidancePlugin);
        app.add_plugins(thriving_moments::ThrivingMomentsPlugin);
        app.add_plugins(living_practice_loop::LivingPracticeLoopPlugin);
        app.add_plugins(rbe_allocate_choice::RbeAllocateChoicePlugin);
        app.add_plugins(abundance_journey_echo::AbundanceJourneyEchoPlugin);
        app.add_plugins(mercy_harvest_nodes::MercyHarvestNodesPlugin);
        app.add_plugins(climate_visible::ClimateVisiblePlugin);
        app.add_plugins(shard_climate::ShardClimatePlugin);
        app.add_plugins(net_mode::NetModePlugin);
        app.add_plugins(climate_script::ClimateScriptPlugin);
        app.add_plugins(climate_plane::ClimatePlanePlugin);
        app.add_plugins(light_gen::LightGenPlugin);
        app.add_plugins(living_ecology::LivingEcologyPlugin);
        app.add_plugins(living_freshness::LivingFreshnessPlugin);
        app.add_plugins(living_body::LivingBodyPlugin);
        app.add_plugins(living_day::LivingDayPlugin);
        app.add_plugins(hands_memory::HandsMemoryPlugin);
        app.add_plugins(companion_bond::CompanionBondPlugin);
        app.add_plugins(harvest_feel::HarvestFeelPlugin);
        app.add_plugins(world_answer::WorldAnswerPlugin);
        app.add_plugins(first_harvest_epiphany::FirstHarvestEpiphanyPlugin);
        // After FirstHarvest so the same-frame well Use credits the sting.
        app.add_plugins(peace_audio::PeaceAudioPlugin);
        app.add_plugins(feel_move::FeelMovePlugin);
        app.add_plugins(human_inventory::HumanInventoryPlugin);
        app.add_plugins(local_session_persist::LocalSessionPersistPlugin);
        app.add_plugins(first_whisper::FirstWhisperPlugin);
        app.add_plugins(first_hour_camera::FirstHourCameraPlugin);
        app.add_plugins(human_soft_panels::HumanSoftPanelsPlugin);
        app.add_plugins(lattice_flow_share::LatticeFlowSharePlugin);
        app.add_plugins(steam_abundance_mirror::SteamAbundanceMirrorPlugin);
        app.add_plugins(foundation_lattice::FoundationLatticePlugin);
        app.add_plugins(resonance_flavors::ResonanceFlavorsPlugin);
        app.add_plugins(mercy_transporters::MercyTransportersPlugin);
        app.add_plugins(human_presence::HumanPresencePlugin);
        app.add_plugins(heartwood_lip::HeartwoodLipPlugin);
        app.add_plugins(heartwood_wards::HeartwoodWardsPlugin);
        app.add_plugins(flow_weather::FlowWeatherPlugin);
        app.add_plugins(local_human_sim::LocalHumanSimPlugin);
        app.add_plugins(player_lineage::PlayerLineagePlugin);
        app.add_plugins(lived_sim_bridge::LivedSimBridgePlugin);
        app.add_plugins(local_sovereign_session::LocalSovereignSessionPlugin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_is_a_plugin() {
        let _ = PowrushClientBundle::new();
    }

    #[test]
    fn first_session_starts_on_move() {
        let g = first_session_guidance::FirstSessionGuidance::default();
        assert!(g.active);
        assert!(!g.dismissed);
    }

    // --- CARD Q0 MACHINE-BEATS ----------------------------------------------
    // Hands cook only. Compose existing pub helpers. 0 meshes · 0 new PlaceId
    // · 0 sockets · 0 new crates. Title Online stays grey.

    fn q0_online_stays_grey() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!shared::hex_protocol::default_client_listens());
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!online_row_is_honest_disabled(ONLINE_STUB_LABEL, true));
        let net = net_mode::SessionNetMode::from_powrush_net(shared::hex_listen::PowrushNet::Off);
        assert!(!net.title_online_enabled());
        assert_eq!(net.mode, shared::net_mode::NetMode::Offline);
    }

    fn q0_place_id_stays_three() {
        use hour_sacred::four_place_landings_only;
        use shared::hex_travel::{PlaceId, LOCAL_HEXES};

        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert!(four_place_landings_only());
        for place in LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
            assert_ne!(place.as_str(), "market");
            assert_ne!(place.as_str(), "auction");
            assert_ne!(place.as_str(), "ah");
        }
    }

    /// CARD Q0 — Title Play / Continue / Settings · Online grey.
    #[test]
    fn q0_title_play_continue_settings_online_grey() {
        use title_screen::{
            f6_title_chrome_holds, l2_title_chrome_holds, s3_title_chrome_holds,
            title_has_portraits_grid, title_has_race_class_lobby, title_has_race_portraits,
            LaunchDoor, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };

        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
        assert!(l2_title_chrome_holds());
        assert!(s3_title_chrome_holds());
        assert!(f6_title_chrome_holds());
        assert!(!title_has_race_portraits());
        assert!(!title_has_race_class_lobby());
        assert!(!title_has_portraits_grid());
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — new soul = light · sealed Continue = dress.
    #[test]
    fn q0_new_soul_light_sealed_continue_dress() {
        use hour_sacred::{
            continue_body_line, continue_is_the_body, continue_sealed_souls_from_hour_two_json,
            doors_are_unsealed, merge_gate_seal_into_hour_two_json, play_new_light_soul,
            soul_is_light, GardenRosterSoul, HousePeople, PeopleLanding, HOUSE_PEOPLES,
        };
        use shared::hex_travel::PlaceId;
        use title_screen::s3_continue_roster_line;

        let play = play_new_light_soul();
        assert_eq!(play, GardenRosterSoul::LightUnsealed);
        assert!(play.is_light());
        assert!(play.is_unsealed());
        assert!(soul_is_light(play.sealed_pair()));
        assert!(doors_are_unsealed(play.sealed_pair()));
        assert_eq!(play.dress_line(), "light");
        assert!(play.last_place().is_none(), "garden is not a Place landing");
        assert_eq!(play.last_place_name(), "Garden");
        assert!(!continue_is_the_body(play));
        assert!(continue_body_line(play).is_none());
        assert!(continue_sealed_souls_from_hour_two_json("{}").is_empty());

        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_sealed_souls_from_hour_two_json(&json);
        assert_eq!(list.len(), 1, "S2 keys are one sealed soul, not a portraits grid");
        assert_ne!(list.len(), HOUSE_PEOPLES.len());
        let soul = list[0];
        assert!(continue_is_the_body(soul));
        assert!(!soul.is_light());
        assert_eq!(soul.dress_line(), "Cydruid · human-in-frame");
        assert!(!soul.dress_line().contains("treant"));
        assert!(!soul.dress_line().contains("portrait"));
        assert_eq!(soul.last_place(), Some(PlaceId::Heartwood));
        assert_eq!(soul.last_place_name(), "Heartwood");
        assert_eq!(
            continue_body_line(soul).as_deref(),
            Some("Cydruid · human-in-frame · Heartwood")
        );
        assert_eq!(
            s3_continue_roster_line(soul),
            "Cydruid · human-in-frame · Heartwood"
        );
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — wrong door / decline = still light (S2).
    #[test]
    fn q0_wrong_door_decline_still_light() {
        use hour_sacred::{
            decline_or_wrong_door, doors_are_unsealed, play_new_light_soul, skip_house_stays_light,
            soul_is_light, still_unsealed_until_eq, HousePeople, PeopleLanding,
        };

        let mut crossed = Some(HousePeople::Draek);
        let mut pending = Some(PeopleLanding::DepthsTealWayHome);
        assert!(decline_or_wrong_door(&mut crossed, &mut pending, None));
        assert!(crossed.is_none());
        assert!(pending.is_none());
        assert!(soul_is_light(None));
        assert!(doors_are_unsealed(None));
        assert!(still_unsealed_until_eq(None));
        assert!(skip_house_stays_light(false));
        let play = play_new_light_soul();
        assert!(play.is_light());
        assert_eq!(play.dress_line(), "light");

        let sealed = Some((HousePeople::Human, PeopleLanding::SanctuaryYard));
        let mut crossed = Some(HousePeople::Human);
        let mut pending = Some(PeopleLanding::SanctuaryYard);
        assert!(!decline_or_wrong_door(&mut crossed, &mut pending, sealed));
        assert_eq!(crossed, Some(HousePeople::Human));
        assert!(!soul_is_light(sealed));
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — each People landing + S1/F7 aftermath line.
    #[test]
    fn q0_each_people_landing_s1_f7_aftermath_line() {
        use first_session_guidance::{
            aftermath_after_people_landing, f7_aftermath_variant_for_landing,
            f7_first_minutes_aftermath_line, first_minutes_aftermath_line, AMBROSIAN_AFTERMATH,
            AMBROSIAN_AFTERMATH_VARIANT, CYDRUID_AFTERMATH, CYDRUID_AFTERMATH_VARIANT,
            DRAEK_AFTERMATH, DRAEK_AFTERMATH_VARIANT, GARDEN_WANT, HUMAN_AFTERMATH,
            HUMAN_AFTERMATH_VARIANT, QUELLORIAN_AFTERMATH, QUELLORIAN_AFTERMATH_VARIANT,
        };
        use hour_sacred::{HousePeople, PeopleLanding, HOUSE_PEOPLES};

        assert_eq!(HOUSE_PEOPLES.len(), 5);
        assert_eq!(aftermath_after_people_landing(None), GARDEN_WANT);
        assert_eq!(
            f7_first_minutes_aftermath_line(None),
            first_minutes_aftermath_line(None)
        );

        let pairs = [
            (
                HousePeople::Human,
                PeopleLanding::SanctuaryYard,
                HUMAN_AFTERMATH,
                HUMAN_AFTERMATH_VARIANT,
            ),
            (
                HousePeople::Ambrosian,
                PeopleLanding::SanctuaryWellFromAbove,
                AMBROSIAN_AFTERMATH,
                AMBROSIAN_AFTERMATH_VARIANT,
            ),
            (
                HousePeople::Cydruid,
                PeopleLanding::Heartwood,
                CYDRUID_AFTERMATH,
                CYDRUID_AFTERMATH_VARIANT,
            ),
            (
                HousePeople::Quellorian,
                PeopleLanding::Threshold,
                QUELLORIAN_AFTERMATH,
                QUELLORIAN_AFTERMATH_VARIANT,
            ),
            (
                HousePeople::Draek,
                PeopleLanding::DepthsTealWayHome,
                DRAEK_AFTERMATH,
                DRAEK_AFTERMATH_VARIANT,
            ),
        ];
        assert_eq!(pairs.len(), HOUSE_PEOPLES.len());
        for (i, (people, landing, s1, variant)) in pairs.iter().enumerate() {
            assert_eq!(people.landing(), *landing);
            let land = Some(*landing);
            assert_eq!(aftermath_after_people_landing(land), *s1);
            assert_eq!(f7_aftermath_variant_for_landing(*landing), *variant);
            assert_ne!(*variant, *s1, "F7 extra line is not a rewrite of S1");
            let s1_plate = first_minutes_aftermath_line(land);
            let f7_plate = f7_first_minutes_aftermath_line(land);
            assert!(s1_plate.contains(*s1));
            assert!(!s1_plate.contains(variant), "S1 plate WRITE stays unread");
            assert_eq!(f7_plate, format!("{s1_plate} · {variant}"));
            assert_eq!(f7_plate.matches(variant).count(), 1);
            for (j, (_, _, other_s1, other_var)) in pairs.iter().enumerate() {
                if i == j {
                    continue;
                }
                assert_ne!(s1, other_s1, "five S1 reads must differ");
                assert_ne!(variant, other_var, "five F7 extras must differ");
            }
        }
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — F1 stance four values · garden = no stance.
    #[test]
    fn q0_f1_stance_four_values_garden_no_stance() {
        use hour_sacred::{
            garden_light_stance_from_hour_two_json, gate_seal_from_hour_two_json,
            merge_gate_seal_into_hour_two_json, play_new_light_soul,
            sealed_soul_stance_from_hour_two_json, set_sealed_soul_stance, soul_is_light,
            soul_may_trade_from_hour_two_json, HousePeople, PeopleLanding,
        };
        use shared::persona::SoulStance;

        assert_eq!(SoulStance::ALL.len(), 4);
        assert_eq!(SoulStance::OpenTrade.as_str(), "Open-trade");
        assert_eq!(SoulStance::Neutral.as_str(), "Neutral");
        assert_eq!(SoulStance::Closed.as_str(), "Closed");
        assert_eq!(SoulStance::Hostile.as_str(), "Hostile");

        let light = "{}";
        assert!(soul_is_light(gate_seal_from_hour_two_json(light)));
        assert!(garden_light_stance_from_hour_two_json(light).is_none());
        assert!(!soul_may_trade_from_hour_two_json(light));
        assert!(set_sealed_soul_stance(light, SoulStance::OpenTrade).is_none());
        let play = play_new_light_soul();
        assert!(play.is_light());
        assert_eq!(play.last_place_name(), "Garden");

        let sealed = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        for stance in SoulStance::ALL {
            let json = set_sealed_soul_stance(&sealed, stance).expect("sealed may set");
            assert_eq!(sealed_soul_stance_from_hour_two_json(&json), Some(stance));
        }
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — F2 window only if Open-trade · ghost lots offline.
    #[test]
    fn q0_f2_window_only_if_open_trade_ghost_lots_offline() {
        use hour_sacred::{
            ah_window_may_open_from_hour_two_json, garden_light_can_open_ah_window,
            merge_gate_seal_into_hour_two_json, set_sealed_soul_stance,
            sync_ah_window_to_hour_two_json, try_open_ah_window_from_hour_two_json, F2_AH_IS_PLACE,
            HousePeople, PeopleLanding,
        };
        use shared::ledger_bind::{GhostLot, LedgerBoard};
        use shared::persona::SoulStance;

        let light = "{}";
        let mut open = false;
        assert!(!try_open_ah_window_from_hour_two_json(light, &mut open));
        assert!(!open);
        assert!(!garden_light_can_open_ah_window(light));

        let sealed = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        for stance in [SoulStance::Neutral, SoulStance::Closed, SoulStance::Hostile] {
            let json = set_sealed_soul_stance(&sealed, stance).unwrap();
            let mut open = false;
            assert!(!ah_window_may_open_from_hour_two_json(&json));
            assert!(!try_open_ah_window_from_hour_two_json(&json, &mut open));
            assert!(!open);
        }

        let open_trade = set_sealed_soul_stance(&sealed, SoulStance::OpenTrade).unwrap();
        let mut open = false;
        assert!(ah_window_may_open_from_hour_two_json(&open_trade));
        assert!(try_open_ah_window_from_hour_two_json(&open_trade, &mut open));
        assert!(open);
        let closed = set_sealed_soul_stance(&open_trade, SoulStance::Closed).unwrap();
        sync_ah_window_to_hour_two_json(&closed, &mut open);
        assert!(!open);
        assert!(!F2_AH_IS_PLACE);

        let mut board = LedgerBoard::default();
        board.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(board.try_open_ah_panel(true));
        assert!(!board.ghost_lots.is_empty());
        assert!(!GhostLot::is_place());
        let rows = board.ah_panel_rows();
        assert!(rows
            .iter()
            .any(|r| r.contains("Ghost lot") && r.contains("ledger row")));
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — F3 Hostile Take allowed · F9 NEVC label · no lockout.
    #[test]
    fn q0_f3_hostile_take_allowed_f9_nevc_label_no_lockout() {
        use hour_sacred::{
            ah_window_may_open_from_hour_two_json, hostile_take_ledger_rows_from_board,
            hostile_take_nevc_label_from_board, merge_gate_seal_into_hour_two_json,
            sealed_hostile_may_take_from_hour_two_json, set_sealed_soul_stance,
            soul_stays_playable_under_hostile_from_hour_two_json,
            try_hostile_take_ledger_from_hour_two_json, HousePeople, PeopleLanding,
        };
        use shared::ledger_bind::{HostilePracticeOutcome, LedgerBoard};
        use shared::persona::SoulStance;

        let light = "{}";
        assert!(!sealed_hostile_may_take_from_hour_two_json(light));

        let sealed = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        for stance in [SoulStance::OpenTrade, SoulStance::Neutral, SoulStance::Closed] {
            let json = set_sealed_soul_stance(&sealed, stance).unwrap();
            assert!(!sealed_hostile_may_take_from_hour_two_json(&json));
        }

        let hostile = set_sealed_soul_stance(&sealed, SoulStance::Hostile).unwrap();
        assert!(sealed_hostile_may_take_from_hour_two_json(&hostile));
        assert!(soul_stays_playable_under_hostile_from_hour_two_json(&hostile));
        assert!(!shared::persona::HOSTILE_LOCKOUT);
        assert!(!shared::persona::soul_is_locked_out(
            true,
            Some(SoulStance::Hostile)
        ));
        assert!(!ah_window_may_open_from_hour_two_json(&hostile));

        let mut board = LedgerBoard::default();
        board.set_sealed_soul_stance(true, SoulStance::Hostile);
        board.ensure_i2("q0-hostile-take");
        assert_eq!(
            try_hostile_take_ledger_from_hour_two_json(&hostile, &mut board),
            HostilePracticeOutcome::Taken
        );
        let rows = hostile_take_ledger_rows_from_board(&board);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains("Hostile Take"));
        assert!(rows[0].contains("not wages"));
        let label = hostile_take_nevc_label_from_board(&board).expect("F9 NEVC label");
        assert!(label.contains("NEVC:"));
        assert!(label.contains("display only"));
        assert!(label.contains("not wages"));
        assert!(!shared::nevc_visibility::nevc_line_invents_wages(label));
        assert!(!shared::persona::HOSTILE_PRACTICE_INVENTS_WAGES);
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    /// CARD Q0 — F4 dress stays · serve other well offline.
    #[test]
    fn q0_f4_dress_stays_serve_other_well_offline() {
        use hour_sacred::{
            continue_sealed_souls_from_hour_two_json, merge_gate_seal_into_hour_two_json,
            online_human_double_serve_from_hour_two_json, sealed_dress_from_hour_two_json,
            sealed_dress_unchanged_after_offline_double_serve,
            soul_stays_playable_under_double_serve_from_hour_two_json,
            try_offline_double_serve_from_hour_two_json, HousePeople, PeopleLanding,
        };
        use shared::ledger_bind::LedgerBoard;

        let sealed = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        assert_eq!(
            sealed_dress_from_hour_two_json(&sealed),
            Some(HousePeople::Cydruid)
        );
        let mut board = LedgerBoard::default();
        assert!(sealed_dress_unchanged_after_offline_double_serve(
            &sealed,
            HousePeople::Draek,
            &mut board
        ));
        assert!(try_offline_double_serve_from_hour_two_json(
            &sealed,
            HousePeople::Draek,
            &mut board
        ));
        assert_eq!(
            sealed_dress_from_hour_two_json(&sealed),
            Some(HousePeople::Cydruid)
        );
        assert_eq!(board.dress_after_offline_double_serve(), Some("Cydruid"));
        assert!(
            !try_offline_double_serve_from_hour_two_json(
                &sealed,
                HousePeople::Cydruid,
                &mut board
            ),
            "home well is not a double-serve"
        );
        assert!(!try_offline_double_serve_from_hour_two_json(
            "{}",
            HousePeople::Draek,
            &mut board
        ));
        let soul = continue_sealed_souls_from_hour_two_json(&sealed)
            .into_iter()
            .next()
            .expect("sealed body");
        assert_eq!(soul.dress_line(), "Cydruid · human-in-frame");
        assert!(!shared::persona::F4_DRESS_SWAP);
        assert!(!online_human_double_serve_from_hour_two_json(&sealed));
        assert!(shared::persona::online_human_double_serve_gated());
        assert!(soul_stays_playable_under_double_serve_from_hour_two_json(
            &sealed
        ));
        assert!(!shared::persona::F4_LOCKOUT);
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }

    // --- CARD Q1b STRANGER-HOUR-LIB ----------------------------------------
    // Hands cook only. Compose existing pub helpers. 0 meshes · 0 new PlaceId
    // · 0 new files · 0 AGENT_QA_TAPE. Title Online stays grey.
    // Cite: docs/AGENT_AUTONOMY_COURT_2026-09-22.md §2/§3 · docs/CURSOR_GROK_MODEL_SEAT.md

    fn q1_demo_bind() -> lived_hour_bind::LivedHourBind {
        use shared::climate_node::LivedHour;
        use shared::shard_climate::ShardClimate;
        use shared::shard_standing::ShardStanding;
        use shared::week_audit::WeekAudit;

        lived_hour_bind::LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    /// CARD Q1 — tap-E take ≠ hold-E tend (distinct handlers / well speech).
    #[test]
    fn q1_tap_e_take_not_equal_hold_e_tend() {
        use first_harvest_epiphany::{
            hold_e_tend_blocked, is_hold_e_tend, tap_vs_hold_prompt, tend_breathe_answer,
            tend_harmony_pulse_line, TEND_HOLD,
        };
        use mercy_harvest_nodes::{apply_node_harvest, apply_node_tend, looks_like_tend_pulse};

        assert!(!is_hold_e_tend(0.20));
        assert!(!is_hold_e_tend(0.39));
        assert!(is_hold_e_tend(TEND_HOLD));
        assert_eq!(tap_vs_hold_prompt(), "tap E take  ·  hold E tend");
        assert_ne!(tap_vs_hold_prompt(), "E tend the glow");

        let take_at = 10.0;
        let hold_at = take_at + TEND_HOLD;
        assert!(
            !hold_e_tend_blocked(1, 0, take_at, hold_at),
            "take cooldown must not eat hold-E tend"
        );

        let mut node = mercy_harvest_nodes::MercyHarvestNode {
            name: "Sanctuary ember",
            climate_id: 1,
            vitality: 1.0,
            harvests: 0,
            pulse: 0.0,
        };
        let before_take = (node.pulse, node.vitality);
        apply_node_harvest(&mut node);
        let after_take = (node.pulse, node.vitality);
        assert!(!looks_like_tend_pulse(
            before_take.0,
            after_take.0,
            before_take.1,
            after_take.1
        ));

        let mut tend_node = mercy_harvest_nodes::MercyHarvestNode {
            name: "Sanctuary ember",
            climate_id: 1,
            vitality: 0.50,
            harvests: 0,
            pulse: 0.0,
        };
        let before_tend = (tend_node.pulse, tend_node.vitality);
        apply_node_tend(&mut tend_node);
        assert!(looks_like_tend_pulse(
            before_tend.0,
            tend_node.pulse,
            before_tend.1,
            tend_node.vitality
        ));

        let take_speech = "still glows · hold E to tend";
        let tend_speech = tend_harmony_pulse_line("Sanctuary ember", 0.4);
        assert!(tend_breathe_answer().contains("breathes"));
        assert!(tend_speech.contains("Tended") || tend_speech.contains("harmony"));
        assert_ne!(take_speech, tend_speech.as_str());
        q0_online_stays_grey();
    }

    /// CARD Q1 — I satchel non-empty after a successful take.
    #[test]
    fn q1_satchel_nonempty_after_take() {
        use shared::climate_node::TendResult;

        let mut bind = q1_demo_bind();
        assert_eq!(bind.satchel_count(), 0);
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        assert!(bind.satchel_count() > 0);
        assert_eq!(bind.satchel_count(), 1);
        q0_online_stays_grey();
    }

    /// CARD Q1 — R then 1 writes flow restore on a tired well.
    #[test]
    fn q1_r1_flow_restores_tired_well() {
        use rbe_allocate_choice::{try_commit_allocate, AllocatePath, RbeAllocateChoice};
        use shared::climate_node::{NodeState, TendResult};

        let mut a = RbeAllocateChoice::default();
        let mut bind = q1_demo_bind();
        a.note_surplus(1.0);
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        // Second extract leaves the node Resting/Stressed (tired).
        let second = bind.tend(1);
        assert!(matches!(second, TendResult::NoTake { .. }));
        assert!(matches!(
            bind.hour.nodes[0].state,
            NodeState::Resting | NodeState::Stressed
        ));
        let stress_before = bind.climate.stress;
        assert!(try_commit_allocate(
            &mut a,
            &mut bind,
            AllocatePath::FlowOutward
        ));
        assert_eq!(bind.hour.allocation.flow, 1);
        assert_eq!(bind.satchel_count(), 0);
        assert_eq!(bind.last_line, "flow restored the well");
        assert!(
            bind.climate.stress < stress_before,
            "R1 flow must lower climate stress on a tired well"
        );
        assert!(matches!(
            bind.hour.nodes[0].state,
            NodeState::Glowing | NodeState::Resting
        ));
        assert_eq!(
            AllocatePath::FlowOutward.title(),
            "Flow · field restore"
        );
        q0_online_stays_grey();
    }

    /// CARD Q1 — R then 2 banks non-zero reserve (allocation + climate pool).
    #[test]
    fn q1_r2_reserve_nonzero_confirm() {
        use rbe_allocate_choice::{try_commit_allocate, AllocatePath, RbeAllocateChoice};
        use shared::climate_node::TendResult;

        let mut a = RbeAllocateChoice::default();
        let mut bind = q1_demo_bind();
        a.note_surplus(1.0);
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        assert!(try_commit_allocate(
            &mut a,
            &mut bind,
            AllocatePath::StewardReserve
        ));
        assert_ne!(bind.hour.allocation.reserve, 0);
        assert_ne!(bind.climate.reserve_pool, 0);
        assert_eq!(bind.hour.allocation.reserve, 1);
        assert_eq!(bind.climate.reserve_pool, 1);
        assert!(a.reserve_total >= 1.0);
        let line = bind.hour.allocation.reserve_bank_line().expect("banked");
        assert!(line.contains("Reserve 1"));
        assert!(!line.contains("0.0"));
        assert!(bind.last_line.contains("repair-rights"));
        q0_online_stays_grey();
    }

    /// CARD Q1 — H hides guidance card; well words still exist.
    #[test]
    fn q1_h_hides_card_well_words_remain() {
        use first_harvest_epiphany::tap_vs_hold_prompt;
        use first_session_guidance::{
            first_minutes_people_want_line, FirstSessionGuidance, GARDEN_WANT, SANCTUARY_WANT,
        };

        let mut g = FirstSessionGuidance::default();
        assert!(g.active);
        assert!(!g.dismissed);
        g.dismiss();
        assert!(g.dismissed, "H dismisses the guidance card");

        let mut bind = q1_demo_bind();
        assert!(!bind.guidance_hidden);
        bind.toggle_guidance();
        assert!(bind.guidance_hidden);

        // World still owns well words — card hush does not erase them.
        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        assert!(GARDEN_WANT.contains("well") || GARDEN_WANT.contains("tending"));
        let spoken = first_minutes_people_want_line();
        assert!(spoken.contains("well") || spoken.contains("tending"));
        assert_eq!(tap_vs_hold_prompt(), "tap E take  ·  hold E tend");
        q0_online_stays_grey();
    }

    /// CARD Q1 — quit/rerun lived tick round-trip (satchel + allocation).
    #[test]
    fn q1_lived_tick_roundtrip() {
        use lived_hour_bind::LIVED_TICK_PATH;
        use lived_sim_bridge::resumable_hour_from_tick_raw;
        use shared::climate_node::{AllocKind, LivedHour, TendResult};

        assert_eq!(LIVED_TICK_PATH, "data/powrush_lived_tick.json");

        let mut hour = LivedHour::new_demo();
        assert!(matches!(hour.tend(1), TendResult::Taken { .. }));
        assert_eq!(hour.satchel.count(), 1);
        let json = hour.to_json().expect("serialize");
        let loaded = resumable_hour_from_tick_raw(&json).expect("hour blob");
        assert_eq!(loaded.satchel.count(), 1);
        assert_eq!(loaded.allocation.flow, 0);
        assert_eq!(loaded.allocation.reserve, 0);

        let mut hour2 = LivedHour::new_demo();
        assert!(matches!(hour2.tend(1), TendResult::Taken { .. }));
        assert!(hour2.allocate(AllocKind::Reserve));
        let json2 = hour2.to_json().expect("serialize banked");
        let loaded2 = resumable_hour_from_tick_raw(&json2).expect("banked blob");
        assert_eq!(loaded2.allocation.reserve, 1);
        assert_eq!(loaded2.satchel.count(), 0);
        let line = loaded2.allocation.reserve_bank_line().expect("banked");
        assert!(line.contains("Reserve 1"));

        let again = LivedHour::from_json(&json2).expect("from_json");
        assert_eq!(again.allocation.reserve, loaded2.allocation.reserve);
        assert_eq!(again.satchel.count(), loaded2.satchel.count());
        q0_online_stays_grey();
    }

    /// CARD Q1 — dual HUD labels: stewardship / harm gate + stewardship quality.
    #[test]
    fn q1_dual_hud_labels_gate_and_quality() {
        use shared::contribution_events::{apply_event, ContributionEvent};
        use shared::contribution_ledger::ContributionLedger;
        use shared::nevc_visibility::{
            nevc_line_invents_wages, panel_fields_channels, status_line_for_event,
            stewardship_quality, summary_from_result,
        };

        let event = ContributionEvent::RbeAction {
            player_id: 21,
            abundance_alignment: 0.0,
            waste_or_harm: 0.0,
        };
        let mut ledger = ContributionLedger::new();
        let r = apply_event(&mut ledger, event.clone());
        let s = summary_from_result(&r);
        let line = status_line_for_event(&s, &event);
        assert!(line.contains("stewardship / harm gate"));
        assert!(line.contains("stewardship quality"));
        assert!(!line.to_lowercase().contains("gold"));
        assert!(!line.to_lowercase().contains("wages"));

        let fields = panel_fields_channels(&s, stewardship_quality(&event));
        assert_eq!(fields[1].0, "stewardship / harm gate");
        assert_eq!(fields[2].0, "stewardship quality");
        assert!(!nevc_line_invents_wages(&line));
        q0_online_stays_grey();
    }

    /// CARD Q1 — Title Online stays grey.
    #[test]
    fn q1_title_online_still_grey() {
        use title_screen::{
            f6_title_chrome_holds, l2_title_chrome_holds, s3_title_chrome_holds,
            TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };

        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(s3_title_chrome_holds());
        assert!(f6_title_chrome_holds());
        q0_online_stays_grey();
        q0_place_id_stays_three();
    }
}
