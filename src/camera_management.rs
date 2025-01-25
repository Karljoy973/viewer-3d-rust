use std::{f32::consts::FRAC_PI_2, ops::Range};

use bevy::{input::{mouse::AccumulatedMouseMotion, ButtonInput}, math::Vec2, prelude::{Camera, KeyCode, Mesh3d, MouseButton, Res, Resource, Single, Transform, With}};

#[derive(Debug, Resource)]
pub struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
    pub x_translation_speed: f32,
    pub y_translation_speed: f32,
    pub z_translation_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        // Limiting pitch stops some unexpected rotation past 90° up or down.
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            // These values are completely arbitrary, chosen because they seem to produce
            // "sensible" results for this example. Adjust as required.
            orbit_distance: 1.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
            x_translation_speed: 0.01, 
            y_translation_speed: 0.01, 
            z_translation_speed: 0.01,
        }
    }
}


pub fn camera_keyboard_translation_system(
    key: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
) {
    // define my translations with keyboard and arrow keys
    if key.pressed(KeyCode::ArrowUp) && !key.pressed(KeyCode::ShiftLeft) {
        camera.translation.z -= camera_settings.z_translation_speed;    
        println!("Arrow up pressed");
        println!("new camera position : {}", camera.translation);
    }
    else if key.pressed(KeyCode::ArrowDown) && !key.pressed(KeyCode::ShiftLeft) {
        camera.translation.z += camera_settings.z_translation_speed;
        println!("Arrow down pressed");
    }
    else if key.pressed(KeyCode::ArrowLeft) {
        camera.translation.x += camera_settings.x_translation_speed;
        println!("Arrow left pressed");
    }
    else if key.pressed(KeyCode::ArrowRight) {
        camera.translation.x -= camera_settings.x_translation_speed;
        println!("Arrow right pressed");
    }
    else if key.pressed(KeyCode::ArrowUp) && key.pressed(KeyCode::ShiftLeft) {
        camera.translation.y -= camera_settings.y_translation_speed;
        println!("Arrow left + Shift key pressed");
    }
    else if key.pressed(KeyCode::ArrowDown) && key.pressed(KeyCode::ShiftLeft){
        camera.translation.y += camera_settings.y_translation_speed;
        println!("Arrow right + Shift key pressed");
    }
}

pub fn camera_mouse_keyboard_rotation_system(

    key: Res<ButtonInput<KeyCode>>,
    mouse_mouvement: Res<AccumulatedMouseMotion>, 
    mouse_state: Res<ButtonInput<MouseButton>>,
    mut mesh: Single<&mut Transform, With<Mesh3d>>,
) {
    if key.just_pressed(KeyCode::KeyC) {
        println!("C key pressed");
        //select cube state 
        if /*key.pressed(KeyCode::KeyX) &&*/ mouse_state.pressed(MouseButton::Left) {
            mesh.translation.x = mouse_mouvement.delta.project_onto(Vec2::new(mesh.translation.x,mesh.translation.y)).x;
        }
        else if key.pressed(KeyCode::KeyY) {
            
        }
        else if key.pressed(KeyCode::KeyZ) { // I could use a switch statement (or the ruste equivalent)
            
        }
    }

}

