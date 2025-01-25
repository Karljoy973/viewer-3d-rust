
use bevy::{input::{keyboard::{Key, KeyboardInput}, mouse::AccumulatedMouseMotion}, prelude::*, utils::hashbrown::Equivalent};
use std::{f32::consts::FRAC_PI_2, ops::Range};

pub mod camera_management;
pub mod ui_management;
pub mod viewer_states;
use viewer_states::{
    ViewerState, 
    TransformState,
    to_documentation_state,
    to_transform_state,
    to_idle_state,
    to_neutral_transform_state,
    to_translation_transform_state,
    to_rotation_transform_state,
    to_scale_transform_state,
    }
use ui_management::{
    spawn_doc_ui, 
    despawn_doc_ui
}

use camera_management:: {
    camera_keyboard_translation_system,
    camera_mouse_keyboard_rotation_system
}


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
        .add_plugins(DefaultPlugins)
        .init_state::<ViewerState>()
        .init_state::<TransformState>()
        .init_resource::<CameraSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, camera_keyboard_translation_system.run_if(in_state(ViewerState::Idle)))//regarder comment implémenter un raycast
        .add_systems(Update, camera_mouse_keyboard_rotation_system.run_if(in_state(ViewerState::Idle)))
        .add_systems(Update, to_documentation_state)
        .add_systems(Update, to_transform_state)
        .add_systems(Update, to_idle_state)
        .add_systems(Update, to_neutral_transform_state.run_if(in_state(ViewerState::Transform)))
        .add_systems(Update, to_translation_transform_state.run_if(in_state(ViewerState::Transform)))
        .add_systems(Update, to_rotation_transform_state.run_if(in_state(ViewerState::Transform)))
        .add_systems(Update, to_scale_transform_state.run_if(in_state(ViewerState::Transform)))
        .add_systems(OnEnter(ViewerState::Documentation), spawn_doc_ui)
        .add_systems(OnExit(ViewerState::Documentation), despawn_doc_ui)
        .run();
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


