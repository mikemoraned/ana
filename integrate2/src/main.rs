use std::{fs::OpenOptions, error::Error, io::{Write, Cursor}};

fn main() -> Result<(), Box<dyn Error>> {
    let message = "hello bevy\nworld!";
    let png_image_bytes = 
        layout_text_as_png_image(message, 
        28.0, 40.0, 
        320.0, 100.0);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("image.png")?;

    file.write_all(png_image_bytes.as_slice())?;

    Ok(())
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
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        let rgba = image::Rgba([color.r(), color.g(), color.b(), color.a()]);
        draw_filled_rect_mut(&mut imgbuf, Rect::at(x, y).of_size(w, h), rgba);
    });

    let mut bytes: Vec<u8> = Vec::new();
    imgbuf.write_to(
        &mut Cursor::new(&mut bytes), 
        image::ImageOutputFormat::Png).unwrap();
    bytes
}
