#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;
    use shared::peace_audio::audio_output_safe;
    use crate::human_presence::{SoftPresence, STRIDE_SPEED}; use crate::local_settings::MasterMuteGain;
    fn count(speed: f32, grounded: bool, mounted: bool) -> usize {
        let (mut acc, mut n, dt) = (0.0, 0usize, 1.0 / 60.0);
        for _ in 0..120 { let (a, due) = super::step_due(acc, speed, grounded, mounted, dt); acc = a; n += usize::from(due); }
        n
    }
    #[test]
    fn ms1_walk_fewer_than_sprint_idle_zero() { assert!(count(3.4, true, false) < count(5.4, true, false) && count(0.0, true, false) == 0 && count(STRIDE_SPEED, true, false) == 0); }
    #[test]
    fn ms2_air_or_mounted_silent() { assert_eq!((count(5.4, false, false), count(5.4, true, true)), (0, 0)); }
    #[test]
    fn ms3_mute_or_zero_none() {
        let (m, z) = (MasterMuteGain { gain: 1.0, muted: true }, MasterMuteGain { gain: 0.0, muted: false });
        assert_eq!((super::step_volume(Some(&m)), super::step_volume(Some(&z))), (None, None));
    }
    #[test]
    fn ms4_spawn_licence_gain() {
        assert_eq!(super::STEP_GAIN, 0.04);
        assert!(include_str!("../../assets/asset-licenses.csv").contains("assets/audio/footstep.ogg"));
        let expect = audio_output_safe() as usize;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default())).init_asset::<AudioSource>().insert_resource(SoftPresence { velocity: Vec3::new(5.4, 0.0, 0.0), grounded: true, ..Default::default() }).insert_resource(MasterMuteGain { gain: 1.0, muted: false });
        app.world_mut().resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(1.0));
        app.world_mut().run_system_once(super::footstep).unwrap();
        assert_eq!(app.world_mut().query::<&AudioPlayer>().iter(app.world_mut()).count(), expect);
    }
}
