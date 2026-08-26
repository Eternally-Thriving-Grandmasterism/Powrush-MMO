/*!
 * World Answer — v22.2.0
 *
 * Allocate spends a satchel stack. Sky, fog, ambient, node pulse answer.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::pbr::FogSettings;
use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::mercy_harvest_nodes::MercyHarvestNode;
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerKind {
    Idle,
    Take,
    Tend,
    Flow,
    Reserve,
}

#[derive(Resource, Debug)]
pub struct WorldAnswer {
    pub kind: AnswerKind,
    pub until: f64,
    pub warmth: f32,
    pub last_line: String,
}

impl Default for WorldAnswer {
    fn default() -> Self {
        Self {
            kind: AnswerKind::Idle,
            until: 0.0,
            warmth: 0.0,
            last_line: String::new(),
        }
    }
}

impl WorldAnswer {
    pub fn fire(&mut self, kind: AnswerKind, now: f64, line: impl Into<String>) {
        self.kind = kind;
        self.until = now
            + match kind {
                AnswerKind::Flow | AnswerKind::Reserve => 3.4,
                AnswerKind::Tend => 2.2,
                AnswerKind::Take => 1.4,
                AnswerKind::Idle => 0.0,
            };
        self.warmth = match kind {
            AnswerKind::Flow => 0.85,
            AnswerKind::Reserve => 0.45,
            AnswerKind::Tend => 0.55,
            AnswerKind::Take => 0.22,
            AnswerKind::Idle => 0.0,
        };
        self.last_line = line.into();
    }

    pub fn live(&self, now: f64) -> bool {
        now < self.until && self.kind != AnswerKind::Idle
    }
}

pub fn fire_world_answer(answer: &mut WorldAnswer, kind: AnswerKind, now: f64, line: impl Into<String>) {
    answer.fire(kind, now, line);
}

pub struct WorldAnswerPlugin;

impl Plugin for WorldAnswerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldAnswer>()
            .add_systems(Update, (notice_allocate, paint_world_answer));
    }
}

fn notice_allocate(
    allocate: Res<RbeAllocateChoice>,
    mut pool: ResMut<SoftRbePool>,
    mut answer: ResMut<WorldAnswer>,
    time: Res<Time>,
    mut last: Local<u32>,
) {
    if allocate.choices_made == *last {
        return;
    }
    *last = allocate.choices_made;
    let now = time.elapsed_seconds_f64();
    let Some(path) = allocate.last_choice else {
        return;
    };
    let spent = pool.spend_allocate(path, 1.0);
    match path {
        AllocatePath::FlowOutward => fire_world_answer(
            &mut answer,
            AnswerKind::Flow,
            now,
            format!("Flow −{spent:.1} vitality — the climate brightens"),
        ),
        AllocatePath::StewardReserve => fire_world_answer(
            &mut answer,
            AnswerKind::Reserve,
            now,
            format!("Reserve −{spent:.1} harmony — the climate steadies"),
        ),
    }
    info!(target: "powrush::answer", ?path, spent, "allocate spent into the climate");
}

fn paint_world_answer(
    answer: Res<WorldAnswer>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut fogs: Query<&mut FogSettings>,
    mut nodes: Query<&mut MercyHarvestNode>,
    mut last_kind: Local<AnswerKind>,
) {
    let now = time.elapsed_seconds_f64();
    let live = answer.live(now);
    let fade = if live {
        ((answer.until - now) / 3.4).clamp(0.0, 1.0) as f32
    } else {
        0.0
    };
    let extra = answer.warmth * fade;
    ambient.brightness = 280.0 + extra * 220.0;

    for mut fog in &mut fogs {
        if live && matches!(answer.kind, AnswerKind::Flow) {
            fog.falloff = bevy::pbr::FogFalloff::Linear {
                start: 14.0,
                end: 52.0,
            };
        } else if live && matches!(answer.kind, AnswerKind::Reserve) {
            fog.falloff = bevy::pbr::FogFalloff::Linear {
                start: 8.0,
                end: 34.0,
            };
        }
    }

    if live && *last_kind != answer.kind {
        for mut node in &mut nodes {
            node.pulse = (node.pulse + 0.35).min(1.0);
            if matches!(answer.kind, AnswerKind::Tend | AnswerKind::Flow) {
                node.vitality = (node.vitality + 0.04).min(1.0);
            }
        }
    }
    *last_kind = if live {
        answer.kind
    } else {
        AnswerKind::Idle
    };
}
