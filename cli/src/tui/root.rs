use std::rc::Rc;
use itertools::Itertools;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Tabs, Widget};
use crate::tui::app::{AppContext};
use crate::tui::theme::THEME;
use crate::tui::tabs::*;

pub struct Root<'a> {
    context: &'a mut AppContext
}

impl<'a> Root<'a> {
    pub fn new(context: &'a mut AppContext) -> Self {
        Root { context }
    }
}

impl Widget for Root<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        Block::new().style(THEME.root).render(area, buf);
        let area = layout(area, Direction::Vertical, vec![1, 0, 1]);
        self.render_title_bar(area[0], buf);
        self.render_selected_tab(area[1], buf);
        self.render_bottom_bar(area[2], buf);
    }
}

impl Root<'_> {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let area = layout(area, Direction::Horizontal, vec![0, 48]);

        Paragraph::new(Span::styled("kaf9s", THEME.app_title)).render(area[0], buf);
        let titles = vec![" (1) Topics ", " (2) Consumers ", " (3) Consumer Groups "];
        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.context.tab_index)
            .padding("", "")
            .divider("")
            .render(area[1], buf);
    }

    fn render_selected_tab(&mut self, area: Rect, buf: &mut Buffer) {
        let row_index = self.context.row_index;
        match self.context.tab_index {
            0 => RecipeTab::new(row_index).render(area, buf),
            1 => EmailTab::new(row_index).render(area, buf),
            2 => TracerouteTab::new(row_index).render(area, buf),
            _ => {
                self.context.row_index = 0;
                self.context.tab_index = 0;
                AboutTab::new(row_index).render(area, buf)
            },
        };
    }

    fn render_bottom_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [
            ("Q/Esc", "Quit"),
            ("Tab", "Next Tab"),
            ("↑/k", "Up"),
            ("↓/j", "Down"),
        ];
        let spans = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", key), THEME.key_binding.key);
                let desc = Span::styled(format!(" {} ", desc), THEME.key_binding.description);
                [key, desc]
            })
            .collect_vec();
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .fg(Color::Indexed(236))
            .bg(Color::Indexed(232))
            .render(area, buf);
    }
}

/// simple helper method to split an area into multiple sub-areas
pub fn layout(area: Rect, direction: Direction, heights: Vec<u16>) -> Rc<[Rect]> {
    let constraints = heights
        .iter()
        .map(|&h| {
            if h > 0 {
                Constraint::Length(h)
            } else {
                Constraint::Min(0)
            }
        })
        .collect_vec();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}