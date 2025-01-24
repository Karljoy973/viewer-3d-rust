
use bevy::{input::mouse::{ AccumulatedMouseMotion}, prelude::*};

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
        .add_plugins(DefaultPlugins)
        .init_state::<ViewerState>()
        .init_state::<TransformState>()
        .init_resource::<CameraSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, camera_keyboard_translation_system.run_if(in_state(ViewerState::Idle)))
        .add_systems(Update, camera_mouse_keyboard_rotation_system.run_if(in_state(ViewerState::Idle)))
        .add_systems(Update, to_documentation_state)
        .add_systems(Update, to_idle_state)
        .add_systems(Update, to_transform_state)
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
        else if key.pressed(KeyCode::KeyZ) {
            
        }
    }

}

#[derive(Eq, PartialEq, States, Debug, Clone, Copy, Hash, Default)]
pub enum ViewerState {
    #[default]
    Idle, //the input system affects the camera 
    Transform, //the input system affects the selected mesh 
    Documentation, // no input system works and there is a ui apearing
}

#[derive(Eq, PartialEq, States, Debug, Clone, Copy, Hash, Default)]
pub enum TransformState {
    #[default]
    Neutral, 
    Translation, 
    Rotation, 
    Scale
}




pub fn to_idle_state(
    key: Res<ButtonInput<KeyCode>>,
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>
){
    if !current_viewer_state.get().eq(&ViewerState::Idle) && key.pressed (KeyCode::ShiftLeft) && key.pressed (KeyCode::KeyI){
        next_viewer_state.set(ViewerState::Idle);
        println!("Current state : Idle State");
    }
}
pub fn to_transform_state(
    key: Res<ButtonInput<KeyCode>>, 
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>){
    if !current_viewer_state.get().eq(&ViewerState::Transform) && key.pressed (KeyCode::ShiftLeft) && key.pressed (KeyCode::KeyT){
        next_viewer_state.set(ViewerState::Transform);
        println!("Current state : Transform State");
    }
}
pub fn to_documentation_state(
    key: Res<ButtonInput<KeyCode>>, 
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>){
if !current_viewer_state.get().eq(&ViewerState::Documentation) && key.pressed (KeyCode::ShiftLeft) && key.pressed (KeyCode::KeyD) {
        next_viewer_state.set(ViewerState::Documentation);
        println!("Current state : Documentation State");
    }    
}
