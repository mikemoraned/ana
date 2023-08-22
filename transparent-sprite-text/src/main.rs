//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(6.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let aspect = 0.25;

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 1.5)
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    });

    // let transparent_mesh = shape::Plane::from_size(6.0);
    // let transparent_material = Color::rgba(1.0, 0.5, 0.3, 0.5);
    // let sprite_handle = asset_server.load("branding/icon.png");
    // let transparent_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(sprite_handle),
    //     reflectance: 0.02,
    //     unlit: false,
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(transparent_mesh.into()),
    //     material: transparent_material,
    //     transform: Transform::from_xyz(-1.0, 0.5, -1.5),
    //     ..default()
    // });
    // let sprite_handle = asset_server.load("branding/icon.png");
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         // Alpha channel of the color controls transparency.
    //         color: Color::rgba(0.0, 0.0, 1.0, 0.7),
    //         ..default()
    //     },
    //     texture: sprite_handle.clone(),
    //     transform: Transform::from_xyz(-1.0, 0.5, -1.5),
    //     ..default()
    // });

    // transparent sphere, uses `alpha_mode: Mask(f32)`
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         Mesh::try_from(shape::Icosphere {
    //             radius: 0.5,
    //             subdivisions: 3,
    //         })
    //         .unwrap(),
    //     ),
    //     material: materials.add(StandardMaterial {
    //         // Alpha channel of the color controls transparency.
    //         // We set it to 0.0 here, because it will be changed over time in the
    //         // `fade_transparency` function.
    //         // Note that the transparency has no effect on the objects shadow.
    //         base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
    //         // Mask sets a cutoff for transparency. Alpha values below are fully transparent,
    //         // alpha values above are fully opaque.
    //         alpha_mode: AlphaMode::Mask(0.5),
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(1.0, 0.5, -1.5),
    //     ..default()
    // });
    // transparent unlit sphere, uses `alpha_mode: Mask(f32)`
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         Mesh::try_from(shape::Icosphere {
    //             radius: 0.5,
    //             subdivisions: 3,
    //         })
    //         .unwrap(),
    //     ),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::rgba(0.2, 0.7, 0.1, 0.0),
    //         alpha_mode: AlphaMode::Mask(0.5),
    //         unlit: true,
    //         ..default()
    //     }),
    //     transform: Transform::from_xyz(-1.0, 0.5, -1.5),
    //     ..default()
    // });
    // transparent cube, uses `alpha_mode: Blend`
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     // Notice how there is no need to set the `alpha_mode` explicitly here.
    //     // When converting a color to a material using `into()`, the alpha mode is
    //     // automatically set to `Blend` if the alpha channel is anything lower than 1.0.
    //     material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.0).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // opaque sphere
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(
    //         Mesh::try_from(shape::Icosphere {
    //             radius: 0.5,
    //             subdivisions: 3,
    //         })
    //         .unwrap(),
    //     ),
    //     material: materials.add(Color::rgb(0.7, 0.2, 0.1).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, -1.5),
    //     ..default()
    // });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));
}

