mod app;

use std::io::{Result};

use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(music_window)?;

        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn music_window(frame: &mut Frame) {
    frame.render_widget("Music Window", frame.area());
}