
use bevy::prelude::*;

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
    };
use ui_management::{
    spawn_doc_ui, 
    despawn_doc_ui
};

use camera_management:: {
    camera_keyboard_translation_system,
    camera_mouse_keyboard_rotation_system, CameraSettings
};



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


