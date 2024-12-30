use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind, Stylize},
    widgets::{Block, List, Widget},
};

pub struct App {
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::Wait,
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [graph_area, list_area, footer_area] = Layout::vertical([
            Constraint::Percentage(30),
            Constraint::Fill(1),
            Constraint::Max(1),
        ])
        .areas(area);

        Block::new().bg(tailwind::RED.c500).render(graph_area, buf);
        Block::new().bg(tailwind::GREEN.c500).render(list_area, buf);
        Block::new()
            .bg(tailwind::CYAN.c500)
            .render(footer_area, buf);
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Wait,
    Exit,
}
