use crate::bubble::Bubble;
use crossterm::event::KeyCode;
use ratatui::prelude::{Color, Rect, Widget};
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Circle};
use ratatui::Frame;

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
}

impl Model {
    pub(crate) fn new() -> Model {
        Model {
            state: RunningState::Running,
            bubbles: vec![],
            min_bubble_size: 0.0,
            max_bubble_size: 10.0,
            speed: 1.0,
        }
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
        let new_bubble = self.create_bubble(key_code);
        self.bubbles.push(new_bubble);
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let canvas = self.canvas(&frame.area());
        frame.render_widget(canvas, frame.area());
    }

    fn canvas(&mut self, rect: &Rect) -> impl Widget + '_ {
        let circles: Vec<Circle> = self.bubbles.iter_mut()
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

    fn create_bubble(&self, key_code: &KeyCode) -> Bubble {
        let (x, y) = self.get_position(key_code);
        let color = self.get_color(key_code);
        Bubble::new(
            x,
            y,
            color,
            self.min_bubble_size,
            self.max_bubble_size,
            self.speed,
        )
    }

    fn get_position(&self, key_code: &KeyCode) -> (f64, f64) {
        let x = 30.0;
        let y = 30.0;

        return (x, y);
    }

    fn get_color(&self, key_code: &KeyCode) -> Color {
        match key_code {
            KeyCode::Char('r') => Color::Red,
            KeyCode::Char('g') => Color::Green,
            KeyCode::Char('b') => Color::Blue,
            _ => Color::White,
        }
    }

    fn clean_finished_bubbles(&mut self) {
        self.bubbles.retain(|bubble| !bubble.finished());
    }
}
