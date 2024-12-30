#![allow(unused)]
use std::{io::stdout, path::PathBuf};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    widgets::{
        block::Title, Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph,
        ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
        Wrap,
    },
    Terminal,
};

use crate::{cmd, conf, error::*};

// # opts
// color
// TODO: make color theme from file
const DEFAULT_BG: Color = tailwind::ZINC.c800;
const DEFAULT_FG: Color = tailwind::YELLOW.c400;
const TITLE_FG: Color = tailwind::WHITE;
const SELECTED_BG: Color = tailwind::GRAY.c600;
const BORDER_BG: Color = tailwind::GREEN.c400;

// size
const LIST_BOX_SIZE: u16 = 20;
const NOTE_BOX_SIZE: u16 = 100 - LIST_BOX_SIZE;

const HELP_TEXT_LIST: &'static str = "List Mode:
________________________________________________
? -> open this menu
l -> go to note mode
j -> go down in list
k -> go up in list
q -> quit
<esc> -> quit
";
const HELP_TEXT_NOTE: &'static str = "Note Mode:
________________________________________________
? -> open this menu";

#[derive(Default, Debug, PartialEq, Eq)]
enum State {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum View {
    #[default]
    Main,
    Note,
    Help,
}

fn get_notes() -> Vec<String> {
    let path = conf::get_default_notes_dir();
    std::fs::read_dir(path)
        .unwrap()
        .filter(|p| !p.as_ref().unwrap().path().is_dir())
        .map(|file| file.unwrap().path().to_str().unwrap().to_string())
        .collect()
}

#[derive(Debug)]
pub struct App {
    notes: Vec<String>,
    index: usize,
    state: State,
    view: View,
    size: (u16, u16),

    allow_line_wrap: bool,
    note_scroll: u16,
}

impl Default for App {
    fn default() -> Self {
        let notes = get_notes();
        let first_note_lines = &notes.get(0).unwrap().lines().count();
        Self {
            notes,
            index: 0,
            state: State::default(),
            view: View::default(),
            size: ratatui::crossterm::terminal::size().unwrap(),
            allow_line_wrap: true,

            note_scroll: 0,
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.view {
            View::Main | View::Note => {
                let [header_area, rest_area, footer_area] = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ])
                .areas(area);

                let [title_box, note_box] = Layout::horizontal([
                    Constraint::Percentage(LIST_BOX_SIZE),
                    Constraint::Percentage(NOTE_BOX_SIZE),
                ])
                .areas(rest_area);

                // title
                Paragraph::new("notes")
                    .fg(TITLE_FG)
                    .bold()
                    .centered()
                    .render(header_area, buf);

                // notes
                self.render_list(title_box, buf);

                Clear.render(note_box, buf);

                self.render_note(note_box, buf);

                // footer
                Paragraph::new(format!("Mode: {:?}", self.view))
                    .alignment(ratatui::layout::Alignment::Left)
                    .render(footer_area, buf)
            }
            View::Help => {
                let [_, title, help_box, footer] = Layout::vertical([
                    Constraint::Percentage(20),
                    Constraint::Length(2),
                    Constraint::Min(0),
                    Constraint::Length(2),
                ])
                .areas(area);
                let [_, list_box, note_box, _] = Layout::horizontal([
                    Constraint::Percentage(10),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                    Constraint::Percentage(10),
                ])
                .areas(help_box);

                Paragraph::new("nota help")
                    .fg(TITLE_FG)
                    .bold()
                    .centered()
                    .render(title, buf);

                Paragraph::new(HELP_TEXT_LIST).render(list_box, buf);
                Paragraph::new(HELP_TEXT_NOTE).render(note_box, buf);

                Paragraph::new("press \'?\' to go back")
                    .centered()
                    .render(footer, buf);
            }
        }
    }
}

impl App {
    pub fn render_list(&mut self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let outer_block = Block::new()
            .borders(Borders::NONE)
            .fg(DEFAULT_FG)
            .bg(DEFAULT_BG);
        let inner_block = Block::new()
            .borders(Borders::TOP)
            .border_style(BORDER_BG)
            .bg(DEFAULT_BG);

        let outer_area = area;
        let inner_area = outer_block.inner(outer_area);

        outer_block.render(area, buf);

        let items = List::new(self.notes.iter().enumerate().map(|(i, note_name)| {
            let line = format!("{:02} {:}", i, note_name.split('/').last().unwrap());
            if self.index == i {
                ListItem::new(line).bg(SELECTED_BG)
            } else {
                ListItem::new(line)
            }
        }))
        .block(inner_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
                .fg(DEFAULT_FG),
        )
        .highlight_symbol(">")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        StatefulWidget::render(items, area, buf, &mut ListState::default());
    }

    pub fn render_note(&mut self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let outer_block = Block::new()
            .borders(Borders::ALL)
            .border_style(BORDER_BG)
            .fg(DEFAULT_FG)
            .bg(DEFAULT_BG);
        let inner_block = Block::new()
            //.borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .bg(DEFAULT_BG);

        let inner_area = outer_block.inner(area);

        outer_block.render(area, buf);

        let selected_note = self.notes.get(self.index).unwrap();
        let text = std::fs::read_to_string(selected_note).unwrap();
        let text = text.trim_end();

        Paragraph::new(text)
            .fg(DEFAULT_FG)
            .block(inner_block)
            .scroll((self.note_scroll, 0))
            .wrap(Wrap {
                trim: self.allow_line_wrap,
            })
            .render(inner_area, buf);

        //crate::log!(text.lines().count());
        let n = text.lines().count() as usize;
        let mut scrollbar_state = if n > 8 {
            ScrollbarState::new(n).position(self.note_scroll as usize)
        } else {
            ScrollbarState::new(n).position(self.note_scroll as usize)
        };

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(tailwind::GRAY.c600)
            .render(inner_area, buf, &mut scrollbar_state);
    }
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>, opts: conf::Opts) -> Result<()> {
        while self.state == State::Running {
            self.draw(terminal)?;
            self.handle(&opts)?;
        }
        Ok(())
    }
    fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(Clear, frame.area());
            frame.render_widget(self, frame.area());
        })?;
        Ok(())
    }
    fn handle(&mut self, opts: &conf::Opts) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // TODO: integgrate with custom config
                match self.view {
                    View::Main => match key.code {
                        KeyCode::Char('q') => self.state = State::Quitting,
                        KeyCode::Char('k') => self.dec_index(1),
                        KeyCode::Char('j') => self.inc_index(1),
                        KeyCode::Char('?') => self.view = View::Help,
                        KeyCode::Char('l') => {
                            self.view = View::Note;
                        }
                        KeyCode::Char('D') => println!("{self:?}"),
                        _ => (),
                    },

                    View::Note => match key.code {
                        KeyCode::Char('q') => self.state = State::Quitting,
                        KeyCode::Char('h') => self.view = View::Main,
                        KeyCode::Char('k') => {
                            if self.note_scroll > 0 {
                                self.note_scroll -= 1;
                                //self.scrollbar_state.scroll(ScrollDirection::Backward)
                            }
                        }
                        KeyCode::Char('j') => {
                            self.note_scroll += 1;
                            //self.scrollbar_state.scroll(ScrollDirection::Forward)
                        }
                        _ => (),
                    },

                    View::Help => match key.code {
                        KeyCode::Char('q') => self.state = State::Quitting,
                        KeyCode::Char('?') => self.view = View::default(),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        Ok(())
    }
    fn dec_index(&mut self, n: usize) {
        if self.index == 0 {
            self.index = self.notes.len() - 1;
        } else {
            self.index -= 1
        }
        self.note_scroll = 0;
    }
    fn inc_index(&mut self, n: usize) {
        self.index = (self.index + n) % self.notes.len();
        self.note_scroll = 0;
    }
}

pub fn init_error_hooks() -> Result<()> {
    //let (panic, error) = color_w
    Ok(())
}

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout()))?)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
