use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub enum ImageOptions {}

pub fn end_game() {
    std::process::exit(0);
}

pub fn set_background_image(canvas: &mut WindowCanvas, image: Option<&str>) {
    canvas.clear();
    match canvas.output_size() {
        Ok((width, height)) => {
            println!("Canvas size: {}x{}", width, height);
            match image {
                Some(image) => draw_image(canvas, image, 0, 0, width, height),
                None => canvas.present(),
            }
        }
        Err(e) => {
            eprintln!("Failed to get the canvas size: {}", e);
        }
    }
}

pub fn set_background_color(canvas: &mut WindowCanvas) {
    canvas.clear();
    canvas.present();
}

pub fn draw_image(canvas: &mut WindowCanvas, image: &str, x: i32, y: i32, width: u32, height: u32) {
    let img = format!("./client/assets/{}", image);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(img).unwrap();
    canvas
        .copy(&texture, None, Some(Rect::new(x, y, width, height)))
        .unwrap();
    canvas.present();
}
