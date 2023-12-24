use sdl2::event::Event;
use sdl2::keyboard::Keycode;
mod actions;

pub fn handle_events(&event: Event) {
    match event {
        Event::Quit { .. } => actions::end_game(),
        Event::KeyDown { .. } => key_events(keycode, keymod),
        _ => {}
    }
}

fn key_events(&keycode: Option<Keycode>, &keymod: Mod) {
    match keymode {
        Keycode::Escape => actions::end_game(),
    }
}
