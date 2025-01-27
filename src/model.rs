use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::prelude::{Color, Layout, Rect, Widget};
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::canvas::{Canvas, Circle, Map, MapResolution};

pub(crate) enum Message {
    KeyPressed(KeyCode),
    Quit
}

pub(crate) enum RunningState {
    Running,
    Quiting
}

pub(crate) struct Model {
    pub(crate) state: RunningState,
    circles: Vec<Circle>
}

impl Model {
    pub(crate) fn new() -> Model {
        Model {
            state: RunningState::Running,
            circles: vec![]
        }
    }

    pub(crate) async fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => {
                self.add_circle(key);
            }
            Message::Quit => {
                self.state = RunningState::Quiting;
            }
        }
    }

    fn add_circle(&mut self, key_code: KeyCode) {
        let color = match key_code {
            KeyCode::Char('r') => Color::Red,
            KeyCode::Char('g') => Color::Green,
            KeyCode::Char('b') => Color::Blue,
            _ => Color::White
        };

        let circle = Circle{
            x: 30.0,
            y: 30.0,
            radius: 5.0,
            color
        };

        self.circles.push(circle);
    }

    pub(crate) fn draw(&self, frame: &mut Frame) {
        let canvas = self.canvas(&frame.area());
        frame.render_widget(canvas, frame.area());
    }

    fn canvas(&self, rect: &Rect) -> impl Widget + '_  {
        Canvas::default()
            .marker(Marker::Dot)
            .paint(move |ctx| {
                for circle in &self.circles {
                    ctx.draw(circle);
                }
            })
            .x_bounds([0.0, rect.width as f64])
            .y_bounds([0.0, rect.height as f64])
    }
}