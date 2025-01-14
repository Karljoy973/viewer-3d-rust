use blue_engine::{
    header::{ Engine, ObjectSettings },
    primitive_shapes::triangle, WindowDescriptor, primitive_shapes::three_dimensions::cube, 
};

fn main() {
    // initialize the engine
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1920,
        height: 1080,
        title: "My Awesome Render",
        ..Default::default() 
    }).expect("engine couldn't be initialized");

    // create a triangle
    // triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects);

    cube("A cool looking cube", &mut engine.renderer, &mut engine.objects);

    // run the engine
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}