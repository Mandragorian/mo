use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
    symbols::border,
};

use mo::{run_decoder};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use std::io::{stdout, Result, Write};




fn main() -> Result<()> {
    let mut terminal = mo::tui::init()?;
    let app_result = mo::app::App::default().run(&mut terminal);
    mo::tui::restore()?;
    app_result
}
