use simulation::{
    // ... previous items ...
    despawn_expired_visual_effects,  // NEW for lifetime control
};

// In build_app(), inside the particle systems group:
.add_systems(Update, (
    // ... existing reactive systems ...
    despawn_expired_visual_effects,  // NEW
));