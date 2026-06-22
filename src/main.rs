
/// Spawns the initial visual particle effect for policy feedback.
/// Currently spawns the Harmony (Lissajous knot) effect as the default.
/// The HarmonyKnotMarker allows other systems to find and update this entity.
fn spawn_policy_visual_effects(
    mut commands: Commands,
    knot_effects: Res<LissajousKnotEffects>,
) {
    // Spawn the main policy visualization particle effect
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(knot_effects.complex.clone()),
            transform: Transform::from_translation(Vec3::new(0.0, 8.0, 0.0)),
            ..default()
        },
        HarmonyKnotMarker,           // Marker so update systems can find this entity
        Name::new("PolicyVisualEffects"),
    ));
}
