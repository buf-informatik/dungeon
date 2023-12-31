use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use crate::client::logic::action;

pub fn handle_events(event: Event) -> bool {
    match event {
        Event::Quit { .. } => action::end_game(),
        Event::KeyDown {
            keycode, keymod, ..
        } => key_events(keycode, keymod),
        Event::KeyUp { .. } => {}
        _ => {}
    }
    return true;
}

fn key_events(keycode: Option<Keycode>, _keymod: Mod) {
    match keycode {
        Some(Keycode::Escape) => action::end_game(),
        _ => {}
    }
}
