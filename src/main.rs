
/// Spawns the visual particle system that represents active Council policies.
///
/// Currently spawns the advanced Harmony effect (5:3:4 3D Lissajous knot with breathing).
/// Other policy effects (Abundance, Sustainability, Prosperity) can be added here later.
///
/// The `HarmonyKnotMarker` component allows the reactive systems to find and
/// update this particle effect when the player switches presets at runtime.
fn spawn_policy_visual_effects(
    mut commands: Commands,
    knot_effects: Res<LissajousKnotEffects>,
) {
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(knot_effects.complex.clone()),
            transform: Transform::from_translation(Vec3::new(0.0, 8.0, 0.0)),
            ..default()
        },
        HarmonyKnotMarker,
        Name::new("PolicyVisualEffects"),
    ));
}
