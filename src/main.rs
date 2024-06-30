use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod components;
mod setup;
mod simulation_parameters;
mod systems;

use setup::setup_system;
use simulation_parameters::SimulationParameters;
use systems::update_system;

fn main() {
    App::new()
        .insert_resource(SimulationParameters::new())
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, update_system::update_system)
        //.add_systems(FixedUpdate, collision_system::collision_system)
        .run();
}
