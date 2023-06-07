use bevy::ecs::schedule::States;

#[allow(dead_code)]
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
}
