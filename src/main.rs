use std::thread::current;

use bevy::{input::{keyboard::{Key, KeyboardInput}, mouse::{self, AccumulatedMouseMotion, AccumulatedMouseScroll}}, prelude::*};
use bevy::input::keyboard;

 use std::{f32::consts::FRAC_PI_2, ops::Range};
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

fn main() {
   App::new()
        .init_resource::<CameraSettings>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_keyboard_translation_system)
        .run();
    // .add_systems(Update, camera_rotation_system)
    // .add_systems(Update, camera_mouse_translation_system)
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(100, 100, 100))),
        Transform::from_xyz(0.0, 0., 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


// fn camera_rotation_system(
//     mouse_buttons: Res<ButtonInput<MouseButton>>,
//     mouse_motion: Res<AccumulatedMouseMotion>,
//     key_pressed: Res<ButtonInput<KeyCode>>,
//     mut camera: Single<&mut Transform, With<Camera>>,
//     camera_settings: Res<CameraSettings>,
//     time: Res<Time>,
// ) {
//     let delta = mouse_motion.delta;
//     let mut delta_roll = 0.0;

//     let delta_pitch = delta.y * camera_settings.pitch_speed;
//     let delta_yaw = delta.x * camera_settings.yaw_speed;

//     // Conversely, we DO need to factor in delta time for mouse button inputs.
//     delta_roll *= camera_settings.roll_speed * time.delta_secs();

//     // Obtain the existing pitch, yaw, and roll values from the transform.
//     let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

//     // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
//     let pitch = (pitch + delta_pitch).clamp(
//         camera_settings.pitch_range.start,
//         camera_settings.pitch_range.end,
//     );
//     let roll = roll + delta_roll;
//     let yaw = yaw + delta_yaw;
//     camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

//     let target = Vec3::ZERO;
//     // camera.rotation = target - camera.forward() * camera_settings.orbit_distance;

// }

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

pub enum viewerStates {
    Viewer, //the input system affects the camera 
    Compose, //the input system affects the selected mesh 
    
}