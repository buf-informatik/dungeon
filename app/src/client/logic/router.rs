// #[path = "../../config.rs"]
// mod config;
// use config::Cache;
// use config::Screen;
#[path = "../screens/loading.rs"]
mod loading;
use loading::draw_loading_screen;
#[path = "../screens/menu.rs"]
mod menu;
use menu::draw_menu_screen;
#[path = "cache.rs"]
pub mod cache;
use cache::Cache;

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
