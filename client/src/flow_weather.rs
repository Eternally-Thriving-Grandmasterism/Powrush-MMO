/*!
 * Flow Weather — v22.14.0 + EARTH-CLIMATE Place coupling (H-2026-09-12)
 *
 * Ribbon + hidden band + solo awe inhale + grove nectar current.
 * No Flow % bar. No Horizon pips (H already hides guidance).
 * No second HUD. No live Earth API / sockets.
 *
 * EARTH-CLIMATE: band writes [`WeatherBandCoupling`] for climate fog/breath
 * beds; ribbon glow tints by Place mood; Comfort [`WeatherFidelity`] gates
 * bead scale / drop cadence (Low gentler · Medium default · High richer).
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use shared::local_settings::WeatherFidelity;

use crate::climate_plane::{
    place_mood_for, PlaceMood, WeatherBandCoupling, WeatherBandKind,
};
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::living_body::LivingBody;
use crate::living_freshness::LivingFreshness;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_settings::LocalSettingsState;

const DROP_SECS: f32 = 0.11;
const BEAD_LIFE: f32 = 0.72;
const AWE_SECS: f32 = 5.2;
const NECTAR_FROM: Vec3 = Vec3::new(-5.2, 1.15, -1.4);
const NECTAR_TO: Vec3 = Vec3::new(4.6, 1.55, 2.8);

fn flow_band_to_coupling(band: FlowBand) -> WeatherBandKind {
    match band {
        FlowBand::Rise => WeatherBandKind::Rise,
        FlowBand::Flow => WeatherBandKind::Flow,
        FlowBand::Boredom => WeatherBandKind::Boredom,
        FlowBand::Anxiety => WeatherBandKind::Anxiety,
    }
}

/// Place-mood tint over a FlowWeather band glow (procedural; no asset pack).
pub fn place_mood_glow(mood: PlaceMood, band: FlowBand, fidelity: WeatherFidelity) -> LinearRgba {
    let base = match band {
        FlowBand::Flow => LinearRgba::new(1.0, 0.96, 0.62, 1.0),
        FlowBand::Rise => LinearRgba::new(0.72, 0.88, 0.70, 1.0),
        FlowBand::Anxiety => LinearRgba::new(0.35, 0.40, 0.48, 1.0),
        FlowBand::Boredom => LinearRgba::new(0.28, 0.32, 0.30, 1.0),
    };
    let (tr, tg, tb) = match mood {
        PlaceMood::SanctuarySkyYard => (1.06, 0.98, 0.88),
        PlaceMood::HeartwoodCanopy => (0.92, 1.08, 0.85),
        PlaceMood::ThresholdPipeAir => (0.88, 0.92, 1.10),
        PlaceMood::DepthsWetStone => (0.78, 1.05, 1.12),
    };
    let i = fidelity.intensity();
    LinearRgba::new(
        (base.red * tr * (0.85 + 0.15 * i)).min(1.35),
        (base.green * tg * (0.85 + 0.15 * i)).min(1.35),
        (base.blue * tb * (0.85 + 0.15 * i)).min(1.35),
        1.0,
    )
}

/// Drop cadence scaled by weather fidelity (Low gentler / fewer beads).
pub fn drop_secs_for(fidelity: WeatherFidelity) -> f32 {
    match fidelity {
        WeatherFidelity::Low => DROP_SECS * 1.55,
        WeatherFidelity::Medium => DROP_SECS,
        WeatherFidelity::High => DROP_SECS * 0.82,
    }
}

/// Bead scale gated by fidelity + band.
pub fn bead_scale_for(band: FlowBand, inhaling: bool, fidelity: WeatherFidelity) -> f32 {
    let base = if inhaling || band == FlowBand::Flow {
        1.2
    } else {
        0.85
    };
    base * match fidelity {
        WeatherFidelity::Low => 0.78,
        WeatherFidelity::Medium => 1.0,
        WeatherFidelity::High => 1.18,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBand {
    Rise,
    Flow,
    Boredom,
    Anxiety,
}

#[derive(Resource, Debug)]
pub struct FlowWeather {
    pub band: FlowBand,
    pub chain: f32,
    pub awe_until: f64,
    last_harvests: u32,
}

impl Default for FlowWeather {
    fn default() -> Self {
        Self {
            band: FlowBand::Rise,
            chain: 0.0,
            awe_until: 0.0,
            last_harvests: 0,
        }
    }
}

impl FlowWeather {
    pub fn inhaling(&self, now: f64) -> bool {
        now < self.awe_until
    }
}

#[derive(Component)]
struct RibbonBead {
    born: f32,
    handle: Handle<StandardMaterial>,
    glow: LinearRgba,
}

#[derive(Component)]
struct NectarBead;

#[derive(Resource)]
struct RibbonKit {
    mesh: Handle<Mesh>,
}

pub struct FlowWeatherPlugin;

impl Plugin for FlowWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlowWeather>()
            .init_resource::<WeatherBandCoupling>()
            .add_systems(Startup, (seed_ribbon, seed_nectar))
            .add_systems(
                Update,
                (
                    read_band,
                    maybe_awe,
                    apply_awe_light,
                    drop_beads,
                    fade_beads,
                    ride_nectar,
                    show_nectar,
                ),
            );
    }
}

fn seed_ribbon(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(RibbonKit {
        mesh: meshes.add(Sphere::new(0.06)),
    });
}

fn seed_nectar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.09));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.95, 0.70, 0.55),
        emissive: LinearRgba::new(0.35, 0.55, 0.28, 1.0),
        perceptual_roughness: 0.4,
        ..default()
    });
    let delta = NECTAR_TO - NECTAR_FROM;
    for i in 0..9 {
        let t = i as f32 / 8.0;
        let pos = NECTAR_FROM + delta * t;
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: mat.clone(),
                transform: Transform::from_translation(pos),
                visibility: Visibility::Hidden,
                ..default()
            },
            NectarBead,
        ));
    }
}

fn read_band(
    body: Option<Res<LivingBody>>,
    fresh: Option<Res<LivingFreshness>>,
    harvest: Option<Res<FirstHarvestEpiphany>>,
    presence: Res<SoftPresence>,
    time: Res<Time>,
    mut weather: ResMut<FlowWeather>,
    mut coupling: ResMut<WeatherBandCoupling>,
) {
    let speed = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z).length();
    let moving = speed > 0.4 || !presence.grounded;
    if moving {
        weather.chain = (weather.chain + time.delta_seconds()).min(12.0);
    } else {
        weather.chain = (weather.chain - time.delta_seconds() * 0.55).max(0.0);
    }
    let winded = body.as_ref().map(|b| b.winded).unwrap_or(false);
    let heavy = body.as_ref().map(|b| b.heavy).unwrap_or(false);
    let aging = fresh.map(|f| f.age > 28.0).unwrap_or(false);
    let quiet = harvest
        .map(|h| h.harvests_this_session == 0 && h.tends_this_session == 0)
        .unwrap_or(false);
    weather.band = if winded || heavy {
        FlowBand::Anxiety
    } else if quiet && weather.chain < 1.2 {
        FlowBand::Boredom
    } else if weather.chain > 3.5 && !aging {
        FlowBand::Flow
    } else {
        FlowBand::Rise
    };
    coupling.band = flow_band_to_coupling(weather.band);
}

fn maybe_awe(
    harvest: Option<Res<FirstHarvestEpiphany>>,
    time: Res<Time>,
    mut weather: ResMut<FlowWeather>,
) {
    let Some(h) = harvest else {
        return;
    };
    let total = h.harvests_this_session + h.tends_this_session;
    if total == weather.last_harvests {
        return;
    }
    weather.last_harvests = total;
    if weather.chain >= 2.4 || weather.band == FlowBand::Flow {
        weather.awe_until = time.elapsed_seconds_f64() + AWE_SECS as f64;
        info!(target: "powrush::flow", "solo world inhale");
    }
}

fn apply_awe_light(
    weather: Res<FlowWeather>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
) {
    if weather.inhaling(time.elapsed_seconds_f64()) {
        ambient.brightness = (ambient.brightness + 90.0).min(420.0);
    }
}

fn drop_beads(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kit: Option<Res<RibbonKit>>,
    presence: Res<SoftPresence>,
    weather: Res<FlowWeather>,
    realm: Res<SoftPlayerRealm>,
    settings: Option<Res<LocalSettingsState>>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    let Some(kit) = kit else {
        return;
    };
    let fidelity = settings
        .as_ref()
        .map(|s| s.inner.weather_fidelity())
        .unwrap_or(WeatherFidelity::Medium);
    let speed = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z).length();
    if speed < 0.35 && presence.grounded {
        return;
    }
    let now = time.elapsed_seconds();
    if now - *last < drop_secs_for(fidelity) {
        return;
    }
    *last = now;
    let inhaling = weather.inhaling(time.elapsed_seconds_f64());
    let mood = place_mood_for(realm.current.or(Some(0)));
    let glow = if inhaling {
        let g = place_mood_glow(mood, FlowBand::Flow, fidelity);
        LinearRgba::new(g.red * 1.12, g.green * 1.08, g.blue * 0.95, 1.0)
    } else {
        place_mood_glow(mood, weather.band, fidelity)
    };
    let handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.95, 0.78),
        emissive: glow,
        perceptual_roughness: 0.35,
        ..default()
    });
    let scale = bead_scale_for(weather.band, inhaling, fidelity);
    commands.spawn((
        PbrBundle {
            mesh: kit.mesh.clone(),
            material: handle.clone(),
            transform: Transform::from_translation(presence.position - Vec3::Y * 0.55)
                .with_scale(Vec3::splat(scale)),
            ..default()
        },
        RibbonBead {
            born: now,
            handle,
            glow,
        },
    ));
}

fn fade_beads(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &RibbonBead)>,
) {
    let now = time.elapsed_seconds();
    for (entity, bead) in &q {
        let age = now - bead.born;
        if age > BEAD_LIFE {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        let t = (1.0 - age / BEAD_LIFE).clamp(0.04, 1.0);
        if let Some(mat) = materials.get_mut(&bead.handle) {
            mat.emissive = LinearRgba::new(
                bead.glow.red * t,
                bead.glow.green * t,
                bead.glow.blue * t,
                1.0,
            );
        }
    }
}

fn show_nectar(
    realm: Res<SoftPlayerRealm>,
    mut q: Query<&mut Visibility, With<NectarBead>>,
) {
    let show = matches!(realm.current, Some(0) | Some(2));
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn ride_nectar(
    realm: Res<SoftPlayerRealm>,
    mut presence: ResMut<SoftPresence>,
) {
    if !matches!(realm.current, Some(0) | Some(2)) {
        return;
    }
    let p = presence.position;
    let delta = NECTAR_TO - NECTAR_FROM;
    let len2 = delta.length_squared().max(0.01);
    let t = ((p - NECTAR_FROM).dot(delta) / len2).clamp(0.0, 1.0);
    let nearest = NECTAR_FROM + delta * t;
    if p.distance(nearest) > 1.35 {
        return;
    }
    let dir = delta.normalize();
    presence.velocity.x += dir.x * 1.15;
    presence.velocity.z += dir.z * 1.15;
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::GraphicsPreset;

    #[test]
    fn place_mood_glow_differs_across_places() {
        let fidelity = WeatherFidelity::Medium;
        let s = place_mood_glow(PlaceMood::SanctuarySkyYard, FlowBand::Rise, fidelity);
        let h = place_mood_glow(PlaceMood::HeartwoodCanopy, FlowBand::Rise, fidelity);
        let t = place_mood_glow(PlaceMood::ThresholdPipeAir, FlowBand::Rise, fidelity);
        let d = place_mood_glow(PlaceMood::DepthsWetStone, FlowBand::Rise, fidelity);
        assert_ne!((s.red, s.green, s.blue), (h.red, h.green, h.blue));
        assert_ne!((s.red, s.green, s.blue), (t.red, t.green, t.blue));
        assert_ne!((s.red, s.green, s.blue), (d.red, d.green, d.blue));
        // Heartwood canopy leans green; Threshold pipe air leans cool blue.
        assert!(h.green >= s.green);
        assert!(t.blue >= s.blue);
        assert!(d.blue >= s.blue);
    }

    #[test]
    fn graphics_preset_gates_flow_weather_fidelity() {
        let low = WeatherFidelity::Low;
        let mid = WeatherFidelity::Medium;
        let high = WeatherFidelity::High;
        assert!(drop_secs_for(low) > drop_secs_for(mid));
        assert!(drop_secs_for(high) < drop_secs_for(mid));
        assert!(bead_scale_for(FlowBand::Rise, false, low) < bead_scale_for(FlowBand::Rise, false, mid));
        assert!(bead_scale_for(FlowBand::Flow, true, high) > bead_scale_for(FlowBand::Flow, true, mid));

        assert_eq!(GraphicsPreset::Low.weather_fidelity(), low);
        assert_eq!(GraphicsPreset::Medium.weather_fidelity(), mid);
        assert_eq!(GraphicsPreset::High.weather_fidelity(), high);

        let glow_low = place_mood_glow(PlaceMood::SanctuarySkyYard, FlowBand::Flow, low);
        let glow_high = place_mood_glow(PlaceMood::SanctuarySkyYard, FlowBand::Flow, high);
        let lum_low = glow_low.red + glow_low.green + glow_low.blue;
        let lum_high = glow_high.red + glow_high.green + glow_high.blue;
        assert!(lum_high > lum_low);
    }

    #[test]
    fn band_coupling_token_maps_all_flow_bands() {
        assert_eq!(flow_band_to_coupling(FlowBand::Rise), WeatherBandKind::Rise);
        assert_eq!(flow_band_to_coupling(FlowBand::Flow), WeatherBandKind::Flow);
        assert_eq!(flow_band_to_coupling(FlowBand::Boredom), WeatherBandKind::Boredom);
        assert_eq!(flow_band_to_coupling(FlowBand::Anxiety), WeatherBandKind::Anxiety);
    }

    #[test]
    fn flow_weather_refuses_online_socket_and_live_earth() {
        // Pure honesty: coupling helpers never mention Online / sockets / Earth API.
        for mood in [
            PlaceMood::SanctuarySkyYard,
            PlaceMood::HeartwoodCanopy,
            PlaceMood::ThresholdPipeAir,
            PlaceMood::DepthsWetStone,
        ] {
            let label = mood.mood_label();
            let lower = label.to_ascii_lowercase();
            assert!(!lower.contains("online"));
            assert!(!lower.contains("socket"));
            assert!(!lower.contains("market"));
            assert!(!label.contains("http"));
        }
        for fidelity in WeatherFidelity::ALL {
            let feel = fidelity.feel_label();
            let lower = feel.to_ascii_lowercase();
            assert!(!lower.contains("online"));
            assert!(!lower.contains("earth api"));
            assert!(!feel.contains("ws://"));
        }
    }
}
