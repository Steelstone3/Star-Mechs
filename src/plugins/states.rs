use bevy::app::Plugin;

use crate::components::game_states::GameState;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_state(GameState::MainMenu)
            .insert_state(GameState::Playing)
            .insert_state(GameState::Paused);
    }
}
