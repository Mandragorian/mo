//#![feature(variant_count)]
use std::io::{stdin, stdout, Write};
use std::sync::{mpsc::channel, Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod morse;
mod ring;
pub mod menu;
pub mod code_table;
pub mod tui;
pub mod app;
pub mod decoder;

use morse::{decode_symbols, encode_character};
use ring::RingBuffer;

#[derive(Debug)]
enum Events {
    Press,
    Release,
    ClearMessage,
    Tick,
    Pause,
    Shutdown,
}

#[derive(Debug)]
struct State {
    buf: RingBuffer,
    decoded: Option<char>,
    message: String,
    paused: bool,
}

const WINDOW: u16 = 59;

pub fn run_decoder<W: Write + Send>(stdout: &mut MouseTerminal<W>, width: u16, height: u16) {
    let margin = (width - WINDOW) / 2;
    let state = Arc::new(RwLock::new(State {
        buf: RingBuffer::new(width as usize),
        decoded: None,
        message: String::new(),
        paused: false,
    }));
    let stdin = stdin();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    write!(
        stdout,
        "{}{}c to clear.",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    write!(
        stdout,
        "{}<space> to pause.",
        termion::cursor::Goto(1, 2)
    )
    .unwrap();
    write!(
        stdout,
        "{}q to exit. Dah, dah, dit, dah!",
        termion::cursor::Goto(1, 3)
    )
    .unwrap();
    stdout.flush().unwrap();

    let paused = std::sync::atomic::AtomicBool::new(false);
    let shutdown = std::sync::atomic::AtomicBool::new(false);
    std::thread::scope(|s| {
        use std::sync::atomic::Ordering;
        let (sender, receiver) = channel();
        let sender_keys = sender.clone();

        // User input thread
        let paused_ref = &paused;
        let shutdown_ref = &shutdown;
        s.spawn(move || {
            for c in stdin.events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char('q')) => {
                        shutdown_ref.store(true, Ordering::Relaxed);
                        return;
                    }
                    Event::Key(Key::Char('c')) => sender_keys.send(Events::ClearMessage).unwrap(),
                    Event::Key(Key::Char(' ')) => {
                        paused_ref.fetch_xor(true, Ordering::Relaxed);
                    }
                    Event::Mouse(MouseEvent::Press(button, _, _)) => {
                        if button == termion::event::MouseButton::Left {
                            sender_keys.send(Events::Press).unwrap();
                        }
                    }
                    Event::Mouse(MouseEvent::Release(_, _)) => {
                        sender_keys.send(Events::Release).unwrap();
                    }
                    _ => {}
                }
            }
        });

        // Sample thread
        let shutdown_ref = &shutdown;
        s.spawn(move || loop {
            if shutdown_ref.load(Ordering::Relaxed) {
                return;
            }
            if !paused_ref.load(Ordering::Relaxed) {
                sender.send(Events::Tick).unwrap();
            }
            sleep(Duration::from_millis(16));
        });

        // Model thread
        let cloned_state = state.clone();
        s.spawn(move || {
            let mut pressed = false;
            let mut unpressed_ticks = 0;
            let mut pressed_ticks = 0;
            let mut partial_symbol = vec![];
            for e in receiver.iter() {
                match e {
                    Events::Tick => {
                        let mut state = cloned_state.write().unwrap();
                        state.buf.sample(pressed); 
                        if pressed {
                            pressed_ticks += 1;
                            unpressed_ticks = 0;
                        } else {
                            unpressed_ticks += 1;
                            if pressed_ticks > 0 {
                                partial_symbol.push(
                                    if pressed_ticks < 7 {
                                        crate::morse::MorseSymbol::Dit
                                    } else {
                                        crate::morse::MorseSymbol::Dah
                                    }
                                );
                            } else if unpressed_ticks > 10 {
                                let decoded = decode_symbols(&partial_symbol);
                                partial_symbol = vec![];
                                match decoded {
                                    None => (),
                                    Some(c) => state.message.push(c),
                                };
                            };
                            pressed_ticks = 0;
                        }
                    }
                    Events::Press => {
                        pressed = true;
                    }
                    Events::Release => {
                        pressed = false;
                    }
                    Events::Pause => {
                        let mut state = cloned_state.write().unwrap();
                        state.paused = true;
                    }
                    Events::ClearMessage => {
                        let mut state = cloned_state.write().unwrap();
                        state.message = String::new();
                    }
                    Events::Shutdown => {
                        return;
                    }
                }
            }
        });

        // View thread
        let shutdown_ref = &shutdown;
        s.spawn(|| {
            for x in 0..width {
                write!(stdout, "{}—", termion::cursor::Goto(x, (height / 2) - 1),).unwrap();
                write!(stdout, "{}—", termion::cursor::Goto(x, (height / 2) + 1),).unwrap();
            }
            loop {
                if shutdown_ref.load(Ordering::Relaxed) {
                    return;
                }
                let lock = state.read().unwrap();
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(width / 2 - 10, height / 2 - 10),
                    termion::clear::CurrentLine,
                )
                .unwrap();
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(width / 2 - 10, height / 2 - 10),
                    lock.message
                )
                .unwrap();

                for (x, sample) in lock.buf.iter().enumerate() {
                    let c = if sample { 'X' } else { '.' };
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Goto(x as u16 + 1, height / 2),
                        c
                    )
                    .unwrap();
                }
                drop(lock);
                sleep(Duration::from_millis(8));
            }
        });
    });
}
