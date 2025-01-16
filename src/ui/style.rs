use bevy::prelude::*; 

pub fn spawn_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ui_entity = build_main_ui(commands, asset_server);
}
pub fn despawn_ui() {}

pub fn build_main_ui(commands: &mut Commands, asset_server: &Res<AssetServer>)-> Entity {
    let ui_entity: Entity = commands.spawn(NodeBundle {
        background_color: Color::BLUE.into(),
        ..default()
    }).id();
    ui_entity
}