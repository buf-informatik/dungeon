use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;
use std::time::Duration;

pub mod client;
use cache::Cache;
use client::logic;
use event_handler::handle_events;
use logic::cache;
use logic::event_handler;
use logic::router;
use router::navigate;
use router::ScreenName;

fn main() -> Result<(), String> {
    let mut cache: Cache = cache::new();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

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
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut first_render = true;

    'running: loop {
        if first_render {
            navigate(
                ScreenName::Loading,
                &mut cache,
                &mut canvas,
                Some("background.jpg"),
            );
            ::std::thread::sleep(Duration::new(3, 0));
            navigate(ScreenName::Menu, &mut cache, &mut canvas, None);
        }
        for event in event_pump.poll_iter() {
            if !handle_events(event, &mut canvas) {
                break 'running;
            }
        }
        first_render = false;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
