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

