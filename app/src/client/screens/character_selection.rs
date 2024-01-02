use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::client::logic::cache::Cache;

pub fn draw_character_selection_screen(_cache: &mut Cache, _canvas: &mut Canvas<Window>) {
    println!("Character selection screen");
}
