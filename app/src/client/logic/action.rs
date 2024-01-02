use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub enum ImagePosition {
    Center,
    Left,
    Right,
    Num(i32),
}
pub struct ImageOption<'a> {
    pub(crate) x: ImagePosition,
    pub(crate) y: ImagePosition,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) image: &'a str,
}

pub fn end_game() {
    std::process::exit(0);
}

pub fn set_background_image(canvas: &mut WindowCanvas, image: Option<&str>) {
    canvas.clear();
    match canvas.output_size() {
        Ok((width, height)) => match image {
            Some(image) => add_image(
                ImageOption {
                    x: ImagePosition::Num(0),
                    y: ImagePosition::Num(0),
                    width: width,
                    height: height,
                    image: image,
                },
                canvas,
            ),
            None => canvas.present(),
        },
        Err(e) => {
            eprintln!("Failed to get the canvas size: {}", e);
        }
    }
}

pub fn set_background_color(canvas: &mut WindowCanvas, color: Color) {
    canvas.clear();
    canvas.set_draw_color(color);
    canvas.present();
}

pub fn add_image(options: ImageOption, canvas: &mut WindowCanvas) {
    let img = format!("./client/assets/{}", options.image);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(img).unwrap();
    let x = get_image_position(options.x, canvas.output_size().unwrap().0, options.width);
    let y = get_image_position(options.y, canvas.output_size().unwrap().1, options.height);

    canvas
        .copy(
            &texture,
            None,
            Some(Rect::new(x, y, options.width, options.height)),
        )
        .unwrap();
    canvas.present();
}

pub fn get_image_position(position: ImagePosition, canvas_length: u32, image_lenght: u32) -> i32 {
    match position {
        ImagePosition::Num(num) => num,
        ImagePosition::Center => (canvas_length as i32 - image_lenght as i32) / 2,
        ImagePosition::Left => 0,
        ImagePosition::Right => canvas_length as i32 - image_lenght as i32,
    }
}
