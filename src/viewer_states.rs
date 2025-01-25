use bevy::{input::keyboard::KeyboardInput, prelude::*, utils::hashbrown::Equivalent};


#[derive(States)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
pub enum ViewerState {
    #[default]
    Idle, //the input system affects the camera 
    Transform, //the input system affects the selected mesh 
    Documentation, // no input system works and there is a ui apearing
}

#[derive(States)]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
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
        if !current_transform_state.get().eq(&TransformState::Neutral) && keyboard_event.key_code.eq(&KeyCode::KeyN) {
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
                keyboard_event.key_code.eq(&KeyCode::KeyD) {
                    println!("Current state : {:?}", current_viewer_state.get());
                    next_viewer_state.set(ViewerState::Documentation);
        }   
    }
}