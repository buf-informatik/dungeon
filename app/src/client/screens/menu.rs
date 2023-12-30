#[path = "../logic/router.rs"]
mod router;
use router::cache::Cache;

pub fn draw_menu_screen(cache: &mut Cache) {
    println!("Menu screen");
}
