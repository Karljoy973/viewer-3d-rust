use bevy::{ color::palettes::css::{BLUE, DEEP_PINK, INDIGO, LIME, RED, WHITE_SMOKE}, pbr::CascadeShadowConfigBuilder, prelude::*};


const PI: f32 = 3.141592653589793;
#[derive(Component)]
struct Movable;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup,spawn_light)
        .add_systems(Startup,spawn_cube)
        .add_systems(Startup,spawn_ground)
        .run();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d ::default(), 
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),

    ));
}

fn spawn_light(mut commands : Commands, mut meshes: ResMut<Assets<Mesh>>,  mut materials: ResMut<Assets<StandardMaterial>> ) {
    commands.insert_resource(AmbientLight {
        color: WHITE_SMOKE.into(),
        brightness: 0.6,
    });

      // red point light
    commands
        .spawn((
            PointLight {
                intensity: 100_000.0,
                color: RED.into(),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(1.0, 2.0, 0.0),
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: RED.into(),
                    emissive: LinearRgba::new(4.0, 0.0, 0.0, 0.0),
                    ..default()
                })),
            ));
        });

    // green spot light
    commands
        .spawn((
            SpotLight {
                intensity: 100_000.0,
                color: LIME.into(),
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 0.8,
                ..default()
            },
            Transform::from_xyz(-1.0, 2.0, 0.0).looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        ))
        .with_child((
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.125))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(0.0, 4.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        ));

    // blue point light
    commands
        .spawn((
            PointLight {
                intensity: 100_000.0,
                color: BLUE.into(),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(0.0, 4.0, 0.0),
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: BLUE.into(),
                    emissive: LinearRgba::new(0.0, 0.0, 713.0, 0.0),
                    ..default()
                })),
            ));
        });

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
}

fn spawn_cube(mut commands: Commands,  mut materials: ResMut<Assets<StandardMaterial>>,  mut meshes: ResMut<Assets<Mesh>>) {
      // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DEEP_PINK.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0., 0.0),
        Movable,
    ));
}

fn spawn_ground (mut commands : Commands,  mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>)
{
     // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

     // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate_z(PI / 2.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.15, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: INDIGO.into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform,
    ));
    // back (right) wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate_x(PI / 2.);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.15, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: INDIGO.into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform,
    ));

    // Bevy logo to demonstrate alpha mask shadows
    let mut transform = Transform::from_xyz(-2.2, 0.5, 1.0);
    transform.rotate_y(PI / 8.);
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(2.0, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("branding/bevy_logo_light.png")),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Mask(0.5),
            cull_mode: None,
            ..default()
        })),
        transform,
        Movable,
    ));

}