use bevy::{math::Vec2, transform::components::Transform};

use crate::{
    components::constants::TILE_SIZE,
    queries::{
        camera_queries::CameraTransformOrthographicProjectionQueryItem,
        window_queries::WindowQueryItem,
    },
};

#[allow(dead_code)]
pub fn get_cursor_location(
    transform: &mut Transform,
    cursor_position: bevy::prelude::Vec2,
    window_query: WindowQueryItem<'_>,
    camera_query: CameraTransformOrthographicProjectionQueryItem<'_>,
) {
    transform.translation.x = ((cursor_position.x - window_query.window.resolution.width() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.x;
    transform.translation.y = -((cursor_position.y
        - window_query.window.resolution.height() / 2.0)
        * camera_query.projection.scale)
        + camera_query.transform.translation.y;
}

#[allow(dead_code)]
pub fn get_tile_location(
    transform: &mut Transform,
    cursor_position: bevy::prelude::Vec2,
    window_query: WindowQueryItem<'_>,
    camera_query: CameraTransformOrthographicProjectionQueryItem<'_>,
) {
    get_cursor_location(transform, cursor_position, window_query, camera_query);

    let tile_position =
        get_nearest_tile(Vec2::new(transform.translation.x, transform.translation.y));

    transform.translation.x = tile_position.x;
    transform.translation.y = tile_position.y;
}

fn get_nearest_tile(cursor_position: Vec2) -> Vec2 {
    Vec2::new(
        (cursor_position.x / TILE_SIZE).round() * TILE_SIZE,
        (cursor_position.y / TILE_SIZE).round() * TILE_SIZE,
    )
}

#[cfg(test)]
mod get_location_should {
    use super::*;
    use bevy::math::Vec2;
    use rstest::rstest;

    #[rstest]
    #[case(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(1.0, 1.0), Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(16.0, 16.0), Vec2::new(32.0, 32.0))]
    #[case(Vec2::new(17.0, 17.0), Vec2::new(32.0, 32.0))]
    #[case(Vec2::new(33.0, 33.0), Vec2::new(32.0, 32.0))]
    #[case(Vec2::new(47.0, 47.0), Vec2::new(32.0, 32.0))]
    #[case(Vec2::new(48.0, 48.0), Vec2::new(64.0, 64.0))]
    #[case(Vec2::new(63.0, 63.0), Vec2::new(64.0, 64.0))]
    #[case(Vec2::new(-1.0, -1.0), Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(-16.0, -16.0), Vec2::new(-32.0, -32.0))]
    #[case(Vec2::new(-17.0, -17.0), Vec2::new(-32.0, -32.0))]
    #[case(Vec2::new(53.0, -12.0), Vec2::new(64.0, 0.0))]
    #[case(Vec2::new(1876.111, -245.666), Vec2::new(1888.0, -256.0))]
    #[case(Vec2::new(-458.453, 768.997), Vec2::new(-448.0, 768.0))]
    fn get_the_nearest_tile_position(
        #[case] cursor_position: Vec2,
        #[case] expected_nearest_tile_position: Vec2,
    ) {
        // When
        let nearest_tile_position = get_nearest_tile(cursor_position);

        // Then
        assert_eq!(expected_nearest_tile_position, nearest_tile_position);
    }
}
