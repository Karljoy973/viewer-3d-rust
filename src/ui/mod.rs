
use bevy::prelude::*;
use system::layout::*;

struct UI; 
 
impl UI for UiPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(ui);
    }
}

fn ui() {
    println!("Hello Wold")
}