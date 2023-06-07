use crate::{
    components::game_states::GameState,
    systems::{
        animate_sprites::animate_sprites,
        camera::{
            camera_movement::camera_movement, camera_position_reset::camera_position_reset,
            camera_zoom_keyboard::camera_zoom_keyboard,
            camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
        },
    },
};
use bevy::{
    app::Update,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    prelude::{App, Plugin},
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_sprites.run_if(in_state(GameState::Playing)),
                camera_zoom_keyboard,
                camera_zoom_mouse_and_touchpad,
                camera_movement,
                camera_position_reset,
            ),
        );
    }
}
