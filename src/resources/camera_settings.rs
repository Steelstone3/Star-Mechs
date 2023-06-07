use bevy::ecs::system::Resource;

#[derive(Resource, PartialEq, Debug)]
pub struct CameraSettings {
    pub zoom_speed: f32,
    pub minimum_zoom: f32,
    pub maximum_zoom: f32,
    pub current_zoom: f32,
    pub zoom_in: f32,
    pub zoom_out: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 2.0,
            current_zoom: 1.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        }
    }
}

#[cfg(test)]
mod random_generator_should {
    use super::*;

    #[test]
    fn have_default() {
        // Given
        let expected_camera_settings = CameraSettings {
            zoom_speed: 1.5,
            minimum_zoom: 0.5,
            maximum_zoom: 2.0,
            current_zoom: 1.0,
            zoom_in: 1.1,
            zoom_out: 0.9,
        };

        // When
        let camera_settings = CameraSettings::default();

        // Then
        assert_eq!(expected_camera_settings, camera_settings);
    }
}
