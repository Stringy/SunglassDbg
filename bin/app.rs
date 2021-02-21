use std::io;
use termion::screen::AlternateScreen;
use tui::backend::{TermionBackend, Backend};
use tui::{Terminal, Frame};
use termion::raw::IntoRawMode;
use termion::input::MouseTerminal;
use crate::event::{Event, Events};
use termion::event::Key;
use std::error::Error;
use tui::widgets::{ListItem, List, Block, Borders};
use tui::text::{Span, Spans};
use tui::layout::{Layout, Direction, Constraint, Corner, Rect};
use unicode_width::UnicodeWidthStr;

const DEFAULT_HIST_FILE: &'static str = ".sdbg_history";

type AppResult = Result<(), Box<dyn Error>>;

pub(crate) struct App {
    input: String,
    output: Vec<String>,
    history: Vec<String>,
    history_file: String,
}

impl App {
    pub fn new() -> Self {
        Self {
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

        let events = Events::new();

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


            let event = events.next()?;
            if let Event::Input(key) = event {
                if key == Key::Ctrl('c') {
                    break;
                }
            }

            self.handle_input(event)?;
        }

        Ok(())
    }

    fn render_output_box<B: Backend>(&mut self, chunk: &Rect, f: &mut Frame<B>) {
        let output_lines: Vec<ListItem> = self.output.iter()
            .map(|line| {
                let content = vec![Spans::from(Span::raw(line))];
                ListItem::new(content)
            }).collect();
        let output = List::new(output_lines)
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

    fn handle_input(&mut self, event: Event<Key>) -> AppResult {
        if let Event::Input(input) = event {
            match input {
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
        Ok(())
    }

    fn handle_command(&mut self, command: String) -> AppResult {
        self.history.push(command.clone());
        self.output.push(command);
        Ok(())
    }
}
