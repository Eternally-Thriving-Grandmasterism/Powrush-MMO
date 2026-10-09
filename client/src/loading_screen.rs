//! CARD LOADING-SCREEN-1 — boot overlay above the title.
//!
//! The bar is the asset server's load states for handles already in the
//! world (plus the default UI font id when `Assets<Font>` has it). It does
//! not advance on a timer. No images. No new verbs. Title Online stays grey.
//! Contact: info@Rathor.ai

use std::collections::HashSet;
use std::path::Path;

use bevy::asset::{AssetServer, LoadState, RecursiveDependencyLoadState, UntypedAssetId};
use bevy::audio::AudioSource;
use bevy::ecs::schedule::common_conditions::any_with_component;
use bevy::prelude::*;
use bevy::image::Image;
use bevy::ui::FocusPolicy;

use crate::loading_lines::LOADING_LINES;
use crate::ui_above_world::LivedUiPlate;

/// Above the title persona plate (`title_screen` `GlobalZIndex(142)`).
const LOADING_Z: i32 = 200;
const LINE_INTERVAL_SECS: f32 = 3.5;
const MIN_VISIBLE_SECS: f32 = 1.5;

/// Bevy's default file asset root. Byte totals are file lengths here only.
const ASSET_ROOT: &str = "assets";

/// Deep violet veil. Opaque so the title does not show through muddy.
const VEIL: Color = Color::srgb(0.07, 0.03, 0.14);
/// Saturated violet panel. Dark enough for cream text, not neon.
const PANEL: Color = Color::srgb(0.30, 0.11, 0.46);
/// Gold with a rose cast. The frame, not a brown bronze.
const FRAME: Color = Color::srgb(0.93, 0.62, 0.50);
/// Darker violet track under the bar.
const TRACK: Color = Color::srgb(0.13, 0.05, 0.22);
/// Bright gold/rose fill. Short of pure yellow and short of neon.
const FILL: Color = Color::srgb(0.98, 0.74, 0.52);
const TEXT_CREAM: Color = Color::srgb(0.98, 0.95, 0.90);
const TEXT_ROSE: Color = Color::srgb(0.96, 0.82, 0.76);

/// `POWRUSH_Q2_FRAME` set → do not spawn. Q1 `--script-run` is not a switch.
fn should_spawn(q2_frame_set: bool) -> bool {
    !q2_frame_set
}

fn q2_frame_set() -> bool {
    std::env::var_os("POWRUSH_Q2_FRAME").is_some()
}

/// Index of the flavour line at `elapsed_secs`. Wraps. Empty list stays 0.
fn line_index(elapsed_secs: f32, line_count: usize) -> usize {
    if line_count == 0 {
        return 0;
    }
    let elapsed = if elapsed_secs.is_finite() && elapsed_secs > 0.0 {
        elapsed_secs
    } else {
        0.0
    };
    let steps = (elapsed / LINE_INTERVAL_SECS).floor();
    if !steps.is_finite() || steps <= 0.0 {
        return 0;
    }
    (steps as usize) % line_count
}

fn line_at(elapsed_secs: f32) -> &'static str {
    let count = LOADING_LINES.len();
    if count == 0 {
        return "";
    }
    LOADING_LINES[line_index(elapsed_secs, count)]
}

/// `tracked == 0` means nothing is in flight: the bar is full.
fn progress_fraction(settled: usize, tracked: usize) -> f32 {
    if tracked == 0 {
        return 1.0;
    }
    (settled as f32 / tracked as f32).clamp(0.0, 1.0)
}

fn overlay_ready(fraction: f32, elapsed_secs: f32) -> bool {
    fraction >= 1.0 && elapsed_secs >= MIN_VISIBLE_SECS
}

/// Percent, plus `X.X / Y.Y MB` only when both byte counts are real.
fn progress_label(fraction: f32, loaded_bytes: Option<u64>, total_bytes: Option<u64>) -> String {
    let pct = (fraction.clamp(0.0, 1.0) * 100.0).round();
    let pct = if pct.is_finite() { pct as u32 } else { 0 };
    match (loaded_bytes, total_bytes) {
        (Some(loaded), Some(total)) => format!(
            "{pct}%\n{:.1} / {:.1} MB",
            loaded as f64 / 1_048_576.0,
            total as f64 / 1_048_576.0
        ),
        _ => format!("{pct}%"),
    }
}

/// `Some(true)` settled, `Some(false)` still loading, `None` not an asset-server load.
fn classify_load(server: &AssetServer, id: UntypedAssetId) -> Option<bool> {
    if !server.is_managed(id) {
        return None;
    }
    match server.get_load_state(id) {
        Some(LoadState::Loading) => Some(false),
        Some(LoadState::Failed(_)) => Some(true),
        Some(LoadState::Loaded) => match server.get_recursive_dependency_load_state(id) {
            Some(RecursiveDependencyLoadState::Loaded)
            | Some(RecursiveDependencyLoadState::Failed(_)) => Some(true),
            Some(RecursiveDependencyLoadState::Loading)
            | Some(RecursiveDependencyLoadState::NotLoaded) => Some(false),
            None => Some(true),
        },
        Some(LoadState::NotLoaded) | None => None,
    }
}

fn on_disk_len(server: &AssetServer, id: UntypedAssetId) -> Option<u64> {
    let asset_path = server.get_path(id)?;
    let full = Path::new(ASSET_ROOT).join(asset_path.path());
    let meta = std::fs::metadata(full).ok()?;
    meta.is_file().then(|| meta.len())
}

struct BootSnapshot {
    settled: usize,
    tracked: usize,
    loaded_bytes: Option<u64>,
    total_bytes: Option<u64>,
}

fn boot_snapshot(server: Option<&AssetServer>, ids: &[UntypedAssetId]) -> BootSnapshot {
    let Some(server) = server else {
        return BootSnapshot {
            settled: 0,
            tracked: 0,
            loaded_bytes: None,
            total_bytes: None,
        };
    };
    let mut seen = HashSet::new();
    let mut settled = 0usize;
    let mut tracked = 0usize;
    let mut loaded_bytes = 0u64;
    let mut total_bytes = 0u64;
    let mut bytes_complete = true;
    for id in ids {
        if !seen.insert(*id) {
            continue;
        }
        let Some(done) = classify_load(server, *id) else {
            continue;
        };
        tracked += 1;
        if done {
            settled += 1;
        }
        match on_disk_len(server, *id) {
            Some(len) => {
                total_bytes = total_bytes.saturating_add(len);
                if done {
                    loaded_bytes = loaded_bytes.saturating_add(len);
                }
            }
            None => bytes_complete = false,
        }
    }
    let (loaded_bytes, total_bytes) = if tracked > 0 && bytes_complete {
        (Some(loaded_bytes), Some(total_bytes))
    } else {
        (None, None)
    };
    BootSnapshot {
        settled,
        tracked,
        loaded_bytes,
        total_bytes,
    }
}

fn push_observed(
    ids: &mut Vec<UntypedAssetId>,
    fonts: Option<&Assets<Font>>,
    texts: impl IntoIterator<Item = AssetId<Font>>,
    images: impl IntoIterator<Item = AssetId<Image>>,
    audio: impl IntoIterator<Item = AssetId<AudioSource>>,
) {
    if let Some(fonts) = fonts {
        let id = AssetId::<Font>::default();
        if fonts.get(id).is_some() {
            ids.push(id.untyped());
        }
    }
    ids.extend(texts.into_iter().map(|id| id.untyped()));
    ids.extend(images.into_iter().map(|id| id.untyped()));
    ids.extend(audio.into_iter().map(|id| id.untyped()));
}

#[derive(Component)]
struct LoadingRoot {
    spawned_at: f32,
}

#[derive(Component)]
struct LoadingBarFill;

#[derive(Component)]
struct LoadingPercent;

#[derive(Component)]
struct LoadingFlavour;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_loading_overlay).add_systems(
            Update,
            refresh_loading_overlay.run_if(any_with_component::<LoadingRoot>),
        );
    }
}

fn spawn_loading_overlay(
    mut commands: Commands,
    time: Res<Time<Real>>,
    server: Option<Res<AssetServer>>,
    fonts: Option<Res<Assets<Font>>>,
    texts: Query<&TextFont>,
    images: Query<&ImageNode>,
    audio: Query<&AudioPlayer>,
) {
    if !should_spawn(q2_frame_set()) {
        return;
    }
    let spawned_at = time.elapsed_secs();
    let mut ids = Vec::new();
    push_observed(
        &mut ids,
        fonts.as_deref(),
        texts.iter().map(|font| font.font.id()),
        images.iter().map(|node| node.image.id()),
        audio.iter().map(|player| player.0.id()),
    );
    let snap = boot_snapshot(server.as_deref(), &ids);
    let fraction = progress_fraction(snap.settled, snap.tracked);
    let percent = progress_label(fraction, snap.loaded_bytes, snap.total_bytes);
    let flavour = line_at(0.0).to_string();

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                background_color: VEIL.into(),
                                focus_policy: FocusPolicy::Block,
                ..default()
            },
GlobalZIndex(LOADING_Z),
            LoadingRoot { spawned_at },
            LivedUiPlate,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                node: Node {
                    width: Val::Px(520.0),
                    max_width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(22.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: PANEL.into(),
                border_color: FRAME.into(),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            })
            .with_children(|panel| {
                panel
                    .spawn(NodeBundle {
                        node: Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(14.0),
                            border: UiRect::all(Val::Px(1.0)),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Stretch,
                            ..default()
                        },
                        background_color: TRACK.into(),
                        border_color: FRAME.into(),
                        ..default()
                    })
                    .with_children(|track| {
                        track.spawn((
                            NodeBundle {
                                node: Node {
                                    width: Val::Percent((fraction * 100.0).clamp(0.0, 100.0)),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: FILL.into(),
                                ..default()
                            },
                            LoadingBarFill,
                        ));
                    });
                panel.spawn((
                    (
Text::new(percent),
TextFont { font_size: 18.0 / 1.2, ..default() },
TextColor(TEXT_CREAM),
TextLayout::new_with_justify(JustifyText::Center),
Node {
                            width: Val::Percent(100.0),
                            ..default()
                        },
),
                    LoadingPercent,
                ));
                panel.spawn((
                    (
Text::new(flavour),
TextFont { font_size: 16.0 / 1.2, ..default() },
TextColor(TEXT_ROSE),
TextLayout::new_with_justify(JustifyText::Center),
Node {
                            width: Val::Percent(100.0),
                            ..default()
                        },
),
                    LoadingFlavour,
                ));
            });
        });
    info!(target: "powrush::loading", "boot loading panel up");
}

fn refresh_loading_overlay(
    mut commands: Commands,
    time: Res<Time<Real>>,
    server: Option<Res<AssetServer>>,
    fonts: Option<Res<Assets<Font>>>,
    texts: Query<&TextFont, (Without<LoadingPercent>, Without<LoadingFlavour>)>,
    images: Query<&ImageNode>,
    audio: Query<&AudioPlayer>,
    roots: Query<(Entity, &LoadingRoot)>,
    mut fills: Query<&mut Node, With<LoadingBarFill>>,
    mut percents: Query<&mut Text, (With<LoadingPercent>, Without<LoadingFlavour>)>,
    mut flavours: Query<&mut Text, (With<LoadingFlavour>, Without<LoadingPercent>)>,
) {
    let mut ids = Vec::new();
    push_observed(
        &mut ids,
        fonts.as_deref(),
        texts.iter().map(|font| font.font.id()),
        images.iter().map(|node| node.image.id()),
        audio.iter().map(|player| player.0.id()),
    );
    let snap = boot_snapshot(server.as_deref(), &ids);
    let fraction = progress_fraction(snap.settled, snap.tracked);
    let now = time.elapsed_secs();

    for mut style in &mut fills {
        style.width = Val::Percent((fraction * 100.0).clamp(0.0, 100.0));
    }

    for (entity, root) in &roots {
        let elapsed = (now - root.spawned_at).max(0.0);
        let percent = progress_label(fraction, snap.loaded_bytes, snap.total_bytes);
        let flavour = line_at(elapsed);
        for mut text in &mut percents {
            **text = percent.clone();
        }
        for mut text in &mut flavours {
            **text = flavour.to_string();
        }
        if overlay_ready(fraction, elapsed) {
            commands.entity(entity).despawn_recursive();
            info!(
                target: "powrush::loading",
                fraction,
                elapsed,
                "boot loading panel down"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_index_maps_time_and_wraps() {
        assert_eq!(line_index(0.0, 20), 0);
        assert_eq!(line_index(-2.0, 20), 0);
        assert_eq!(line_index(3.49, 20), 0);
        assert_eq!(line_index(3.5, 20), 1);
        assert_eq!(line_index(7.0, 20), 2);
        assert_eq!(line_index(7.0, 3), 2);
        assert_eq!(line_index(10.5, 3), 0);
        assert_eq!(line_index(3.5 * 20.0, 20), 0);
        assert_eq!(line_index(3.5 * 21.0, 20), 1);
        assert_eq!(line_index(0.0, 0), 0);
    }

    #[test]
    fn progress_fraction_full_when_no_handles_and_partial_when_counted() {
        assert_eq!(progress_fraction(0, 0), 1.0);
        assert_eq!(progress_fraction(0, 4), 0.0);
        assert_eq!(progress_fraction(1, 4), 0.25);
        assert_eq!(progress_fraction(2, 4), 0.5);
        assert_eq!(progress_fraction(4, 4), 1.0);
        assert_eq!(progress_fraction(5, 4), 1.0);
    }

    #[test]
    fn should_spawn_follows_q2_frame_switch() {
        assert!(should_spawn(false));
        assert!(!should_spawn(true));
    }

    #[test]
    fn loading_lines_nonempty_at_most_20_each_at_most_60_chars() {
        assert!(!LOADING_LINES.is_empty());
        assert!(LOADING_LINES.len() <= 20);
        assert_eq!(LOADING_LINES.len(), 20);
        for line in LOADING_LINES {
            assert!(!line.is_empty(), "blank line");
            assert!(
                line.chars().count() <= 60,
                "{line} has {} chars",
                line.chars().count()
            );
        }
        // The seed ellipsis is one char, three bytes.
        assert!(LOADING_LINES[0].chars().count() < LOADING_LINES[0].len());
        assert_eq!(LOADING_LINES[0], "Charging the obelisks…");
        assert_eq!(
            LOADING_LINES[7],
            "Walk (default WASD), jump (Space), sprint (Shift)."
        );
        assert_eq!(LOADING_LINES[8], "Walk to a glow and tend it (default E).");
        assert_eq!(LOADING_LINES[9], "Open your satchel (default I).");
        assert_eq!(
            LOADING_LINES[10],
            "After a tend, allocate (default R): 1 flow, 2 reserve."
        );
        assert_eq!(
            LOADING_LINES[11],
            "After you allocate, press Tab to take the ridge."
        );
        assert_eq!(
            LOADING_LINES[12],
            "Past the ridge, Q plants your first House stake."
        );
        assert_eq!(
            LOADING_LINES[13],
            "Past the ridge, press L to open the Ledger."
        );
        assert_eq!(LOADING_LINES[14], "Hide the guidance strip (default H).");
        assert_eq!(
            LOADING_LINES[19],
            "Draek plate is hard chitin, lit violet at the seams."
        );
    }

    fn line_has_banned(line: &str) -> bool {
        const BANNED: &[&str] = &["clone", "crypto", "nft", "female", "draexx", "drazhen"];
        let lower = line.to_lowercase();
        BANNED.iter().any(|word| lower.contains(word))
    }

    #[test]
    fn loading_lines_omit_banned_words() {
        for line in LOADING_LINES {
            assert!(!line_has_banned(line), "{line}");
        }
        assert!(line_has_banned("A Female clone"));
        assert!(line_has_banned("NFT"));
        assert!(line_has_banned("Crypto"));
        assert!(line_has_banned("Draexx"));
        assert!(line_has_banned("drazhen"));
    }

    #[test]
    fn progress_label_shows_megabytes_only_when_byte_counts_exist() {
        assert_eq!(progress_label(0.42, None, None), "42%");
        assert_eq!(progress_label(0.42, Some(1_048_576), None), "42%");
        assert_eq!(progress_label(0.42, None, Some(1_048_576)), "42%");
        let both = progress_label(0.42, Some(1_048_576), Some(2_097_152));
        assert!(both.starts_with("42%"), "{both}");
        assert!(both.contains("1.0 / 2.0 MB"), "{both}");
    }

    #[test]
    fn overlay_waits_for_full_progress_and_minimum_time() {
        assert!(!overlay_ready(1.0, 1.49));
        assert!(overlay_ready(1.0, 1.5));
        assert!(!overlay_ready(0.5, 10.0));
        assert!(!overlay_ready(0.0, 0.0));
    }

    #[test]
    fn loading_z_clears_title_persona_layer() {
        assert!(LOADING_Z > 142);
        assert_eq!(LOADING_Z, 200);
    }

    #[test]
    fn plugin_spawn_matches_q2_frame_switch() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(LoadingScreenPlugin);
        app.update();
        let mut query = app.world_mut().query::<&LoadingRoot>();
        let spawned = query.iter(app.world()).next().is_some();
        assert_eq!(spawned, should_spawn(q2_frame_set()));
    }
}
