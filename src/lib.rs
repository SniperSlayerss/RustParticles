pub mod components {
    pub mod particle;
}

pub mod initialisation {
    pub mod simulation_parameters;
    pub mod setup;

    pub use simulation_parameters::SimulationParameters;
    pub use setup::setup_system;
}

pub mod systems {
    pub mod particle_spawner_system;
    pub mod update_system;
    pub mod collision_system;

    pub use particle_spawner_system::particle_spawner_system;
    pub use particle_spawner_system::particle_spawner_system_tests;
    pub use update_system::update_system;
    pub use collision_system::check_collisions;
}
