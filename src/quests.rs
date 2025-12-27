use bevy::prelude::*;
use bevy_replicon::prelude::*;
use rand::Rng;

/// Main quest component — attached to player
#[derive(Component, Replicated)]
pub struct Quest {
    pub name: String,
    pub kind: QuestKind,
    pub description: String,
    pub progress: f32,
    pub goal: f32,
    pub completed: bool,
    pub reward_mercy: f32,
    pub difficulty: QuestDifficulty,
}

/// Types of quests — mercy-themed
#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestKind {
    Explore,
    Share,
    MercyWave,
    LatticeGrow,
    Forgive,
    BuildHome,
}

/// Adaptive difficulty based on player trust
#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum QuestDifficulty {
    Gentle,
    Balanced,
    Challenging,
    Eternal,
}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_initial_quests)
            .add_systems(Update, (
                procedural_quest_generation,
                quest_progress_system,
                quest_completion_system,
                quest_reward_system,
            ));
    }
}

/// Initial quests on player spawn
fn spawn_initial_quests(
    mut commands: Commands,
    players: Query<Entity, Added<Player>>,
    trust_query: Query<&TrustCredits>,
) {
    let mut rng = rand::thread_rng();
    for player in &players {
        let trust = trust_query.get(player).map(|t| t.0).unwrap_or(0.0);
        let difficulty = if trust > 1000.0 {
            QuestDifficulty::Eternal
        } else if trust > 500.0 {
            QuestDifficulty::Challenging
        } else if trust > 100.0 {
            QuestDifficulty::Balanced
        } else {
            QuestDifficulty::Gentle
        };

        let kind = match rng.gen_range(0..6) {
            0 => QuestKind::Explore,
            1 => QuestKind::Share,
            2 => QuestKind::MercyWave,
            3 => QuestKind::LatticeGrow,
            4 => QuestKind::Forgive,
            _ => QuestKind::BuildHome,
        };

        let (name, desc, goal, reward) = generate_quest_details(kind, difficulty);

        commands.entity(player).insert(Quest {
            name,
            kind,
            description: desc,
            progress: 0.0,
            goal,
            completed: false,
            reward_mercy: reward,
            difficulty,
        });

        info!("Initial quest spawned — {} ({:?})", name, difficulty);
    }
}

/// Dynamic new quests over time
fn procedural_quest_generation(
    mut commands: Commands,
    players: Query<(Entity, &TrustCredits), Without<Quest>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.002 * time.delta_seconds() as f64) {
        for (player, trust) in &players {
            let difficulty = match trust.0 {
                t if t > 2000.0 => QuestDifficulty::Eternal,
                t if t > 1000.0 => QuestDifficulty::Challenging,
                t if t > 300.0 => QuestDifficulty::Balanced,
                _ => QuestDifficulty::Gentle,
            };

            let kind = match rng.gen_range(0..6) {
                0 => QuestKind::Explore,
                1 => QuestKind::Share,
                2 => QuestKind::MercyWave,
                3 => QuestKind::LatticeGrow,
                4 => QuestKind::Forgive,
                _ => QuestKind::BuildHome,
            };

            let (name, desc, goal, reward) = generate_quest_details(kind, difficulty);

            commands.entity(player).insert(Quest {
                name,
                kind,
                description: desc,
                progress: 0.0,
                goal,
                completed: false,
                reward_mercy: reward,
                difficulty,
            });

            info!("Procedural quest generated — {}", name);
        }
    }
}

/// Generate details based on kind + difficulty
fn generate_quest_details(
    kind: QuestKind,
    difficulty: QuestDifficulty,
) -> (String, String, f32, f32) {
    let mut rng = rand::thread_rng();
    let base_goal = match difficulty {
        QuestDifficulty::Gentle => 10.0,
        QuestDifficulty::Balanced => 25.0,
        QuestDifficulty::Challenging => 50.0,
        QuestDifficulty::Eternal => 100.0,
    };
    let goal = base_goal * rng.gen_range(0.8..1.4);

    let reward = goal * 3.0;

    match kind {
        QuestKind::Explore => (
            "Explore the Lattice".to_string(),
            "Wander and discover new nodes".to_string(),
            goal,
            reward,
        ),
        QuestKind::Share => (
            "Share Mercy".to_string(),
            "Trade or give items to others".to_string(),
            goal,
            reward,
        ),
        QuestKind::MercyWave => (
            "Trigger Mercy Wave".to_string(),
            "Participate in forgiveness duels".to_string(),
            goal,
            reward,
        ),
        QuestKind::LatticeGrow => (
            "Grow the Lattice".to_string(),
            "Help expand nodes and connections".to_string(),
            goal,
            reward,
        ),
        QuestKind::Forgive => (
            "Practice Forgiveness".to_string(),
            "Lose a duel gracefully".to_string(),
            goal,
            reward * 1.5,
        ),
        QuestKind::BuildHome => (
            "Build Your Sanctuary".to_string(),
            "Construct a personal home".to_string(),
            goal,
            reward * 2.0,
        ),
    }
}

/// Progress based on actions + time
fn quest_progress_system(
    mut query: Query<&mut Quest>,
    time: Res<Time>,
    lattice: Res<LatticeStats>,
) {
    for mut quest in &mut query {
        if quest.completed { continue; }

        let progress_per_sec = match quest.difficulty {
            QuestDifficulty::Gentle => 0.5,
            QuestDifficulty::Balanced => 0.3,
            QuestDifficulty::Challenging => 0.2,
            QuestDifficulty::Eternal => 0.1,
        };

        quest.progress += progress_per_sec * time.delta_seconds();

        // Bonus from lattice growth
        if matches!(quest.kind, QuestKind::LatticeGrow) {
            quest.progress += lattice.nodes as f32 * 0.01;
        }

        if quest.progress >= quest.goal {
            quest.completed = true;
            info!("Quest completed — {}", quest.name);
        }
    }
}

/// Reward on completion
fn quest_reward_system(
    query: Query<&Quest>,
    mut trust: Query<&mut TrustCredits>,
) {
    for quest in &query {
        if quest.completed {
            if let Ok(mut player_trust) = trust.get_mut(quest.entity()) {
                player_trust.0 += quest.reward_mercy;
                info!("Quest reward — +{} mercy", quest.reward_mercy);
            }
        }
    }
}
