use ra_thor_core::ValenceScore;

pub struct ValenceGate {
    threshold: f32,
}

impl ValenceGate {
    pub fn new(threshold: f32) -> Self {
        ValenceGate { threshold }
    }

    pub fn allow_action(&self, score: ValenceScore) -> bool {
        score >= self.threshold
    }

    pub fn apply_penalty(&self, score: ValenceScore) -> f32 {
        if score < self.threshold {
            (self.threshold - score) * 0.5 // example penalty scaling
        } else {
            0.0
        }
    }
}
