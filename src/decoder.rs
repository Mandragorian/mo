use std::io::Result;
use std::sync::{mpsc::channel, Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use layout::Offset;
use ratatui::widgets::{Borders, Sparkline};
use ratatui::{
    prelude::*,
    widgets::{List, Block, Paragraph},
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseEventKind};

use crate::morse::decode_symbols;
use crate::ring::RingBuffer;

#[derive(Debug, Default)]
pub struct Decoder {
    state: Arc<RwLock<State>>,
}

#[derive(Debug)]
enum Events {
    Press,
    Release,
    ClearMessage,
    Tick,
}

#[derive(Debug, Default)]
struct State {
    buf: RingBuffer,
    message: String,
}

impl Widget for &Decoder {

    fn render(self, area: Rect, buf: &mut Buffer)
        where Self: Sized 
    {
        let offset = Offset{
            x: area.x as i32,
            y: area.y as i32,
        };

        let list =List::new(["c to clear.", "<space> to pause.", "q to exit.", "Dah, dah, dit, dah!"]);
        let list_area = Rect::new(0, 0, 100, 10).offset(offset);
        Widget::render(list, list_area, buf);

        let lock: std::sync::RwLockReadGuard<State> = self.state.read().unwrap();
        let spark_area = Rect::new(0, 8, lock.buf.buf.len() as u16, 3).offset(offset);
        let data: Vec<_> = lock.buf.iter().map(|dp| if dp {1u64} else { 0u64}).collect();
        let text = lock.message.clone();
        drop(lock);

        let spark = Sparkline::default()
        .block(Block::new().borders(Borders::TOP | Borders::BOTTOM))
        .data(data.as_slice());
        Widget::render(spark, spark_area, buf);

        let text = Paragraph::new(text)
        .alignment(Alignment::Center);
        let offset = Offset{
            x: 0_i32,
            y: (area.height / 2) as i32,
        };
        let text_area = Rect::new(0, 0, area.width, 1).offset(offset);
        Widget::render(text, text_area, buf);

    }
}

impl Decoder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, terminal: &mut crate::tui::Tui) -> Result<()> {
        terminal.clear().expect("terminal coudln't be cleared");

        let width = terminal.size()?.width;
        self.state.write().unwrap().buf = RingBuffer::new(width as usize);

        let paused = std::sync::atomic::AtomicBool::new(false);
        let shutdown = std::sync::atomic::AtomicBool::new(false);
        std::thread::scope(|s| {
            use std::sync::atomic::Ordering;
            let (sender, receiver) = channel();
            let sender_keys = sender.clone();

            // Sample thread
            let paused_ref = &paused;
            let shutdown_ref = &shutdown;
            s.spawn(move || loop {
                if shutdown_ref.load(Ordering::Relaxed) {
                    return Ok::<(), std::io::Error>(());
                }
                if !paused_ref.load(Ordering::Relaxed) {
                    sender.send(Events::Tick).unwrap();
                }
                sleep(Duration::from_millis(16));
            });

            // Model thread
            let cloned_state = Arc::clone(&self.state);
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
                        Events::ClearMessage => {
                            let mut state = cloned_state.write().unwrap();
                            state.message = String::new();
                        }
                    }
                };
            });

            // View thread
            let shutdown_ref = &shutdown;
            s.spawn(|| {
                loop {
                    if shutdown_ref.load(Ordering::Relaxed) {
                        return Ok::<(), std::io::Error>(());
                    }
                    terminal.draw(|frame| self.render_frame(frame))?;
                    sleep(Duration::from_millis(8));
                }
            });
            
            loop {
                match event::read()? {
                    // it's important to check that the event is a key press event as
                    // crossterm also emits key release and repeat events on Windows.
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Char('q') => {
                                shutdown_ref.store(true, Ordering::Relaxed);
                                return Ok(());
                            }
                            KeyCode::Char('c') => sender_keys.send(Events::ClearMessage).unwrap(),
                            KeyCode::Char(' ') => {
                                paused_ref.fetch_xor(true, Ordering::Relaxed);
                            }
                            _ => {}
                        }
                    }
                    Event::Mouse(mouse_event) => {
                        match mouse_event.kind {
                            MouseEventKind::Down(event::MouseButton::Left) => {
                                sender_keys.send(Events::Press).unwrap();
                            }
                            MouseEventKind::Up(event::MouseButton::Left) => {
                                sender_keys.send(Events::Release).unwrap();   
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                };
            }
        })
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self,frame.size())
    }
}
