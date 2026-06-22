/*!
 * InterestZone with zone boundary visualization support.
 */

#[derive(Component, Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct InterestZone {
    pub center: Vec3,
    pub base_radius: f32,
    pub valence_multiplier: f32,
    pub council_boost: f32,
    pub mercy_resonance: f32,
    pub target_center: Vec3,
    pub target_base_radius: f32,
    pub version: u64,

    // Visualization for zone boundaries (driven by policies and council activity)
    pub visual_highlight: f32,        // 0.0–1.0 — how strongly to render the boundary glow
    pub visual_tint: [f32; 3],        // RGB hint for boundary color
}

impl InterestZone {
    pub fn new(center: Vec3, base_radius: f32) -> Self {
        Self {
            center,
            base_radius,
            valence_multiplier: 1.0,
            council_boost: 0.0,
            mercy_resonance: 0.0,
            target_center: center,
            target_base_radius: base_radius,
            version: 0,
            visual_highlight: 0.0,
            visual_tint: [0.3, 0.7, 1.0], // default cyan-ish
        }
    }

    // ... existing methods ...

    pub fn apply_valence_and_mercy(&mut self, valence: f32, mercy: f32) {
        self.valence_multiplier = valence.clamp(0.5, 3.0);
        self.mercy_resonance = mercy.clamp(0.0, 2.0);
        self.version += 1;

        // Boost visualization when council/mercy activity is high
        self.visual_highlight = (self.visual_highlight + mercy * 0.3).min(1.0);
    }
}
