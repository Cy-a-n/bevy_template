use bevy::app::{Plugin, Startup};

use self::systems::setup;

mod components;
mod resources;
mod systems;

pub(super) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            // .init_state::<MainMenuStates>()
            ;
    }
}

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
// enum MainMenuStates {
//     #[default]

// }
