use crate::app::{App, Focus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

const BG_DARK_BLUE: Color = Color::Rgb(8, 12, 24);

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    frame.render_widget(
        Block::default().style(Style::default().bg(BG_DARK_BLUE)),
        area,
    );

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(vertical[0]);

    draw_headings(frame, horizontal[0], app);
    draw_content(frame, horizontal[1], app);
    draw_status(frame, vertical[1], app);
}

fn draw_headings(frame: &mut Frame, area: Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Headings);
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan).bg(BG_DARK_BLUE)
    } else {
        Style::default().fg(Color::DarkGray).bg(BG_DARK_BLUE)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Headings ")
        .border_style(border_style)
        .style(Style::default().bg(BG_DARK_BLUE));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if app.doc.headings.is_empty() {
        let empty_msg = Paragraph::new("No headings found")
            .style(Style::default().fg(Color::DarkGray).bg(BG_DARK_BLUE));
        frame.render_widget(empty_msg, inner_area);
        return;
    }

    let mut lines = Vec::with_capacity(app.doc.headings.len());
    for (i, heading) in app.doc.headings.iter().enumerate() {
        let indent = "  ".repeat(heading.level.saturating_sub(1) as usize);
        let text = format!("{}{}", indent, heading.title);
        let is_selected = i == app.selected;

        let style = if is_selected {
            if is_focused {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            }
        } else {
            Style::default().fg(Color::White).bg(BG_DARK_BLUE)
        };

        lines.push(Line::from(Span::styled(text, style)));
    }

    let inner_height = inner_area.height as usize;
    let scroll_y = if inner_height > 0 && app.selected >= inner_height {
        (app.selected + 1 - inner_height) as u16
    } else {
        0
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(BG_DARK_BLUE))
        .scroll((scroll_y, 0));
    frame.render_widget(paragraph, inner_area);
}

fn draw_content(frame: &mut Frame, area: Rect, app: &App) {
    let is_focused = matches!(app.focus, Focus::Content);
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan).bg(BG_DARK_BLUE)
    } else {
        Style::default().fg(Color::DarkGray).bg(BG_DARK_BLUE)
    };

    let title = if let Some(h) = app.doc.headings.get(app.selected) {
        format!(" Content: {} ", h.title)
    } else {
        " Content ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(border_style)
        .style(Style::default().bg(BG_DARK_BLUE));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let raw_markdown = app.current_content().join("\n");
    let highlighted_text = tui_markdown::from_str(&raw_markdown);

    let paragraph = Paragraph::new(highlighted_text)
        .style(Style::default().bg(BG_DARK_BLUE))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));

    frame.render_widget(paragraph, inner_area);
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let count = app.doc.headings.len();
    let status_text = format!(
        " {} | {} heading{} | [j/k/↓/↑] Nav [Enter/l] Focus [Esc/h] Back [g/G] First/Last [PgUp/PgDn] Scroll [q] Quit",
        app.doc.path,
        count,
        if count == 1 { "" } else { "s" }
    );

    let paragraph =
        Paragraph::new(status_text).style(Style::default().fg(Color::Cyan).bg(BG_DARK_BLUE));

    frame.render_widget(paragraph, area);
}
