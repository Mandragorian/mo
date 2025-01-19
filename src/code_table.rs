use std::{default, io::Result};
use ratatui::{
    prelude::*,
    widgets::{Table as RatatuiTable, TableState, Row, Widget, StatefulWidget},
    layout::Constraint,
};
use crossterm::event::{self, Event, KeyEventKind, KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct Table {
    exit: bool,
}

impl Widget for &Table {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized 
    {
        let rows = [
            Row::new(["Option 1"]), Row::new(["Option 2"]), Row::new(["Option 3"])];
        let mut rows = vec![];
        for (i, (c1, c2)) in ('a'..='m').zip('n'..='z').enumerate() {
            let symbols1 = crate::encode_character(c1).unwrap();
            let symbols2 = crate::encode_character(c2).unwrap();
            rows.push(Row::new([format!("{}", c1), format!("{}", symbols1), c2.to_string(), format!("{}", symbols2)]));
        }
        let widths = [Constraint::Length(10), Constraint::Length(20), Constraint::Length(10), Constraint::Length(20)];
        let table = RatatuiTable::new(rows, widths);
        Widget::render(table, area, buf)
    }
}

impl Table {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, terminal: &mut crate::tui::Tui) -> Result<()> {
        terminal.clear();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self,frame.size())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit = true
            }
            _ => {}
        }
    }
}