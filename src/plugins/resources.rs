use bevy::prelude::{App, Plugin};

use crate::resources::camera_settings::CameraSettings;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default());
    }
}
