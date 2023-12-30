use cache::Cache;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::time::Duration;
#[path = "client/logic/event_handler.rs"]
mod event_handler;
use event_handler::handle_events;
#[path = "client/logic/router.rs"]
mod router;
use router::cache;
use router::navigate;
use router::Screen;

fn main() -> Result<(), String> {
    let mut cache: cache::Cache = cache::new();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Dungeon", 960, 540)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut event_pump = sdl_context.event_pump()?;
    let mut first_render = true;

    'running: loop {
        if first_render {
            navigate(&Screen::Loading, &mut cache);
            ::std::thread::sleep(Duration::new(3, 0));
            navigate(&Screen::Menu, &mut cache);
        }
        for event in event_pump.poll_iter() {
            if !handle_events(event) {
                break 'running;
            }
        }
        first_render = false;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
