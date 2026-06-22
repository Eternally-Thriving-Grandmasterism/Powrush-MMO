
fn spawn_harmony_knot_particle(
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
        Name::new("HarmonyKnotParticle"),
    ));
}
