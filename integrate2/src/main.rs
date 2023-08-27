use std::{fs::OpenOptions, error::Error, io::{Write, Cursor}};

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() -> Result<(), Box<dyn Error>> {
    let message = "hello bevy world! how are you doing?";
    let png_image_bytes = 
        layout_text_as_png_image(message, 
        28.0, 40.0, 
        200.0, 120.0);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("assets/image.png")?;

    file.write_all(png_image_bytes.as_slice())?;

    App::new()
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(12.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("image.png");

    // create a new quad mesh. this is what we will apply the texture to
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        8.0,
        2.0,
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
        transform: Transform::from_xyz(0.0, 1.0, 6.0)
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 12.0),
        ..default()
    });
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 14.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));
}

fn layout_text_as_png_image(
    s: &str, 
    font_size: f32, line_height: f32,
    width: f32, height: f32
) -> Vec<u8> {
    use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics, Shaping};
    use imageproc::drawing::draw_filled_rect_mut;
    use imageproc::rect::Rect;

    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();

    let metrics = Metrics::new(font_size, line_height);

    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    buffer.set_size(width, height);

    let attrs = Attrs::new();
    buffer.set_text(s, attrs, Shaping::Advanced);
    
    buffer.shape_until_scroll();

    let text_color = Color::rgb(0xFF, 0xFF, 0xFF);

    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);
    let bg = image::Rgba([255, 255, 255, 10]);
    draw_filled_rect_mut(&mut imgbuf, 
        Rect::at(0, 0).of_size(width as u32, height as u32), bg);
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        let rgba = image::Rgba([color.r(), color.g(), color.b(), color.a()]);
        draw_filled_rect_mut(&mut imgbuf, 
            Rect::at(x, y).of_size(w, h), rgba);
    });

    let mut bytes: Vec<u8> = Vec::new();
    imgbuf.write_to(
        &mut Cursor::new(&mut bytes), 
        image::ImageOutputFormat::Png).unwrap();
    bytes
}
