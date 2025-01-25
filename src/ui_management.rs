use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

pub fn spawn_doc_ui(mut commands: Commands, mut active_camera_query: Query<&mut Camera, With<Camera3d>>){
    for camera in active_camera_query.iter_mut(){
        camera.into_inner().order +=1; 
    }

    commands.spawn(Camera2d);
    commands
    .spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child((
                Text::new("Button"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
    }); 
}

pub fn despawn_doc_ui(mut command: Commands, ui_query: Query<Entity, With<ComputedNode>>, camera_query: Query<Entity, With<Camera2d>>){
 
    for ui in ui_query.iter() {
        command.entity(ui).try_despawn_recursive();
        
    }
    for camera in camera_query.iter() {
        command.entity(camera).try_despawn();
    }
}