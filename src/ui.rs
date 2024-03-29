use std::rc::Rc;

use tui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

use crate::app::App;

use self::{chart_wrapper::ChartWrapper, cpus_bars::CpusBars, disks::Disks, processes::Processes};

mod chart_wrapper;
mod cpus_bars;
mod disks;
pub mod processes;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let mut block_style = Style::default().fg(*app.config.theme.widget.frame_color);
    block_style.bg = app.config.theme.widget.background_color.map(|c| c.0);

    let mut title_style = Style::default().fg(*app.config.theme.widget.title_color);
    title_style.bg = app.config.theme.widget.background_color.map(|c| c.0);

    let block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let layout = Layout::default()
        .margin(0)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(frame.size());

    let cpus = split_cpus(layout[0], app.cpu_history.len());

    let mem_and_disks = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2); 2])
        .split(layout[1]);

    frame.render_widget(
        ChartWrapper::new(
            &app.cpu_history,
            Box::new(|percentage, i| format!("cpu{i}: {percentage:.1}%")),
            [0.0, 100.0],
            &app.config,
        )
        .style(block_style)
        .block(block.clone().title(Line::styled("cpu", title_style)))
        .label_suffix('%'),
        cpus[0],
    );

    frame.render_widget(
        CpusBars::new(app)
            .style(block_style)
            .block(block.clone().title(Line::styled("cpu", title_style))),
        cpus[1],
    );

    frame.render_widget(
        ChartWrapper::new(
            &[app.mem_history.clone()],
            Box::new(|used_mem, _| format!("used mem: {used_mem:.1}{}", app.mem_prefix.prefix())),
            [0.0, app.mem_total],
            &app.config,
        )
        .style(block_style)
        .block(block.clone().title(Line::styled("mem", title_style)))
        .label_suffix(app.mem_prefix.prefix()),
        mem_and_disks[0],
    );

    frame.render_widget(
        Disks::new(app)
            .block(block.clone().title(Line::styled("disks", title_style)))
            .style(block_style),
        mem_and_disks[1],
    );

    frame.render_widget(
        Processes::new(app)
            .block(block.title(Line::styled("procs", title_style)))
            .style(block_style),
        layout[2],
    )
}

fn split_cpus(area: Rect, _cpus: usize) -> Rc<[Rect]> {
    Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area)
}
