use bevy::prelude::*;
mod people_plugin;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(people_plugin::HelloPlugin)
        .run();

}
