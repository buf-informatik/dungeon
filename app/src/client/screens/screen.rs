use sdl2::render::WindowCanvas;

use crate::client;
use action::set_background_image;
use cache::Cache;
use client::logic;
use logic::action;
use logic::cache;
use logic::router;
use router::ScreenName;

pub struct Screen<'a> {
    pub(crate) name: ScreenName,
    pub(crate) _cache: &'a mut Cache,
    pub(crate) canvas: &'a mut WindowCanvas,
    pub(crate) background: Option<&'a str>,
}

impl<'a> Screen<'a> {
    pub fn init(&mut self) {
        let screen: String = self.name.to_string();
        println!("Screen {} initialized", screen);
        set_background_image(self.canvas, self.background);
    }
}
