use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod setup;
mod systems;
mod components;

use setup::setup_system;
use systems::{ update_system, collision_system };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_system)
        .add_systems(Update, update_system::update_system)
        .add_systems(Update, collision_system::collision_system)
        .run();
}
