
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
        ListItem, Padding, Paragraph, HighlightSpacing,
    },
    Frame,
};

use crate::app::{App, AppMode};

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
    let footer_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(content_area)[1];

    draw_header(frame, header_area);
    draw_calendar(frame, sidebar_area);

    if app.is_todo_empty() {
        draw_empty_content(frame, content_area);
    } else {
        draw_list(frame, app, content_area);
    }

    draw_footer(frame, app, footer_area);

    if app.is_help_visible() {
        draw_help_popup(frame, help_popup_area);
    }
}

fn draw_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let footer_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Min(10)])
        .split(area);

    let footer_left = footer_areas[0];
    let footer_right = footer_areas[1];

    let action_display = Paragraph::new(app.get_action())
        .alignment(Alignment::Right)
        .style(Style::default());

    let mode_display = Paragraph::new(app.get_mode().to_string())
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .padding(Padding::horizontal(1))
        )
        .style(Style::default());

    f.render_widget(mode_display, footer_left);
    f.render_widget(action_display, footer_right);
}

fn draw_help_popup(f: &mut Frame, area: Rect) {
    let help_text = vec![
        "Toggle help    - h",
        "Quit           - q, Ctrl-C,",
        "                 Esc",
        "Exit insert    - Ctrl-C, End",
        "Navigate       - j, k, g, G",
        "Add task       - o, O",
        "Toggle task    - x",
        "Delete task    - d",
        "Change task    - c",
        "Append to task - a",
        "Yank task      - y",
        "Paste task     - p, P",
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
    if r.width < width || r.height < height {
        return r;
    }


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
    let selected = app.get_todo_list_state().selected().unwrap_or(usize::MAX);
    let app_mode = app.get_mode();

    let incomplete_tasks = app
        .get_incomplete_tasks()
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let bullet = "[ ] ".fg(Color::Rgb(200, 200, 200));
            let line = match (index == selected, app_mode) {
                // selected and insert mode
                (true, AppMode::Insert) => vec![
                    bullet,
                    task.clone().fg(Color::White).bg(Color::Rgb(60, 60, 60)),
                    "█".fg(Color::White),
                    " ".repeat(area.width as usize).bg(Color::Rgb(60, 60, 60)),
                ],
                // selected
                (true, _) => vec![
                    bullet.fg(Color::White),
                    task.clone().fg(Color::White).bg(Color::Rgb(60, 60, 60)),
                ],
                // not selected
                _ => vec![bullet, task.clone().fg(Color::Rgb(200, 200, 200))],
            };
            ListItem::new(Line::from(line))
        })
        .collect::<Vec<_>>();

    let incomplete_tasks_len = incomplete_tasks.len();

    let complete_tasks = app
        .get_complete_tasks()
        .iter()
        .enumerate()
        .map(|(index, task)| {
            let bullet = "[x] ".fg(Color::DarkGray);
            let line = match (index + incomplete_tasks_len == selected, app_mode) {
                // selected and insert mode
                (true, AppMode::Insert) => vec![
                    bullet,
                    task.clone().fg(Color::White).bg(Color::Rgb(60, 60, 60)),
                    "█".fg(Color::White),
                    " ".repeat(area.width as usize).bg(Color::Rgb(60, 60, 60)),
                ],
                // selected
                (true, _) => vec![
                    bullet.fg(Color::White),
                    task.clone().fg(Color::White).bg(Color::Rgb(60, 60, 60)),
                ],
                // not selected
                _ => vec![bullet, task.clone().fg(Color::DarkGray)],
            };
            ListItem::new(Line::from(line))
        })
        .collect::<Vec<_>>();

    let all_tasks = incomplete_tasks
        .iter()
        .chain(complete_tasks.iter())
        .cloned()
        .collect::<Vec<_>>();

    let all_tasks_len = all_tasks.len();

    let highlight_symbol = match app_mode {
        AppMode::Insert => ">>",
        _ => "> ",
    };

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
                .fg(Color::Rgb(100, 200, 228))
                .title_style(Style::default().fg(Color::White))
                .padding(Padding::horizontal(1)),
        )
        .highlight_symbol(highlight_symbol)
        .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(list, area, &mut app.get_todo_list_state());
}

// create a list of faces (3 characers wide)
const FACES: &[&str] = &[
    "(^-^)",
    "(·∀·)",
    "(≧ω≦)",
    "(^O^)",
    "(^Д^)",
    "(≧∇≦)",
    "(^_^)",
    "(^o^)",
    "(≧◡≦)"
];

fn get_random_face() -> String {
    // use pid to get a different face each time
    let pid = std::process::id() as usize;
    let index = pid % FACES.len();
    FACES[index].to_string()
}

fn draw_empty_content(f: &mut Frame, area: Rect) {
    let text = vec![
        "No tasks for today! 🎉",
        "",
        "Add one by pressing 'o'.",
        "",
        "Press 'h' to toggle help.",
        "",
        "Contributions are welcome!",
        "https://github.com/egegungordu/todo-term",
    ];

    let paragraph = Paragraph::new(text.join("\n"))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(format!("Tasks {}", get_random_face()))
                .title_style(Style::default().fg(Color::White))
                .padding(Padding::uniform(1)),
        )
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(paragraph, area);
}
