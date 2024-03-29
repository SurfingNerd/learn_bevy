//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.



mod hexagon;
mod map;
mod examples;
mod hex_demo;
mod hex2d_demo;
mod components;
// use crate::map::TiledMapPlugin;



use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};
use examples::run_hexagon_columns;
use hex2d_demo::run_hex2d_demo;
// use bevy_ecs_tilemap::TilemapPlugin;
// use bevy_flycam::{PlayerPlugin, MovementSettings};
use hex_demo::run_hex_demo;
use hexagon::Hexagon3D;
// use bevy::log;




fn main() {

    // runHexagonColums();
    // run_hexagon_columns();
    // run_hex_demo();
    run_hex2d_demo();

    // App::new()
    //     .insert_resource(Msaa { samples: 4 })
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(TilemapPlugin)
    //     //.add_plugin(TiledMapPlugin)
    //     .add_plugin(PlayerPlugin)
    //     .insert_resource(MovementSettings {
    //         sensitivity: 0.00015, // default: 0.00012
    //         speed: 12.0,          // default: 12.0
    //     })
    //     .add_startup_system(setup)
    //     //.add_startup_system(map::map_startup)
    //     //.add_system(set_texture_filters_to_nearest)
    //     //.add_system(cam_movement)
    //     .add_system(rotate)
    //     .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.;

#[allow(dead_code)]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    
    let shapes = [
       // meshes.add(shape::Cube::default().into()),
       // meshes.add(shape::Box::default().into()),
       // meshes.add(shape::Capsule::default().into()),
       // meshes.add(shape::Torus::default().into()),
       // meshes.add(shape::Icosphere::default().into()),
       // meshes.add(shape::UVSphere::default().into()),
       meshes.add( Hexagon3D::default().into()),
       meshes.add( Hexagon3D::default().into())
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        
        info!(i);
        commands
            .spawn_bundle(PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                        2.0,
                        0.0,
                    ),
                    ..default()
                },
                ..Default::default()
            })
            .insert(Shape);
    }

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..Default::default()
    });

    // ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..Default::default()
    });

    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 64.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    //     ..Default::default()
    // });

    //let mut cam = Camera2dBundle::new_with_far(200.0);
    // let cam = Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // };
    
    //log::info!("{:?}", cam.transform);

    // commands.spawn_bundle(cam);
    
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}


fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.seconds_since_startup() as f32 / 2.)
            * Quat::from_rotation_x(-std::f32::consts::PI / 4.)
    }
}

// fn cam_movement(time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(&mut FlyCamera, &mut Transform)>,) {

// }

// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}