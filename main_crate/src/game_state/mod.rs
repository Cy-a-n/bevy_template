use bevy::{
    app::{Plugin, Startup},
    ecs::schedule::States,
};

use self::systems::setup;

mod components;
mod in_game;
mod main_menu;
mod resources;
mod systems;

pub(super) struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup).init_state::<GameStates>();
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameStates {
    #[default]
    MainMenu,
    InGame,
}
