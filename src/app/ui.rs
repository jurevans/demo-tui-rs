use std::time::Duration;

use symbols::line;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, LineGauge, Paragraph, Row, Table};
use tui::{symbols, Frame};
use tui_logger::TuiLoggerWidget;

use super::actions::Actions;
use super::state::AppState;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    let is_size_valid = check_size(&size);

    if is_size_valid {
        // Vertical Layout Constraints
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(12),
                ]
                .as_ref(),
            )
            .split(size);

        // Title
        let title = draw_title();
        rect.render_widget(title, chunks[0]);

        // Body & Help
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
            .split(chunks[1]);

        let body = draw_body(app.is_loading(), app.state());
        rect.render_widget(body, body_chunks[0]);

        let help = draw_help(app.actions());
        rect.render_widget(help, body_chunks[1]);

        // Duration LineGauge
        if let Some(duration) = app.state().duration() {
            let duration_block = draw_duration(duration);
            rect.render_widget(duration_block, chunks[2]);
        }

        // Logging Display
        let logs = draw_logs();
        rect.render_widget(logs, chunks[3]);
    } else {
        // Error layout - Utilize full terminal
        let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(4)].as_ref())
                .split(size);

        let help = format!(
            "This app requires height >= 28, width = 52\nReceived height = {}, width = {}",
            &size.height,
            &size.width,
        );
        let error = draw_error(&help); 
        rect.render_widget(error, chunks[0]);
    }
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Test TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_error(error_msg: & str) -> Paragraph {
    Paragraph::new(error_msg)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red))
                .border_type(BorderType::Double),
        )
}

fn check_size(rect: &Rect) -> bool {
    !(rect.width < 52 || rect.height < 28)
}

fn draw_body<'a>(loading: bool, state: &AppState) -> Paragraph<'a> {
    let initialized_text = if state.is_initialized() {
        "App is initialized"
    } else {
        "App is not initialized !"
    };
    let loading_text = if loading { "Loading..." } else { "" };
    let sleep_text = if let Some(sleeps) = state.count_sleep() {
        format!("Sleep count: {}", sleeps)
    } else {
        String::default()
    };
    let tick_text = if let Some(ticks) = state.count_tick() {
        format!("Tick count: {}", ticks)
    } else {
        String::default()
    };
    Paragraph::new(vec![
        Spans::from(Span::raw(initialized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::raw(sleep_text)),
        Spans::from(Span::raw(tick_text)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

fn draw_duration(duration: &Duration) -> LineGauge {
    let sec = duration.as_secs();
    let label = format!("{}s", sec);
    let ratio = sec as f64 / 10.0;
    LineGauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sleep duration"),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .line_set(line::THICK)
        .label(label)
        .ratio(ratio)
}

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
}
