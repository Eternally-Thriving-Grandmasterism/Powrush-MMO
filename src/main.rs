        // === Hanabi + Lissajous Knot Visual Effects ===
        .add_plugins(HanabiPlugin)
        .init_resource::<PolicyParticleEffects>()
        .init_resource::<LissajousKnotEffects>()
        .init_resource::<CurrentLissajousKnotPreset>()
        .add_event::<SwitchLissajousKnotPreset>()

        // Startup
        .add_systems(Startup, (
            setup_policy_particle_effects,
            register_lissajous_knot_effects,
            lissajous_knot_ui_panel,
            spawn_harmony_knot_particle,
        ))

        // Update (reactive systems)
        .add_systems(Update, (
            debug_lissajous_knot_input,
            handle_switch_lissajous_knot_preset,
            highlight_active_preset_button.after(handle_switch_lissajous_knot_preset),
            update_lissajous_knot_ui.after(handle_switch_lissajous_knot_preset),
            update_active_lissajous_knot.after(handle_switch_lissajous_knot_preset),
        ))
