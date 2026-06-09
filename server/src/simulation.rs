// server/src/simulation.rs
// Powrush-MMO v17.75 — SimulationApp (Bevy App Wrapper)
// Clean integration layer for combat + replication + rathor systems

use bevy::prelude::*;
use crate::combat::CombatPlugin;
use crate::replication::ReplicationPlugin;
use crate::rathor_integration::RathorIntegrationPlugin;

/// Dedicated Bevy App for all simulation systems.
/// This runs independently of the outer networking/persistence loop.
pub struct SimulationApp {
    pub app: App,
}

impl SimulationApp {
    pub fn new() -> Self {
        let mut app = App::new();

        // Register all simulation plugins
        app.add_plugins((
            CombatPlugin,
            ReplicationPlugin,
            RathorIntegrationPlugin,
        ));

        // Future plugins can be added here:
        // app.add_plugins(DynamicEventsPlugin);
        // app.add_plugins(HarvestingPlugin);

        Self { app }
    }

    /// Runs one full simulation tick (Bevy schedule)
    pub fn tick(&mut self) {
        self.app.update();
    }

    /// Access to the underlying World if needed for advanced integration
    pub fn world(&self) -> &World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }
}
