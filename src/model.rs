use crate::bubble::Bubble;
use crossterm::event::KeyCode;
use rand::Rng;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Layout, Rect, Size, Widget};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Circle};
use ratatui::{DefaultTerminal, Frame};
use std::collections::HashMap;

pub(crate) enum Message {
    KeyPressed(KeyCode),
    Quit,
}

pub(crate) enum RunningState {
    Running,
    Quiting,
}

pub(crate) struct Model {
    pub(crate) state: RunningState,
    bubbles: Vec<Bubble>,
    min_bubble_size: f64,
    max_bubble_size: f64,
    speed: f64,
    keys_layout: HashMap<KeyCode, (usize, usize)>,
    button_size: (u16, u16),
    keyboard_height: u16,
    empty_space_height: u16,
}

impl Model {
    pub(crate) fn new(terminal: &DefaultTerminal, max_size: f64, speed: f64) -> Self {
        let terminal_size = terminal.size().expect("Failed to get terminal size");
        let keys_layout = Self::create_keyboard_layout();
        let button_size = Self::calculate_button_size(&keys_layout, terminal_size);
        let keyboard_height = Self::calculate_keyboard_height(&keys_layout, button_size);
        let empty_space_height = terminal_size.height - keyboard_height;

        Model {
            state: RunningState::Running,
            bubbles: vec![],
            min_bubble_size: 0.0,
            max_bubble_size: max_size,
            speed,
            keys_layout,
            button_size,
            keyboard_height,
            empty_space_height,
        }
    }

    fn calculate_button_size(
        keys_layout: &HashMap<KeyCode, (usize, usize)>,
        terminal_size: Size,
    ) -> (u16, u16) {
        let (rows, columns) = keys_layout
            .values()
            .max()
            .map(|(row, col)| (row + 1, col + 1))
            .unwrap_or((1, 1));
        (
            terminal_size.width / columns as u16,
            terminal_size.height / rows as u16,
        )
    }

    fn calculate_keyboard_height(
        keys_layout: &HashMap<KeyCode, (usize, usize)>,
        button_size: (u16, u16),
    ) -> u16 {
        let max_row = keys_layout.values().map(|(row, _)| row).max().unwrap_or(&0);
        (*max_row + 1) as u16 * button_size.1
    }

    pub(crate) async fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => self.add_circle(&key),
            Message::Quit => self.state = RunningState::Quiting,
        }
    }

    fn add_circle(&mut self, key_code: &KeyCode) {
        if let Some(new_bubble) = self.create_bubble(key_code) {
            self.bubbles.push(new_bubble);
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let areas = Layout::vertical(vec![
            Constraint::Length(self.empty_space_height),
            Constraint::Length(self.keyboard_height),
            Constraint::Length(self.empty_space_height),
        ])
            .split(frame.area());

        let canvas = self.canvas(areas.get(1).unwrap());
        frame.render_widget(canvas, frame.area());
    }

    fn canvas(&mut self, rect: &Rect) -> impl Widget + '_ {
        let circles: Vec<Circle> = self
            .bubbles
            .iter_mut()
            .filter_map(|bubble| bubble.tick_and_return_circle())
            .collect();

        self.clean_finished_bubbles();

        Canvas::default()
            .marker(Marker::Dot)
            .paint(move |ctx| {
                for circle in &circles {
                    ctx.draw(circle);
                }
            })
            .x_bounds([0.0, rect.width as f64])
            .y_bounds([0.0, rect.height as f64])
    }

    fn create_bubble(&self, key_code: &KeyCode) -> Option<Bubble> {
        self.get_position(key_code).map(|(x, y)| {
            let color = self.get_color();
            Bubble::new(
                x,
                y,
                color,
                self.min_bubble_size,
                self.max_bubble_size,
                self.speed,
            )
        })
    }

    fn get_position(&self, key_code: &KeyCode) -> Option<(f64, f64)> {
        self.keys_layout.get(key_code).map(|(row, column)| {
            (
                *column as f64 * self.button_size.0 as f64 + self.button_size.0 as f64 / 2.0,
                *row as f64 * self.button_size.1 as f64 + self.button_size.1 as f64 / 2.0,
            )
        })
    }

    fn get_color(&self) -> Color {
        let mut rng = rand::rng();
        Color::Rgb(rng.random(), rng.random(), rng.random())
    }

    fn clean_finished_bubbles(&mut self) {
        self.bubbles.retain(|bubble| !bubble.finished());
    }

    fn create_keyboard_layout() -> HashMap<KeyCode, (usize, usize)> {
        const KEY_ROWS: [&[char]; 4] = [
            &['z', 'x', 'c', 'v', 'b', 'n', 'm'],
            &['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        ];

        let mut layout = HashMap::new();
        for (row_idx, row) in KEY_ROWS.iter().enumerate() {
            for (col_idx, key) in row.iter().enumerate() {
                layout.insert(KeyCode::Char(*key), (row_idx, col_idx));
            }
        }
        layout
    }
}