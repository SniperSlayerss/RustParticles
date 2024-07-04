use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;

use RustParticles::initialisation::{ setup_system , SimulationParameters} ;
use RustParticles::systems::{ particle_spawner_system, update_system };

fn main() {
    App::new()
        .insert_resource(SimulationParameters::new())
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, update_system)
        .add_systems(
            FixedUpdate,
            particle_spawner_system,
        )
        .run();
}
