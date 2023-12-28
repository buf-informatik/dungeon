#[path = "../../config.rs"]
mod config;
use config::Screen;

pub fn navigate(screen: &Screen) {
    match screen {
        Screen::Home => println!("Home"),
        Screen::Login => println!("Login"),
        Screen::Register => println!("Register"),
        Screen::Profile => println!("Profile"),
        Screen::Settings => println!("Settings"),
        Screen::NotFound => println!("Not Found"),
    }
}
