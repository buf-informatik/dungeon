#[path = "../logic/router.rs"]
mod router;
use router::cache::Cache;

pub fn draw_loading_screen(&mut cache: &mut Cache) {
    println!("Loading screen");
}
