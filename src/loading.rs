use bevy::prelude::*;

#[derive(Resource)]
pub struct LoadingProgress {
    pub loaded: usize,
    pub total: usize,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingProgress { loaded: 0, total: 1 })
           .add_systems(Startup, setup_loading_screen)
           .add_systems(Update, update_loading_progress.run_if(in_state(GameState::Loading)));
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

fn setup_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let assets = vec![
        asset_server.load("sounds/mercy_chime.ogg"),
        asset_server.load("textures/gold_particle.png"),
        // Add more critical assets here
    ];

    commands.insert_resource(LoadingProgress {
        loaded: 0,
        total: assets.len(),
    });

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.05, 0.05, 0.1)),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Loading Mercy Universe...",
            TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                ..default()
            },
        ));

        parent.spawn((
            TextBundle::from_section(
                "0%",
                TextStyle {
                    font_size: 40.0,
                    color: Color::CYAN,
                    ..default()
                },
            ),
            LoadingText,
        ));

        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Px(20.0),
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            ..default()
        }).with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GOLD),
                    ..default()
                },
                LoadingBar,
            ));
        });
    });
}

#[derive(Component)]
struct LoadingText;

#[derive(Component)]
struct LoadingBar;

fn update_loading_progress(
    asset_server: Res<AssetServer>,
    progress: Res<LoadingProgress>,
    mut text_query: Query<&mut Text, With<LoadingText>>,
    mut bar_query: Query<&mut Style, With<LoadingBar>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let percent = if progress.total > 0 {
        (progress.loaded as f32 / progress.total as f32) * 100.0
    } else {
        100.0
    };

    for mut text in &mut text_query {
        text.sections[0].value = format!("{:.0}%", percent);
    }

    for mut style in &mut bar_query {
        style.width = Val::Percent(percent);
    }

    if percent >= 100.0 {
        next_state.set(GameState::InGame);
        info!("Loading complete — mercy universe ready");
    }
}
