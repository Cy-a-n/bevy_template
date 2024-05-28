use bevy::{
    app::{Plugin, Startup},
    ecs::schedule::States,
};

use self::systems::setup;

mod components;
mod resources;
mod systems;

pub(super) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            // .init_state::<InGameStates>()
            ;
    }
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
// enum InGameStates {
//     #[default]

// }
