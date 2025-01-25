
use bevy::{input::{keyboard::{Key, KeyboardInput}, mouse::AccumulatedMouseMotion}, prelude::*, utils::hashbrown::Equivalent};

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
        .add_systems(Update, camera_keyboard_translation_system.run_if(in_state(ViewerState::Idle)))//regarder comment implémenter un raycast
        .add_systems(Update, camera_mouse_keyboard_rotation_system.run_if(in_state(ViewerState::Idle)))
        // .add_systems(Update, to_documentation_state)
        .add_systems(Update, to_transform_state)
        .add_systems(Update, to_idle_state)
        .add_systems(Update, to_neutral_transform_state)
        .add_systems(Update, to_translation_transform_state)
        .add_systems(Update, to_rotation_transform_state)
        .add_systems(Update, to_scale_transform_state)
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
        else if key.pressed(KeyCode::KeyZ) { // I could use a switch statement (or the ruste equivalent)
            
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

impl std::fmt::Display for TransformState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformState::Neutral => write!(f, "Neutral"), 
            TransformState::Translation => write!(f, "Translation"),
            TransformState::Rotation => write!(f, "Rotation"),
            TransformState::Scale => write!(f, "Scale"),
        }
    }
}

impl std::fmt::Display for ViewerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewerState::Idle => write!(f, "Idle"), 
            ViewerState::Transform => write!(f, "Transform"),
            ViewerState::Documentation => write!(f, "Documentation"),
        }
    }
}


pub fn to_neutral_transform_state(
    current_transform_state: Res<State<TransformState>>, 
    mut next_transform_state: ResMut<NextState<TransformState>>, 
    mut keyboard_events: EventReader<KeyboardInput>
){
    for keyboard_event in keyboard_events.read()
    {
        if !current_transform_state.get().eq(&TransformState::Neutral) && keyboard_event.key_code.eq(&KeyCode::Escape) {
            println!("Current state : Neutural");
            next_transform_state.set(TransformState::Neutral);
    }
}
}
pub fn to_translation_transform_state(
    current_transform_state: Res<State<TransformState>>, 
    mut next_transform_state: ResMut<NextState<TransformState>>, 
    mut keyboard_events: EventReader<KeyboardInput>
){
    for keyboard_event in keyboard_events.read(){
        if !current_transform_state.get().eq(&TransformState::Neutral) && keyboard_event.key_code.eq(&KeyCode::KeyT) {
            println!("Current state : Translation");
            next_transform_state.set(TransformState::Translation);
        }
    }
}
pub fn to_rotation_transform_state(
    current_transform_state: Res<State<TransformState>>, 
    mut next_transform_state: ResMut<NextState<TransformState>>, 
    mut keyboard_events: EventReader<KeyboardInput>
){
    for keyboard_event in keyboard_events.read(){
        if !current_transform_state.get().eq(&TransformState::Neutral) && keyboard_event.key_code.eq(&KeyCode::KeyR) {
        println!("Current state : {}", current_transform_state.get());
        next_transform_state.set(TransformState::Rotation);
        }
    }
}
pub fn to_scale_transform_state(
    current_transform_state: Res<State<TransformState>>, 
    mut next_transform_state: ResMut<NextState<TransformState>>, 
    mut keyboard_events: EventReader<KeyboardInput>
){
    for keyboard_event in keyboard_events.read(){
        if !current_transform_state.get().eq(&TransformState::Neutral) && keyboard_event.key_code.eq(&KeyCode::KeyS)  {
            println!("Current state : {}", current_transform_state.get());
            next_transform_state.set(TransformState::Neutral);
        }
    }
}



pub fn to_idle_state(
    mut keyboard_events: EventReader<KeyboardInput>,
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>
){
    for keyboard_event in keyboard_events.read(){
        if current_viewer_state.get().eq(&ViewerState::Idle) && keyboard_event.key_code.equivalent(&KeyCode::Escape) {
            println!("Already in {} state", current_viewer_state.get());
        }
        else if !current_viewer_state.get().eq(&ViewerState::Idle) && keyboard_event.key_code.equivalent(&KeyCode::Escape) {
            println!("Current state : {}", current_viewer_state.get() );
            next_viewer_state.set(ViewerState::Idle);
    }
}
}
pub fn to_transform_state(
    mut keyboard_events: EventReader<KeyboardInput>, 
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>){

        for keyboard_event in keyboard_events.read(){
            
            if current_viewer_state.get().eq(&ViewerState::Transform) && keyboard_event.key_code.equivalent(&KeyCode::KeyT) {
                println!("Already in {:?} state", current_viewer_state.get());
            }
            else if !current_viewer_state.get().eq(&ViewerState::Transform) && 
                // keyboard_event.key_code.equivalent(&KeyCode::ShiftLeft) && 
                keyboard_event.key_code.equivalent(&KeyCode::KeyT){
                    println!("Current state : {}", current_viewer_state.get());
                    next_viewer_state.set(ViewerState::Transform);
        }
    }
}
pub fn to_documentation_state(
    mut keyboard_events: EventReader<KeyboardInput>,
    current_viewer_state: Res<State<ViewerState>>, 
    mut next_viewer_state: ResMut<NextState<ViewerState>>){
        for keyboard_event in keyboard_events.read(){
            if !current_viewer_state.get().eq(&ViewerState::Documentation) && 
                keyboard_event.key_code.eq(&KeyCode::ShiftLeft) && 
                keyboard_event.key_code.eq(&KeyCode::KeyD) {
                    println!("Current state : {:?}", current_viewer_state.get());
                    next_viewer_state.set(ViewerState::Documentation);
        }   
    }
}


//en transform mode, j'aurais aimé pouvoir montrer le transform tool (au début avec seulement le scale)
//le transform tool serait par rapport au repère local et permetterait d'appliquer une transformation suivant un axe ou suivant un plan 