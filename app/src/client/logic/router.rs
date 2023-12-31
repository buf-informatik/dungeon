use crate::client;
use client::logic;
use client::screens;
use logic::cache::Cache;
use screens::loading::draw_loading_screen;
use screens::menu::draw_menu_screen;

pub enum Screen {
    Menu,
    // Settings,
    Loading,
}

pub fn navigate(screen: &Screen, cache: &mut Cache) {
    match screen {
        Screen::Menu => draw_menu_screen(cache),
        Screen::Loading => draw_loading_screen(cache),
    }
}
