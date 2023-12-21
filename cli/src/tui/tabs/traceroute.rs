use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

use crate::tui::{root::layout, colours::RgbSwatch, theme::THEME};

#[derive(Debug)]
pub struct TracerouteTab {
    selected_row: usize,
}

impl TracerouteTab {
    pub fn new(selected_row: usize) -> Self {
        Self {
            selected_row: selected_row,
        }
    }
}

impl Widget for TracerouteTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = area.inner(&Margin {
            vertical: 1,
            horizontal: 2,
        });
        Clear.render(area, buf);
        Block::new().style(THEME.content).render(area, buf);
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);
        let left_area = layout(area[0], Direction::Vertical, vec![0, 3]);
        render_map(self.selected_row, area[1], buf);
    }
}


fn render_map(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let theme = THEME.traceroute.map;
    ratatui::widgets::Block::default()
        .render(area, buf);
    // Canvas::default()
    //     .background_color(theme.background_color)
    //     .block(
    //         Block::new()
    //             .padding(Padding::new(1, 0, 1, 0))
    //             .style(theme.style),
    //     )
    //     .marker(Marker::HalfBlock)
    //     // picked to show Australia for the demo as it's the most interesting part of the map
    //     // (and the only part with hops ;))
    //     .x_bounds([112.0, 155.0])
    //     .y_bounds([-46.0, -11.0])
    //     .render(area, buf);
}

#[derive(Debug)]
struct Hop {
    host: &'static str,
    address: &'static str,
    location: (f64, f64),
}

impl Hop {
    const fn new(name: &'static str, address: &'static str, location: (f64, f64)) -> Self {
        Self {
            host: name,
            address,
            location,
        }
    }
}
