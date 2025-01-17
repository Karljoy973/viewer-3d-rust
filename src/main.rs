use bevy::{ prelude::*, window::PrimaryWindow};
use rand::prelude::*;
pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;
pub const ENEMY_COUNT: usize = 4; 
pub const ENEMY_SIZE: f32 = 64.; 
pub const ENEMY_SPEED: f32 = 480.;
pub const NUMBER_OF_STARS: usize = 10; 
pub const STAR_SIZE: f32 = 30.;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2, 

}
#[derive(Component)]
pub struct Star {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, confine_player_mouvement)
        .add_systems(Update, player_mouvement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_movement)
        .run();
}

fn spawn_camera(mut commands: Commands) {
       commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        }
    ));
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite::from_image(
        asset_server.load("sprites/ball_blue_large.png"),
    ), Player {}));
}


fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=ENEMY_COUNT {
            let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/ball_red_large.png")), 
                Enemy {
                    direction: Vec2::new(random::<f32>(),random::<f32>()).normalize()
                }, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>){
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction* ENEMY_SPEED* time.delta_secs();
    }

}
fn update_enemy_direction(mut enemy_query:Query<(&Transform, &mut Enemy)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = match window_query.get_single(){
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for (transform, mut enemy) in enemy_query.iter_mut() {
            let mut direction_changed: bool = false; 
            let half_enemy_size: f32 = ENEMY_SIZE/2.;
            let  translation:Vec3 = transform.translation;  
            let enemy_x_min: f32 =  half_enemy_size - window.width()/2. ;
            let enemy_x_max: f32 = window.width()/2. - half_enemy_size;
            let enemy_y_min: f32 =  half_enemy_size - window.height()/2.;
            let enemy_y_max: f32=   window.height()/2. - half_enemy_size ;

            if  translation.x < enemy_x_min ||translation.x > enemy_x_max{ enemy.direction.x *=-1.; direction_changed = true;};
            if  translation.y<enemy_y_min  ||translation.y > enemy_y_max {enemy.direction.y *= -1.;direction_changed = true;};

            if direction_changed {
                
            }


        }
}

fn player_mouvement( keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>){
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO; 
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.,0.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.,1.,0.)
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.,0.,0.)
        }

        if direction.length() > 0.00 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();

    }
}

pub fn confine_player_mouvement(mut player_query: Query<&mut Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        let half_player_size: f32 = PLAYER_SIZE/2.;
        let player_x_min: f32 =  half_player_size - window.width()/2. ;
        let player_x_max: f32 = window.width()/2. - half_player_size;
        let player_y_min: f32 =  half_player_size - window.height()/2.;
        let player_y_max: f32=   window.height()/2. - half_player_size ;

        let mut translation:Vec3 = player_transform.translation; 
        if translation.x < player_x_min { translation.x = player_x_min;}
        else if translation.x > player_x_max {translation.x = player_x_max;}
        if translation.y < player_y_min {translation.y = player_y_min;}
        else if translation.y > player_y_max {translation.y = player_y_max;}
        player_transform.translation = translation; 
    }
}

pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = match window_query.get_single() {
            Ok(v) => v,
            Err(_) => panic!("No Window found")
        };
        for _ in 0..=NUMBER_OF_STARS {
            let random_x = random::<f32>() * window.width()/2.;
            let random_y = random::<f32>() * window.height()/2.;

            commands.spawn( 
                (Sprite::from_image( asset_server.load("sprites/star.png")), 
                Star {}, 
                Transform::from_xyz(random_x, random_y, 0.)));
            
        } 
}