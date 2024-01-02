use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt;

use crate::client;
use client::logic;
use client::screens;
use logic::cache::Cache;
use screens::screen::Screen;

#[derive(Debug)]
pub enum ScreenName {
    Menu,
    Game,
    Loading,
    Maps,
    Shop,
    Settings,
    Character,
    CharacterSelection,
}

impl fmt::Display for ScreenName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn navigate(screen: ScreenName, cache: &mut Cache, canvas: &mut Canvas<Window>,  background: Option<&str>) {
    let mut screen_obj = Screen {
        name: screen,
        _cache: cache,
        canvas: canvas,
        background: background,
    };

    screen_obj.init();
}
