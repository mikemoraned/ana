use cosmic_text::fontdb::{Source, Database};
use cosmic_text::{Attrs, Color, FontSystem, SwashCache, Buffer, Metrics, Shaping};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn main() {

    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();

    let mut db = Database::new();
    let locale = "en-US";

    // let mut font_system = FontSystem::new_with_fonts(
    //     vec![Source::File("assets/fonts/FiraSans-Bold.ttf".into())].into_iter()
    // );

    db.load_fonts_dir("assets/fonts/");
        
    // let mut font_system = FontSystem::new_with_locale_and_db(locale.into(), db);

    println!("{:?}", font_system.db());

    font_system.db_mut().load_font_file("assets/fonts/Noto_Emoji/NotoEmoji-VariableFont_wght.ttf").unwrap();

    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Text metrics indicate the font size and line height of a buffer
    // let metrics = Metrics::new(14.0, 20.0);
    let metrics = Metrics::new(14.0 * 2.0, 20.0 * 2.0);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);

    // Borrow buffer together with the font system for more convenient method calls
    let mut buffer = buffer.borrow_with(&mut font_system);

    // Set a size for the text buffer, in pixels
    let buf_width = 80.0 * 4.0;
    let buf_height = 25.0 * 4.0;
    buffer.set_size(buf_width, buf_height);

    // Attributes indicate what font to choose
    let mut attrs = Attrs::new();

    attrs = attrs.family(cosmic_text::Family::Name("Noto Emoji"));

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

    let mut imgbuf: image::ImageBuffer<_, Vec<_>> = image::ImageBuffer::new(buf_width as u32, buf_height as u32);

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
