use std::collections::VecDeque;

use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Widget},
};

use crate::app::HISTORY_LEN;

pub struct ChartWrapper<'a, F: Fn(f64, usize) -> String> {
    data: Vec<Vec<(f64, f64)>>,
    style: Style,
    block: Option<Block<'a>>,
    label_generator: F,
}

impl<'a, F: Fn(f64, usize) -> String> ChartWrapper<'a, F> {
    pub fn new(data: &[VecDeque<f64>], label_generator: F) -> Self {
        let data = data
            .iter()
            .map(|cpu| {
                (0..HISTORY_LEN)
                    .map(|x| x as f64)
                    .zip(cpu.iter().copied())
                    .collect()
            })
            .collect();

        Self {
            data,
            style: Style::default(),
            block: None,
            label_generator,
        }
    }

    pub fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    pub fn block<'b>(self, block: Block<'b>) -> ChartWrapper<'b, F> {
        ChartWrapper {
            block: Some(block),
            ..self
        }
    }
}

impl<'a, F: Fn(f64, usize) -> String> Widget for ChartWrapper<'a, F> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let colors = [
            Color::Blue,
            Color::Cyan,
            Color::Green,
            Color::Magenta,
            Color::Red,
            Color::Yellow,
        ]
        .iter()
        .cycle();

        let datasets = self
            .data
            .iter()
            .zip(colors)
            .enumerate()
            .map(|(i, (data, &color))| {
                Dataset::default()
                    .data(data)
                    .graph_type(GraphType::Line)
                    .marker(Marker::Braille)
                    .name((self.label_generator)(data.last().unwrap().1, i))
                    .style(Style::default().fg(color))
            })
            .collect();

        let mut chart = Chart::new(datasets)
            .x_axis(Axis::default().bounds([0.0, HISTORY_LEN as f64]))
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(vec![
                        Span::raw(""),
                        Span::raw("20"),
                        Span::raw("40"),
                        Span::raw("60"),
                        Span::raw("80"),
                        Span::raw("100"),
                    ])
                    .labels_alignment(Alignment::Right),
            )
            .hidden_legend_constraints((Constraint::Percentage(75), Constraint::Percentage(75)))
            .style(self.style);

        if let Some(block) = self.block {
            chart = chart.block(block);
        }

        chart.render(area, buf);
    }
}