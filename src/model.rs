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
    pub(crate) fn new(terminal: &DefaultTerminal, max_size: f64, speed: f64) -> Model {
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
            .map(|(rows, columns)| (rows + 1, columns + 1))
            .unwrap();
        let height = terminal_size.height / rows as u16;
        let width = terminal_size.width / columns as u16;

        (width, height)
    }

    fn calculate_keyboard_height(
        keys_layout: &HashMap<KeyCode, (usize, usize)>,
        button_size: (u16, u16),
    ) -> u16 {
        let keyboard_height = keys_layout.values().map(|(row, _)| row).max().unwrap() + 1;
        let button_height = button_size.1;
        keyboard_height as u16 * button_height
    }

    pub(crate) async fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => {
                self.add_circle(&key);
            }
            Message::Quit => {
                self.state = RunningState::Quiting;
            }
        }
    }

    fn add_circle(&mut self, key_code: &KeyCode) {
        if let Some(new_bubble) = self.create_bubble(key_code) {
            self.bubbles.push(new_bubble);
        }
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let areas = Layout::vertical(vec![
            Constraint::Length(self.empty_space_height as u16),
            Constraint::Length(self.keyboard_height as u16),
            Constraint::Length(self.empty_space_height as u16),
        ])
        .split(frame.area());

        let canvas = self.canvas(areas.get(1).unwrap());
        frame.render_widget(canvas, frame.area());
    }

    fn canvas(&mut self, rect: &Rect) -> impl Widget + '_ {
        let circles: Vec<Circle> = self
            .bubbles
            .iter_mut()
            .map(|bubble| bubble.tick_and_return_circle())
            .filter(|circle| circle.is_some())
            .map(|circle| circle.unwrap())
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
            let x = *column as f64 * self.button_size.0 as f64 + self.button_size.0 as f64 / 2.0;
            let y = *row as f64 * self.button_size.1 as f64 + self.button_size.1 as f64 / 2.0;
            (x, y)
        })
    }

    fn get_color(&self) -> Color {
        let mut rng = rand::rng();
        let r = rng.random::<u8>();
        let g = rng.random::<u8>();
        let b = rng.random::<u8>();
        Color::Rgb(r, g, b)
    }

    fn clean_finished_bubbles(&mut self) {
        self.bubbles.retain(|bubble| !bubble.finished());
    }

    fn create_keyboard_layout() -> HashMap<KeyCode, (usize, usize)> {
        let mut layout = HashMap::new();

        // Bottom line (Z, X, C, V, B, N, M)
        layout.insert(KeyCode::Char('z'), (0, 0));
        layout.insert(KeyCode::Char('x'), (0, 1));
        layout.insert(KeyCode::Char('c'), (0, 2));
        layout.insert(KeyCode::Char('v'), (0, 3));
        layout.insert(KeyCode::Char('b'), (0, 4));
        layout.insert(KeyCode::Char('n'), (0, 5));
        layout.insert(KeyCode::Char('m'), (0, 6));

        // Third line (A, S, D, F, G, H, J, K, L)
        layout.insert(KeyCode::Char('a'), (1, 0));
        layout.insert(KeyCode::Char('s'), (1, 1));
        layout.insert(KeyCode::Char('d'), (1, 2));
        layout.insert(KeyCode::Char('f'), (1, 3));
        layout.insert(KeyCode::Char('g'), (1, 4));
        layout.insert(KeyCode::Char('h'), (1, 5));
        layout.insert(KeyCode::Char('j'), (1, 6));
        layout.insert(KeyCode::Char('k'), (1, 7));
        layout.insert(KeyCode::Char('l'), (1, 8));

        // Second line (Q, W, E, R, T, Y, U, I, O, P)
        layout.insert(KeyCode::Char('q'), (2, 0));
        layout.insert(KeyCode::Char('w'), (2, 1));
        layout.insert(KeyCode::Char('e'), (2, 2));
        layout.insert(KeyCode::Char('r'), (2, 3));
        layout.insert(KeyCode::Char('t'), (2, 4));
        layout.insert(KeyCode::Char('y'), (2, 5));
        layout.insert(KeyCode::Char('u'), (2, 6));
        layout.insert(KeyCode::Char('i'), (2, 7));
        layout.insert(KeyCode::Char('o'), (2, 8));
        layout.insert(KeyCode::Char('p'), (2, 9));

        // Top line (1, 2, 3, 4, 5, 6, 7, 8, 9, 0)
        layout.insert(KeyCode::Char('1'), (3, 0));
        layout.insert(KeyCode::Char('2'), (3, 1));
        layout.insert(KeyCode::Char('3'), (3, 2));
        layout.insert(KeyCode::Char('4'), (3, 3));
        layout.insert(KeyCode::Char('5'), (3, 4));
        layout.insert(KeyCode::Char('6'), (3, 5));
        layout.insert(KeyCode::Char('7'), (3, 6));
        layout.insert(KeyCode::Char('8'), (3, 7));
        layout.insert(KeyCode::Char('9'), (3, 8));
        layout.insert(KeyCode::Char('0'), (3, 9));

        layout
    }
}
