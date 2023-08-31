#![windows_subsystem = "windows"]
mod systray;
mod gui;

use gui::MyApp;
use iced::{Sandbox, Settings, window::Position};

fn main() {
    let _thread_join_handle = std::thread::spawn(|| {
        systray::create_systray().unwrap();
    });
    let mut set: Settings<_> = Settings::default();
    set.window.position = Position::Specific(1600, 800);
    set.window.size = (300, 200);
    set.window.always_on_top = true;
    MyApp::run(set).unwrap();
    //MyApp::run(iced::Settings::default()).unwrap();
}
