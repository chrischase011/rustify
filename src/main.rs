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

