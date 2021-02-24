use std::error::Error;
use std::io;
use std::path::PathBuf;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::{Frame, Terminal};
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Corner, Direction, Layout, Rect};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use unicode_width::UnicodeWidthStr;

use log::info;

use sundbg::Debugger;

use crate::event_bus::{Bus, Event};
use crate::logger::init_logging;

const DEFAULT_HIST_FILE: &'static str = ".sdbg_history";

type AppResult = Result<(), Box<dyn Error>>;

pub(crate) struct App {
    input: String,
    output: Vec<String>,
    history: Vec<String>,
    history_file: String,
    debugger: Debugger,
}

impl App {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            debugger: Debugger::start(path),
            input: String::new(),
            output: Vec::new(),
            history: Vec::new(),
            history_file: String::from(DEFAULT_HIST_FILE),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        init_logging().unwrap();

        let bus = Bus::instance();

        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(75),
                            Constraint::Percentage(25),
                        ].as_ref(),
                    )
                    .split(f.size());

                self.render_output_box(&chunks[0], f);
                self.render_input_box(&chunks[1], f);
            })?;

            loop {
                match bus.try_next() {
                    None => break,
                    Some(event) => {
                        if let Event::Input(key) = event {
                            if key == Key::Ctrl('c') {
                                break;
                            }
                        } else {
                            self.handle_event(event)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn render_output_box<B: Backend>(&mut self, chunk: &Rect, f: &mut Frame<B>) {
        let output_lines = Text::raw(self.output.join("\n"));
        let output = Paragraph::new(output_lines)
            .block(Block::default().borders(Borders::ALL).title("Output"));
        f.render_widget(output, *chunk);
    }

    fn render_input_box<B: Backend>(&mut self, chunk: &Rect, f: &mut Frame<B>) {
        let mut commands: Vec<ListItem> = self.history.iter()
            .rev()
            .map(|cmd| {
                let content = vec![Spans::from(Span::raw(format!("sdbg>> {}", cmd)))];
                ListItem::new(content)
            })
            .collect();
        commands.insert(0, ListItem::new(vec![Spans::from(Span::raw(format!("sdbg>> {}", self.input)))]));

        f.set_cursor(
            chunk.x + self.input.width() as u16 + 1 + "sdbg>> ".width() as u16,
            chunk.y + chunk.height - 2,
        );

        let messages = List::new(commands)
            .start_corner(Corner::BottomLeft)
            .block(Block::default().borders(Borders::ALL).title("Commands"));
        f.render_widget(messages, *chunk);
    }

    fn handle_event(&mut self, event: Event) -> AppResult {
        match event {
            Event::Input(key) => {
                match key {
                    Key::Char('\n') => {
                        let command: String = self.input.drain(..).collect();
                        self.handle_command(command)?;
                    }
                    Key::Ctrl('u') => { self.input.clear(); }
                    Key::Char(c) => { self.input.push(c); }
                    Key::Backspace => { self.input.pop(); }
                    Key::Up => {}
                    Key::Down => {}
                    _ => {}
                }
            }
            Event::Message(msg) => { self.output.push(msg); }
            Event::Tick => {}
        }
        Ok(())
    }

    fn handle_command(&mut self, command: String) -> AppResult {
        self.history.push(command.clone());
        self.output.push(command);
        Ok(())
    }
}
