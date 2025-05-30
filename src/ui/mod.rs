use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table},
};
use tui_textarea::{Input, Key, TextArea};

use crate::{locales, ok::response};

trait Clear {
    fn clear(&mut self);
}

impl<'a> Clear for TextArea<'a> {
    fn clear(&mut self) {
        *self = TextArea::default();
    }
}

pub struct PositonTable<'a> {
    data: Vec<response::PositionResponse>,
    pub show_table: bool,
    pub input: TextArea<'a>,
    pub show_input: bool,
}

impl<'a> PositonTable<'a> {
    pub fn new(data: Vec<response::PositionResponse>) -> Self {
        let textarea = TextArea::default();
        // 初始化表格数据
        PositonTable {
            data,
            show_table: true,
            input: textarea,
            show_input: false,
        }
    }

    pub async fn handle_input<F>(&mut self, event: event::KeyEvent, cb: F) -> bool
    where
        F: AsyncFn(response::PositionResponse),
    {
        match self.show_input {
            false => {
                match event.code {
                    KeyCode::Char('q') => return true, // 按下 q 键退出
                    KeyCode::Char('c') => {
                        self.show_input = !self.show_input;
                    }
                    KeyCode::Esc => self.show_table = !self.show_table, // 老板键 按下 tEsc 隐藏/显示内容
                    _ => {}
                }
            }
            true if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc => self.show_input = !self.show_input,
                KeyCode::Enter => {
                    let lines = self.input.lines();
                    if lines.len() == 1 {
                        if let Ok(tid) = lines[0].trim().parse::<usize>() {
                            if tid < self.data.len() {
                                let position = self.data.get(tid).unwrap().clone();
                                cb(position).await;
                                self.show_input = !self.show_input;
                            }
                        }
                    }
                    self.input.clear();
                }
                _ => {
                    let input = match event.code {
                        KeyCode::Char(c) => Input {
                            key: Key::Char(c),
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Backspace => Input {
                            key: Key::Backspace,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Enter => Input {
                            key: Key::Enter,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Left => Input {
                            key: Key::Left,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Right => Input {
                            key: Key::Right,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Up => Input {
                            key: Key::Up,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Down => Input {
                            key: Key::Down,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Tab => Input {
                            key: Key::Tab,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        KeyCode::Delete => Input {
                            key: Key::Delete,
                            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
                            alt: event.modifiers.contains(KeyModifiers::ALT),
                            shift: event.modifiers.contains(KeyModifiers::SHIFT),
                        },
                        _ => Input::default(), // Ignore unmapped keys
                    };
                    self.input.input(input);
                }
            },
            true => {}
        }
        false
    }

    pub fn update(&mut self, data: Vec<response::PositionResponse>) {
        self.data = data;
    }

    pub fn draw(&self, frame: &mut Frame) {
        if !self.show_table {
            return;
        }

        let mut rows = vec![];

        for (index, item) in self.data.iter().enumerate() {
            let upl = item.upl_ratio.parse::<f64>().unwrap_or_default() * 100 as f64;

            let upl_color = if upl > 0.0 { Color::Green } else { Color::Red };

            let upl = Cell::new(format!(
                "{:.2} ({:.2}%)",
                item.upl.parse::<f64>().unwrap_or_default(),
                item.upl_ratio.parse::<f64>().unwrap_or_default() * 100 as f64
            ))
            .style(Style::default().fg(upl_color));
            let row = Row::new(vec![
                Cell::new(format!("{}", index,)),
                Cell::new(format!(
                    "{} {} {}x",
                    item.inst_id, item.pos_side, item.lever
                )),
                Cell::new(format!(
                    "{:.2}",
                    item.pos.parse::<f64>().unwrap_or_default() * 1000 as f64
                )),
                Cell::new(format!(
                    "{:.2}",
                    (item.margin.parse::<f64>().unwrap_or_default() * 100.0).floor() / 100.0
                )),
                Cell::new(format!(
                    "{:.5}",
                    item.avg_px.parse::<f64>().unwrap_or_default()
                )),
                upl,
                Cell::new(format!(
                    "{:.5}",
                    item.mark_px.parse::<f64>().unwrap_or_default()
                )),
                Cell::new(format!(
                    "{:.5}",
                    (item.liq_px.parse::<f64>().unwrap_or_default() * 100000.0).floor() / 100000.0
                )),
            ]);
            rows.push(row);
        }
        let widths = [Constraint::Length(5), Constraint::Length(5)];
        let table = Table::new(rows, widths)
            .header(
                Row::new(vec![
                    locales::ui().table.head.tid,
                    locales::ui().table.head.inst_id,
                    locales::ui().table.head.pos,
                    locales::ui().table.head.margin,
                    locales::ui().table.head.avg_px,
                    locales::ui().table.head.upl,
                    locales::ui().table.head.mark_px,
                    locales::ui().table.head.liq_px,
                ])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
            )
            .block(
                Block::default()
                    .title(locales::ui().table.title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .widths(&[
                Constraint::Length(5),
                Constraint::Length(25),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(20),
                Constraint::Length(15),
                Constraint::Length(15),
            ])
            .column_spacing(1);

        frame.render_widget(table, frame.area());

        if self.show_input {
            let block = Block::bordered().title(
                locales::ui()
                    .input
                    .close_position
                    .title
                    .replace("{0}", &locales::ui().table.head.tid),
            );

            let block_area = ratatui::layout::Rect::new(
                frame.area().width / 4,
                frame.area().height / 4,
                frame.area().width / 2,
                3,
            );
            frame.render_widget(block, block_area);

            let textbox_area = ratatui::layout::Rect::new(
                (frame.area().width / 4) + 1,
                (frame.area().height / 4) + 1,
                (frame.area().width / 2) - 2,
                3,
            );

            frame.render_widget(&self.input, textbox_area);
        }
    }
}
