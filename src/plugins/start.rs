use bevy::prelude::{App, Plugin, Startup};

use crate::systems::camera::spawn_camera::spawn_camera;

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
