use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics, Shaping};
use image::Rgb;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn main() {

    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(14.0, 20.0);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);

    // Borrow buffer together with the font system for more convenient method calls
    let mut buffer = buffer.borrow_with(&mut font_system);

    // Set a size for the text buffer, in pixels
    buffer.set_size(80.0, 25.0);

    // Attributes indicate what font to choose
    let attrs = Attrs::new();

    // Add some text!
    buffer.set_text("Hello, Rust! 🦀\n", attrs, Shaping::Advanced);

    // Perform shaping as desired
    buffer.shape_until_scroll();

    // Inspect the output runs
    for run in buffer.layout_runs() {
        for glyph in run.glyphs.iter() {
            println!("{:#?}", glyph);
        }
    }

    // Create a default text color
    let text_color = Color::rgb(0xFF, 0xFF, 0xFF);

    let mut imgbuf: image::ImageBuffer<_, Vec<_>> = image::ImageBuffer::new(80 * 2, 25 * 2);

    // Draw the buffer (for performance, instead use SwashCache directly)
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        println!("{x},{y},{w},{h},{color:?}");
        // Fill in your code here for drawing rectangles
        let v = [color.r(), color.g(), color.b(), color.a()];
        let rgba = image::Rgba(v);
        println!("{v:?} -> {rgba:?}");
        draw_filled_rect_mut(&mut imgbuf, Rect::at(x, y).of_size(w, h), rgba);
    });

    imgbuf.save("image.png").unwrap();
}
