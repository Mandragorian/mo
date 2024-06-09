use mo::{run_decoder, run_menu, run_code_table};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use std::io::{stdout, Write};

fn main() {
    let (width, height) = termion::terminal_size().unwrap();
    let mut terminal = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    loop {
        match run_menu(&mut terminal, width, height) {
         mo::Choice::DecodeMode => run_decoder(&mut terminal, width, height),
         mo::Choice::CodeTable => run_code_table(&mut terminal, width, height),
         mo::Choice::EOF | mo::Choice::Shutdown => std::process::exit(0),
        }
    }
}
