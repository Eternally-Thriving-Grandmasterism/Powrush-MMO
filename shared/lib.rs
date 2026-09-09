// shared/lib.rs
// Powrush-MMO — Shared Crate Root
// Phase 0–11 NEVC surfaces (including real-estate lattice attachment).
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai

pub mod protocol;
pub mod hex_protocol;
pub mod hex_join;
pub mod hex_shard_apply;
pub mod hex_listen;
pub mod climate_node;
pub mod climate_script;
pub mod shard_climate;
pub mod shard_standing;
pub mod week_audit;
pub mod stranger_loop_proof;
pub mod f_book_fixture;
pub mod user_persist;
pub mod hex_travel;
pub mod heartwood_lamp;
pub mod threshold_shelf;
pub mod heartwood_wards;
pub mod house_name;
pub mod local_settings;
pub mod peace_audio;
pub mod title_house_proof;
pub mod pause_ledger_face;
pub mod lived_tick_ingest;
pub mod powrush_gen;
pub mod genshare;
pub mod shard_sim;
pub mod net_mode;
pub mod shard_slots;
pub mod space_law;
pub mod hour_two;
pub mod vertical_factory;
pub mod coop_voice;
pub mod infra_spill;
pub mod ledger_bind;
pub mod fabricator;
pub mod embassy;
pub mod war_week;
pub mod crownstone;
pub mod mythic_verbs;
pub mod species_redemption;
pub mod hybrid_matrix;
pub mod compass;
pub mod skirmish_well;
pub mod nevc_adapter;
pub mod contribution_ledger;
pub mod contribution_events;
pub mod nevc_pipeline_demo;
pub mod real_estate_lattice_nevc;
pub mod nevc_game_loop;
pub mod nevc_persistence;
pub mod nevc_bridge;
pub mod nevc_visibility;

#[cfg(feature = "full_rbe")]
#[path = "rbe_queries.rs"]
pub mod rbe_queries;

#[cfg(not(feature = "full_rbe"))]
pub mod rbe_queries {
    pub fn stub_note() -> &'static str {
        "RBE deep queries available via Ra-Thor monorepo link. Thunder locked in."
    }
}

pub mod prelude {
    pub use crate::protocol::HotbarSlot;
    pub use crate::climate_node::{AllocKind, Allocation, ClimateNode, ClimateTake, LivedHour, NodeState, Satchel, TendResult};
    pub use crate::climate_script;
    pub use crate::shard_climate::ShardClimate;
    pub use crate::shard_standing::ShardStanding;
    pub use crate::week_audit::WeekAudit;
    pub use crate::user_persist::{persist_dir, persist_path, USER_DIR_OVERRIDE_ENV};
    pub use crate::hex_travel::{
        boot_place, confirm_leave, places_eligible, BootKind, PlaceId, ISOLATION_GAMMA,
    };
    pub use crate::heartwood_lamp::{try_place_building, BuildRefuse, HeartwoodYard};
    pub use crate::threshold_shelf::{visit_threshold, ThresholdShelfState, ThresholdVerb, THRESHOLD_SHELF_CENTER};
    pub use crate::heartwood_wards::{visit_wards, WardDress, WardVerb, WARD_POST_CENTERS};
    pub use crate::house_name::{HouseName, HOUSE_PATH, UNNAMED};
    pub use crate::local_settings::{LocalSettings, SETTINGS_PATH};
    pub use crate::peace_audio::{audio_output_safe, PeaceVoice};
    pub use crate::pause_ledger_face::{face_from, face_lines, ledger_sash_body, bind_only_before_settled_body, wait_line_before_settled, lethal_sign_eligible, lethal_sign_row, q_plate_seal_line, LETHAL_DECLARED_LINE, HEX_ADMITS_HARM, HEX_ADMITS_HARM_OFF, LEDGER_WAITS, NOT_YOUR_CHARTER};
    pub use crate::lived_tick_ingest::{ingest_enabled, soft_write_if_enabled, LivedTickIngest, LIVED_TICK_INGEST_PATH};
    pub use crate::powrush_gen::{parse_powrush_gen, light_gen_enabled, PowrushGen, cull_gen_when_plate_open, grove_seed};
    pub use crate::genshare::{GenShare, GENSHARE_PATH, append_genshare, load_genshare};
    pub use crate::shard_sim::ShardSim;
    pub use crate::net_mode::NetMode;
    pub use crate::hex_protocol::{Envelope, Op, Presence, RejectCode, Snapshot, PROTOCOL_ID, PROTOCOL_REV};
    pub use crate::hex_join::{AuthorityMode, JoinOutcome, attempt_join, leave_or_drop};
    pub use crate::hex_shard_apply::{apply_jsonl, apply_verb, soft_cap_houses, ShardLedger, SOFT_CAP_HOUSES, LISTEN_PARKED_MSG, LISTEN_LOOPBACK_MSG};
    pub use crate::hex_listen::{parse_powrush_net, PowrushNet, validate_listen_bind, client_may_outbound_ws};
    pub use crate::shard_slots::{ShardBank, ShardSlot};
    pub use crate::space_law::{CharterKind, HexFlag, SpaceSession, WarrantBand, WarrantWeight};
    pub use crate::hour_two::HourTwoPack;
    pub use crate::vertical_factory::{FactoryNodeKind, VerticalFactory};
    pub use crate::coop_voice::{CoopVoice, QuorumCard, VoiceTopic};
    pub use crate::infra_spill::{EvidencePack, InfraWitness, OffenseCode};
    pub use crate::ledger_bind::{LedgerBoard, LedgerContract, Purse, WinCondition};
    pub use crate::fabricator::{Fabricator, ProofPack, Recipe};
    pub use crate::embassy::{BlueprintBook, Embassy};
    pub use crate::war_week::WarWeek;
    pub use crate::crownstone::{CrownPath, CrownstoneState};
    pub use crate::mythic_verbs::{mythic_unlocked, MythicVerb};
    pub use crate::species_redemption::SpeciesRedemptionState;
    pub use crate::hybrid_matrix::HybridMatrix;
    pub use crate::compass;
    pub use crate::skirmish_well::{SkirmishWell, WellHold};
    pub use crate::rbe_queries;
    pub use crate::nevc_adapter::{ContributionClass, NevcSample, NevcResult, NevcConfig, NevcSummary, compute_nevc, score_instant, sample_from_rbe_action};
    pub use crate::contribution_ledger::{ContributionLedger, PlayerContribution};
    pub use crate::contribution_events::{ContributionEvent, apply_event, apply_event_class};
    pub use crate::nevc_pipeline_demo::{run_demo, classify};
    pub use crate::real_estate_lattice_nevc::{RealEstateStewardshipEvent, RealEstateNevcLedger, apply_real_estate_event, sample_from_stewardship, sample_from_event};
    pub use crate::nevc_game_loop::{HarvestNevcInput, harvest_to_event, apply_harvest_to_ledger, apply_harvest_class, apply_harvest_summary};
    pub use crate::nevc_persistence::{NevcPlayerRecord, NevcPersistenceStore};
    pub use crate::nevc_bridge::{compute_nevc_bridged, score_instant_bridged, summary_bridged, active_mode};
    pub use crate::nevc_visibility::{HorizonPreset, status_line, badge_text, summary_from_result, panel_fields};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_helpers_reachable() {
        let r = nevc_adapter::score_instant(0.999999, 0.0);
        let s = nevc_visibility::summary_from_result(&r);
        assert_eq!(nevc_visibility::badge_text(s.class), "Contributor");
    }

    #[test]
    fn real_estate_ledger_reachable() {
        let mut rrel = real_estate_lattice_nevc::RealEstateNevcLedger::new();
        let r = rrel.apply(real_estate_lattice_nevc::RealEstateStewardshipEvent::Stewardship {
            agent_id: 1,
            alignment: 1.0,
        });
        assert!(r.is_contributor());
    }

    #[test]
    fn space_law_peace_is_sacred() {
        let s = space_law::SpaceSession::default();
        assert_eq!(s.warrant_live(), 0.0);
        assert!(!s.charter_skin_live());
    }

    #[test]
    fn vertical_factory_arrives() {
        let mut f = vertical_factory::VerticalFactory::default();
        f.found_house();
        for _ in 0..6 {
            f.advance();
        }
        assert!(f.tutorial_complete());
    }

    #[test]
    fn coop_voice_carries() {
        let mut v = coop_voice::CoopVoice::default();
        v.ensure_tutorial();
        assert_eq!(v.vote_local(true), "carried");
    }

    #[test]
    fn infra_spill_witness() {
        let mut w = infra_spill::InfraWitness::default();
        w.ensure_offline_extractor();
        assert!(w.visible_on(space_law::HexFlag::Frontier));
        assert!(!w.visible_on(space_law::HexFlag::Peace));
    }

    #[test]
    fn ledger_bind_settles() {
        let mut b = ledger_bind::LedgerBoard::default();
        b.ensure_i2("test");
        assert_eq!(b.act_local(), "bound");
        assert_eq!(b.act_local(), "escorting");
        assert_eq!(b.act_local(), "escorting");
        assert_eq!(b.act_local(), "settled");
    }

    #[test]
    fn fabricator_unlocks() {
        let mut f = fabricator::Fabricator::default();
        assert_eq!(f.craft_next(), "planted");
        assert_eq!(f.craft_next(), "crafted");
        assert_eq!(f.craft_next(), "unlocked");
        assert!(f.pack.unlocked());
    }

    #[test]
    fn embassy_seats() {
        let mut f = fabricator::Fabricator::default();
        f.craft_next();
        f.craft_next();
        f.craft_next();
        let mut e = embassy::Embassy::default();
        e.ensure_lamp(&f.pack);
        assert_eq!(e.request_seat(), "seated");
    }

    #[test]
    fn war_week_scores() {
        let mut w = war_week::WarWeek::default();
        w.declare();
        w.ingest(1.0, 1);
        assert_eq!(w.score(), 2.0);
    }

    #[test]
    fn lethal_is_opt_in() {
        let mut c = ledger_bind::LedgerContract::from_i2("z");
        assert_eq!(c.win, ledger_bind::WinCondition::BindEscort);
        assert_eq!(c.opt_lethal(), "lethal");
        assert_eq!(c.bind(), "idle");
    }

    #[test]
    fn crownstone_is_seen() {
        let mut c = crownstone::CrownstoneState::default();
        assert_eq!(c.path, crownstone::CrownPath::Unset);
        assert_eq!(c.witness(), "witnessed");
        assert_eq!(c.path, crownstone::CrownPath::Unset);
    }

    #[test]
    fn sylvaris_tend() {
        let mut s = species_redemption::SpeciesRedemptionState::default();
        assert_eq!(s.offer_tend(), "tended");
        assert!(s.sylvaris > 0.0);
        assert_eq!(s.veythari, 0.0);
    }

    #[test]
    fn hybrid_attunes() {
        let mut h = hybrid_matrix::HybridMatrix::default();
        assert_eq!(h.attune(), "attuned");
        assert_eq!(h.stability, 1.0);
    }

    #[test]
    fn compass_tells() {
        let mut w = space_law::WarrantWeight::default();
        w.x = 10.0;
        assert!(compass::tell(&w, space_law::HexFlag::Frontier).unwrap().contains("20"));
        assert_eq!(compass::tell(&w, space_law::HexFlag::Peace), None);
    }

    #[test]
    fn skirmish_well_contests() {
        let mut w = skirmish_well::SkirmishWell::default();
        assert_eq!(w.act(), "won");
        assert_eq!(w.traveler_answers(), "lost");
        assert_eq!(w.act(), "dawn");
    }

    #[test]
    fn mythic_verbs_not_damage() {
        assert!(!mythic_verbs::MythicVerb::Witness.is_damage());
        assert!(mythic_verbs::mythic_unlocked(true, true));
    }

    #[test]
    fn shard_slots_diverge_thrive_vs_poor() {
        let b = shard_slots::ShardBank::default();
        assert!(b.thrive.climate.harmony > b.poor.climate.harmony);
    }

    #[test]
    fn net_mode_offline_default_no_fake_peers() {
        assert_eq!(net_mode::NetMode::default(), net_mode::NetMode::Offline);
        assert!(net_mode::NetMode::Offline.peer_count_for_peace_boot().is_none());
    }

    #[test]
    fn hex_protocol_rev_and_no_listen() {
        assert_eq!(hex_protocol::PROTOCOL_ID, "powrush.hex.v1");
        assert_eq!(hex_protocol::PROTOCOL_REV, 1);
        assert!(!hex_protocol::default_client_listens());
        assert!(!hex_protocol::server_unparked_v1());
    }

    #[test]
    fn hex_join_copy_denied_stays_offline() {
        let o = hex_join::attempt_join(false, false, hex_protocol::PROTOCOL_REV);
        assert_eq!(hex_join::authority_after_join(&o), hex_join::AuthorityMode::Offline);
        assert!(!hex_join::join_slice_listens());
        assert!(!hex_join::join_slice_unparks_server());
    }

    #[test]
    fn shard_sim_eases_stress_offline() {
        let mut sim = shard_sim::ShardSim::default();
        let mut c = shard_climate::ShardClimate { stress: 0.7, regen: 0.25, ..Default::default() };
        let mut s = shard_standing::ShardStanding::default();
        let mut w = week_audit::WeekAudit::default();
        let after = sim.lapse(&mut c, &mut s, &mut w, 5);
        assert!(after < 0.7);
    }

    #[test]
    fn week_audit_slab_is_tons_plus_restored() {
        let mut w = week_audit::WeekAudit::default();
        w.sync_from_climate(2, 5);
        let line = w.slab_line();
        assert!(line.contains("tons"));
        assert!(line.contains("restored"));
    }

    #[test]
    fn shard_standing_lethal_parked() {
        let s = shard_standing::ShardStanding::default();
        assert!(!s.declared_lethal);
        assert_eq!(s.human_hybrid_heat, 0.0);
    }

    #[test]
    fn u2_hex_travel_disk_only_gamma_zero_play_sanctuary() {
        assert_eq!(hex_travel::ISOLATION_GAMMA, 0.0);
        assert!(!hex_travel::leak_tick_enabled());
        assert_eq!(
            hex_travel::boot_place(
                hex_travel::BootKind::Play,
                true,
                Some(hex_travel::PlaceId::Heartwood)
            ),
            hex_travel::PlaceId::Sanctuary
        );
        assert!(!hex_travel::may_enter(hex_travel::PlaceId::Heartwood, false));
        assert!(hex_travel::travel_is_disk_only());
        assert!(!hex_protocol::default_client_listens());
    }

    #[test]
    fn u3_heartwood_lamp_refuses_water_and_disk() {
        use heartwood_lamp::{
            try_place_building, BuildRefuse, LAMP_DISK_CENTER, LIP_BUILD_POINT, WATER_POND_CENTER,
        };
        assert_eq!(
            try_place_building(
                hex_travel::PlaceId::Heartwood,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            Err(BuildRefuse::Water)
        );
        assert_eq!(
            try_place_building(
                hex_travel::PlaceId::Heartwood,
                LAMP_DISK_CENTER[0],
                LAMP_DISK_CENTER[1]
            ),
            Err(BuildRefuse::LampDisk)
        );
        assert!(try_place_building(
            hex_travel::PlaceId::Heartwood,
            LIP_BUILD_POINT[0],
            LIP_BUILD_POINT[1]
        )
        .is_ok());
        assert!(!heartwood_lamp::embassy_lamp_is_spatial_gate());
        assert!(!heartwood_lamp::heartwood_mesh_on_sanctuary());
        assert_eq!(hex_travel::ISOLATION_GAMMA, 0.0);
    }

    #[test]
    fn u4_mute_silences_bed_and_sting_unmute_opens_no_socket() {
        use peace_audio::PeaceVoice;
        let mut settings = local_settings::LocalSettings::peace_defaults();
        settings.mute = true;
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!(voice.note_well_use(1, 1));
        assert!((voice.take_sting() - 0.0).abs() < f32::EPSILON);
        voice.set_mute(false);
        assert!(voice.should_play_bed());
        assert!(voice.note_well_use(2, 1));
        assert!(voice.take_sting() > 0.0);
        assert!(!voice.opens_socket());
        assert!(!peace_audio::peace_audio_opens_socket(&voice));
        assert!(peace_audio::title_online_stays_grey());
        assert!(!hex_protocol::default_client_listens());
        assert_eq!(hex_travel::ISOLATION_GAMMA, 0.0);
        assert!(!shard_standing::ShardStanding::default().declared_lethal);
    }

    #[test]
    fn l1_hex_sign_default_off_needs_settled_book_no_ton_mint() {
        let mut standing = shard_standing::ShardStanding::default();
        let mut climate = shard_climate::ShardClimate::default();
        climate.tons_moved = 3;
        climate.reserve_pool = 1;
        assert!(!standing.confirm_hex_sign(false, true));
        assert!(!standing.confirm_hex_sign(true, false));
        assert!(!standing.declared_lethal);
        assert!(standing.confirm_hex_sign(true, true));
        let paid = climate.on_lethal_declare();
        assert_eq!(paid, 1);
        assert_eq!(climate.tons_moved, 3);
        assert!(standing.declared_lethal);
        assert_eq!(
            pause_ledger_face::lethal_sign_row(true, true, true, true),
            pause_ledger_face::HEX_ADMITS_HARM
        );
        assert_eq!(hex_protocol::reject_declare_lethal_before_book(false), Err(hex_protocol::RejectCode::NoBook));
        assert_eq!(
            hex_protocol::reject_declare_lethal_before_settled(false),
            Err(hex_protocol::RejectCode::NotCharter)
        );
        assert!(!hex_protocol::default_client_listens());
    }

    #[test]
    fn shard_climate_flow_lowers_stress() {
        let mut c = shard_climate::ShardClimate::default();
        c.stress = 0.6;
        c.on_flow();
        assert!(c.stress < 0.6);
    }

    #[test]
    fn teaching_climates_reachable() {
        let tired = climate_script::run_extract_only();
        assert!(climate_script::extract_only_holds(&tired, 1));
        let kind = climate_script::run_mercy_restore();
        assert!(climate_script::mercy_restore_holds(&kind, 1));
    }

    #[test]
    fn hour_two_pack_reads_old_session() {
        let pack = hour_two::HourTwoPack::from_json(
            r#"{"hex":"Peace","kind":"PatchworkFirm"}"#,
        );
        assert!(!pack.complete);
        assert_eq!(pack.session.hex, space_law::HexFlag::Peace);
    }
}
