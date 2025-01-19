use std::io::Result;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
    symbols::border,
};
use crossterm::event::{self, Event, KeyEventKind, KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    menu: crate::menu::Menu,
    table: crate::code_table::Table,
    decoder: crate::decoder::Decoder,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
        where
            Self: Sized 
    {
        let menu = crate::menu::Menu::new();
        menu.render(area, buf); 
    }
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut crate::tui::Tui) -> Result<()> {
        loop {
            match self.menu.run(terminal)?{
                    crate::menu::Choice::DecodeMode => {self.decoder.run(terminal)?;}
                    crate::menu::Choice::CodeTable => {self.table.run(terminal)?;}
                    crate::menu::Choice::Shutdown => return Ok(()),
                    _ => {},
            };
        }
    }
}