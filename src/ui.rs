use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    symbols::{
        border::{Set, ROUNDED},
        line::NORMAL,
    },
    widgets::{
        calendar::CalendarEventStore, calendar::Monthly, Block, BorderType, Borders, Clear, List,
        ListItem, Padding, Paragraph,
    },
    Frame,
};

use crate::app::App;

use time::OffsetDateTime;

pub fn render(app: &mut App, frame: &mut Frame) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(frame.size());

    let header_area = areas[0];
    let main_area = areas[1];

    let main_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(24)])
        .split(main_area);

    let content_area = main_areas[0];
    let sidebar_area = main_areas[1];
    let help_popup_area = centered_rect_length(32, 20, content_area);

    draw_header(frame, header_area);
    draw_calendar(frame, sidebar_area);

    if app.is_todo_empty() {
        draw_empty_list(frame, content_area);
    } else {
        draw_list(frame, app, content_area);
    }

    if app.is_help_visible() {
        draw_help_popup(frame, help_popup_area);
    }
}

fn draw_help_popup(f: &mut Frame, area: Rect) {
    let help_text = vec![
        "Toggle help - h",
        "Quit        - q, Ctrl-C, Esc",
        "Navigate    - j, k",
        "",
        "Add task    - o",
        "Delete task - Shift-D",
        "Change task - Shift-C",
        "Toggle task - x",
    ];

    let help = Paragraph::new(help_text.join("\n")).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::DarkGray))
            .title("Help")
            .title_style(Style::default().fg(Color::Rgb(100, 200, 228)))
            .padding(Padding::uniform(1)),
    );

    f.render_widget(Clear, area);
    f.render_widget(help, area);
}

fn centered_rect_length(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::new()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height - height) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height - height) / 2),
        ])
        .split(r);

    Layout::new()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width - width) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width - width) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn draw_calendar(f: &mut Frame, area: Rect) {
    let date = OffsetDateTime::now_utc().date();
    let calendar = Monthly::new(
        date,
        CalendarEventStore::today(Style::new().fg(Color::Rgb(100, 200, 228))),
    )
    .block(
        Block::default()
            .border_set(Set {
                top_left: NORMAL.horizontal_down,
                bottom_left: NORMAL.horizontal_up,
                ..ROUNDED
            })
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .show_surrounding(Style::new().fg(Color::DarkGray))
    .show_month_header(Style::new().add_modifier(Modifier::BOLD))
    .show_weekdays_header(Style::new().add_modifier(Modifier::ITALIC));

    f.render_widget(calendar, area);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let line: Line = vec![
        " ~todo-term🍰 "
            .fg(Color::White)
            .bg(Color::Rgb(20, 100, 128)),
        " @ D:/development/todo-term/target/release/todo.json".into(),
    ]
    .into();

    let header = Paragraph::new(line).style(Style::default().bg(Color::Rgb(10, 50, 64)));

    f.render_widget(header, area);
}

fn draw_list(f: &mut Frame, app: &mut App, area: Rect) {
    let incomplete_tasks = app
        .get_incomplete_tasks()
        .iter()
        .map(|i| ListItem::new(format!("[ ] {}", i)).style(Style::default().fg(Color::Rgb(200, 200, 200))))
        .collect::<Vec<_>>();

    let complete_tasks = app
        .get_complete_tasks()
        .iter()
        .map(|i| ListItem::new(format!("[x] {}", i)).style(Style::default().fg(Color::DarkGray)))
        .collect::<Vec<_>>();

    let all_tasks = incomplete_tasks
        .iter()
        .chain(complete_tasks.iter())
        .cloned()
        .collect::<Vec<_>>();

    let all_tasks_len = all_tasks.len();

    let list = List::new(all_tasks)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!(
                    "Tasks ({}/{})",
                    complete_tasks.len(),
                    all_tasks_len
                ))
                .title_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(60, 60, 60))
                .fg(Color::White),
        );

    f.render_stateful_widget(list, area, &mut app.get_todo_list_state());
}

fn draw_empty_list(f: &mut Frame, area: Rect) {
    let text = vec![
        "No tasks for today! 🎉",
        "",
        "Add one by pressing 'o'.",
        "",
        "Press 'h' to toggle help.",
    ];

    let paragraph = Paragraph::new(text.join("\n"))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!("Tasks (o_o)"))
                .title_style(Style::default().fg(Color::White))
                .padding(Padding::uniform(1)),
        )
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(paragraph, area);
}
