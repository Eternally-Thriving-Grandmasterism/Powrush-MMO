impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        // === Persistence Layer (NEW) ===
        let persistence_manager = PersistenceManager::new(
            std::path::PathBuf::from("saves/players")
        );
        app.insert_resource(persistence_manager);

        // === Existing plugins ===
        app
            .add_plugins(RbeServerPlugin)
            .add_plugins(AscensionMercyAscentPlugin)
            // ... all other existing .add_plugins(...) calls ...
            .add_plugins(PersistencePolishPlugin)
            .add_plugins(FactionPersistencePlugin)
            // ... rest of the build ...
    }
}