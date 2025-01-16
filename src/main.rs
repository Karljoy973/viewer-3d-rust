use bevy::{prelude::*, ui::UiPlugin};
use ui::*;
mod hello_world_plugin;


fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello_world_plugin::HelloPlugin)
        .add_plugins(UiPlugin)
        .run();

}


