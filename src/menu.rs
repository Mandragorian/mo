use std::io::Result;
use ratatui::{
    prelude::*,
    widgets::{Table, TableState, Row, Widget, StatefulWidget},
    layout::Constraint,
};
use crossterm::event::{self, Event, KeyEventKind, KeyCode, KeyEvent};

macro_rules! choice {
    ( $( $x:ident; $s:literal ),* ) => {
            #[derive(Debug, Clone, Copy, Default)]
            #[repr(usize)]
            pub enum Choice {
                #[default]
                $($x,)*
            }

            impl Choice {
                fn rows() -> Vec<Row<'static>> {
                    vec![$(Row::new([$s]),)*]
                }
            }
    };
}

choice!(
    DecodeMode; "Decode",
    CodeTable; "Morse Code Table",
    Shutdown; "Exit"
);

impl Choice {
    fn next(&mut self) {
        match self {
            Self::DecodeMode => *self = Self::CodeTable,
            Self::CodeTable => *self = Self::Shutdown,
            Self::Shutdown => {}
        }
    }

    fn prev(&mut self) {
        match self {
            Self::DecodeMode => {}
            Self::CodeTable => *self = Self::DecodeMode,
            Self::Shutdown => *self = Self::CodeTable,
        }
    }
}

#[derive(Debug, Default)]
pub struct Menu {
    exit: bool,
    selection: Choice,
}

impl Widget for &Menu {
    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized 
    {
        // let rows = [Row::new(["Option 1"]), Row::new(["Option 2"]), Row::new(["Option 3"])];
        let rows = Choice::rows();
        let widths = [Constraint::Percentage(100)];
        let table = Table::new(rows, widths).highlight_symbol(">>");
        let mut state = TableState::new();
        state.select(Some(self.selection as usize));
        StatefulWidget::render(table, area, buf, &mut state)
    }
}

impl Menu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, terminal: &mut crate::tui::Tui) -> Result<Choice> {
        terminal.clear().expect("couldn't clear terminal");
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        println!("returning: {:?}", self.selection);
        self.exit = false;
        Ok(self.selection)
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
            KeyCode::Up => {self.selection.prev()}
            KeyCode::Down => {self.selection.next()}
            KeyCode::Enter => self.exit = true,

            _ => {}
        }
    }
}
