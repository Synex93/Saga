use crate::parser::xml::ParserResult;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};
use std::io;

struct App {
    state: TableState,
    rows: Vec<Vec<String>>,
    total: usize,
}

impl App {
    fn new(results: Vec<ParserResult>, total: usize) -> Self {
        let rows = results
            .into_iter()
            .filter_map(|r| {
                if let ParserResult::Authentication(d) = r {
                    Some(vec![
                        d.time
                            .replace('T', " ")
                            .split('.')
                            .next()
                            .unwrap_or("-")
                            .to_string(),
                        d.event_id.to_string(),
                        d.description,
                        d.subject_user_name,
                        d.target_user_name,
                        d.ip_address,
                        d.logon_type
                            .map(|t| match t {
                                2 => "交互式(2)".into(),
                                3 => "网络(3)".into(),
                                5 => "服务(5)".into(),
                                10 => "RDP(10)".into(),
                                n => format!("未知({})", n),
                            })
                            .unwrap_or("-".into()),
                        if d.status.is_empty() {
                            "-".into()
                        } else {
                            d.status
                        },
                    ])
                } else {
                    None
                }
            })
            .collect();

        let mut state = TableState::default();
        state.select(Some(0));

        App { state, rows, total }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1).min(self.rows.len().saturating_sub(1)),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn run_tui(data: Vec<ParserResult>, total: usize) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(data, total);

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(size);

            let header = vec![
                "时间",
                "事件ID",
                "描述",
                "主体用户",
                "目标用户",
                "来源IP",
                "登录类型",
                "状态码",
            ];

            let header_row = Row::new(header.iter().map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            }))
            .height(1);

            let rows: Vec<Row> = app
                .rows
                .iter()
                .map(|r| {
                    let color = match r[1].as_str() {
                        "4625" | "4740" => Color::Red,
                        "4672" => Color::Yellow,
                        "4634" | "4647" => Color::DarkGray,
                        _ => Color::Cyan,
                    };
                    Row::new(
                        r.iter()
                            .map(|c| Cell::from(c.as_str()).style(Style::default().fg(color))),
                    )
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Length(20),
                    Constraint::Length(8),
                    Constraint::Length(38),
                    Constraint::Length(20),
                    Constraint::Length(20),
                    Constraint::Length(16),
                    Constraint::Length(10),
                    Constraint::Length(12),
                ],
            )
            .header(header_row)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Saga - 认证事件 "),
            )
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("► ");

            f.render_stateful_widget(table, layout[0], &mut app.state);

            // 底部状态栏
            let status = ratatui::widgets::Paragraph::new(format!(
                " ↑↓/jk 滚动  q 退出  匹配：{}条  总扫描：{}条",
                app.rows.len(),
                app.total
            ))
            .style(Style::default().fg(Color::White).bg(Color::DarkGray));

            f.render_widget(status, layout[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::PageDown | KeyCode::Char('d') => {
                    for _ in 0..10 {
                        app.next();
                    }
                }
                KeyCode::PageUp | KeyCode::Char('u') => {
                    for _ in 0..10 {
                        app.previous();
                    }
                }
                KeyCode::Home | KeyCode::Char('g') => app.state.select(Some(0)),
                KeyCode::End | KeyCode::Char('G') => {
                    app.state.select(Some(app.rows.len().saturating_sub(1)));
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}
