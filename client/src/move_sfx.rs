//! MOVE-SFX-1 — grounded footsteps. Distance on horizontal speed. Reads only.
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::*;
use shared::peace_audio::audio_output_safe;
use crate::companion_bond::CompanionBond;
use crate::human_presence::{SoftPresence, STRIDE_SPEED};
use crate::local_settings::MasterMuteGain;
pub const STEP_ASSET: &str = "audio/footstep.ogg";
pub const STEP_GAIN: f32 = 0.04; // committed peak −3.0 dBFS; x0.04 stays under −27.7
const STEP_METRES: f32 = 1.6;
pub fn step_due(acc: f32, speed: f32, grounded: bool, mounted: bool, dt: f32) -> (f32, bool) {
    if !grounded || mounted || speed <= STRIDE_SPEED { return (acc, false); }
    let next = acc + speed * dt;
    if next >= STEP_METRES { (next - STEP_METRES, true) } else { (next, false) }
}
pub fn step_volume(mute: Option<&MasterMuteGain>) -> Option<f32> {
    let gain = mute.map_or(1.0, |m| if m.muted { 0.0 } else { m.gain }) * STEP_GAIN;
    (gain > 0.0).then_some(gain)
}
fn play_step(cmd: &mut Commands, assets: Option<&AssetServer>, mute: Option<&MasterMuteGain>) {
    if let (true, Some(gain), Some(assets)) = (audio_output_safe(), step_volume(mute), assets) {
        cmd.spawn((AudioPlayer::<AudioSource>(assets.load(STEP_ASSET)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(gain))));
    }
}
fn footstep(time: Res<Time>, presence: Option<Res<SoftPresence>>, bond: Option<Res<CompanionBond>>, mut acc: Local<f32>, mut cmd: Commands, assets: Option<Res<AssetServer>>, mute: Option<Res<MasterMuteGain>>) {
    let Some(body) = presence.as_deref() else { return };
    let (next, due) = step_due(*acc, Vec2::new(body.velocity.x, body.velocity.z).length(), body.grounded, bond.as_ref().is_some_and(|b| b.mounted), time.delta_secs());
    *acc = next;
    if due { play_step(&mut cmd, assets.as_deref(), mute.as_deref()); }
}
pub struct MoveSfxPlugin;
impl Plugin for MoveSfxPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Update, footstep); }
}
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
