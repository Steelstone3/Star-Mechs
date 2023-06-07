use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, ButtonInput},
    time::Time,
};

use crate::queries::camera_queries::MutableCameraTransformQuery;

pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<MutableCameraTransformQuery>,
) {
    let Ok(mut camera) = cameras.get_single_mut() else {
        return;
    };

    // Camera Movement Speeds
    let camera_speed = 500.0 * time.delta_seconds();
    let diagonal_camera_speed = calculate_diagonal_camera_speed(camera_speed);
    let slow_camera_speed = 100.0 * time.delta_seconds();
    let slow_diagonal_camera_speed = calculate_diagonal_camera_speed(slow_camera_speed);
    let fast_camera_speed = 1000.0 * time.delta_seconds();
    let fast_diagonal_camera_speed = calculate_diagonal_camera_speed(fast_camera_speed);

    // Inputs
    let is_control_pressed =
        input.pressed(KeyCode::ControlRight) || input.pressed(KeyCode::ControlLeft);
    let is_shift_pressed = input.pressed(KeyCode::ShiftRight) || input.pressed(KeyCode::ShiftLeft);
    let is_camera_up_pressed = input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp);
    let is_camera_right_pressed =
        input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight);
    let is_camera_down_pressed = input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown);
    let is_camera_left_pressed = input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft);

    // Up and Right
    if is_camera_up_pressed && is_camera_right_pressed {
        if is_shift_pressed {
            camera.transform.translation.x += fast_diagonal_camera_speed;
            camera.transform.translation.y += fast_diagonal_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x += slow_camera_speed;
            camera.transform.translation.y += slow_diagonal_camera_speed;
        } else {
            camera.transform.translation.x += diagonal_camera_speed;
            camera.transform.translation.y += diagonal_camera_speed;
        }
    }
    // Down and Right
    else if is_camera_down_pressed && is_camera_right_pressed {
        if is_shift_pressed {
            camera.transform.translation.x += fast_diagonal_camera_speed;
            camera.transform.translation.y -= fast_diagonal_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x += slow_camera_speed;
            camera.transform.translation.y -= slow_diagonal_camera_speed;
        } else {
            camera.transform.translation.x += diagonal_camera_speed;
            camera.transform.translation.y -= diagonal_camera_speed;
        }
    }
    // Down and Left
    else if is_camera_down_pressed && is_camera_left_pressed {
        if is_shift_pressed {
            camera.transform.translation.x -= fast_diagonal_camera_speed;
            camera.transform.translation.y -= fast_diagonal_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x -= slow_camera_speed;
            camera.transform.translation.y -= slow_diagonal_camera_speed;
        } else {
            camera.transform.translation.x -= diagonal_camera_speed;
            camera.transform.translation.y -= diagonal_camera_speed;
        }
    }
    // Up and Left
    else if is_camera_up_pressed && is_camera_left_pressed {
        if is_shift_pressed {
            camera.transform.translation.x -= fast_diagonal_camera_speed;
            camera.transform.translation.y += fast_diagonal_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x -= slow_camera_speed;
            camera.transform.translation.y += slow_diagonal_camera_speed;
        } else {
            camera.transform.translation.x -= diagonal_camera_speed;
            camera.transform.translation.y += diagonal_camera_speed;
        }
    }
    // Up
    else if is_camera_up_pressed {
        if is_shift_pressed {
            camera.transform.translation.y += fast_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.y += slow_camera_speed;
        } else {
            camera.transform.translation.y += camera_speed;
        }
    }
    // Right
    else if is_camera_right_pressed {
        if is_shift_pressed {
            camera.transform.translation.x += fast_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x += slow_camera_speed;
        } else {
            camera.transform.translation.x += camera_speed;
        }
    }
    // Down
    else if is_camera_down_pressed {
        if is_shift_pressed {
            camera.transform.translation.y -= fast_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.y -= slow_camera_speed;
        } else {
            camera.transform.translation.y -= camera_speed;
        }
    }
    // Left
    else if is_camera_left_pressed {
        if is_shift_pressed {
            camera.transform.translation.x -= fast_camera_speed;
        } else if is_control_pressed {
            camera.transform.translation.x -= slow_camera_speed;
        } else {
            camera.transform.translation.x -= camera_speed;
        }
    }
}

fn calculate_diagonal_camera_speed(camera_speed: f32) -> f32 {
    let diagonal_speed_squared = (camera_speed * camera_speed) + (camera_speed * camera_speed);
    let diagonal_speed = (diagonal_speed_squared).sqrt();

    camera_speed * (camera_speed / diagonal_speed)
}

#[cfg(test)]
mod camera_movement_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 0.70710677)]
    #[case(100.0, 70.71068)]
    #[case(141.142, 99.80246)]
    fn be_able_to_calculate_diagonal_camera_speed(
        #[case] camera_speed: f32,
        #[case] expected_diagonal_speed: f32,
    ) {
        // Given
        let diagonal_camera_speed = calculate_diagonal_camera_speed(camera_speed);

        // Then
        assert_eq!(expected_diagonal_speed, diagonal_camera_speed);
    }
}
