use ratatui::prelude::Color;
use ratatui::widgets::canvas::Circle;

enum BubbleState {
    Animating,
    Finished,
}

pub(crate) struct Bubble {
    state: BubbleState,
    circle: Circle,
    size_max: f64,
    speed: f64,
}

impl Bubble {
    pub fn new(x: f64, y: f64, color: Color, min_size: f64, max_size: f64, speed: f64) -> Self {
        let circle = Circle {
            x,
            y,
            radius: min_size,
            color,
        };

        Self {
            circle,
            state: BubbleState::Animating,
            size_max: max_size,
            speed,
        }
    }

    pub fn finished(&self) -> bool {
        match self.state {
            BubbleState::Finished => true,
            _ => false,
        }
    }

    pub fn tick_and_return_circle(&mut self) -> Option<Circle> {
        if self.finished() {
            return None;
        }

        let old_circle = &self.circle;

        let new_radius = old_circle.radius + self.speed;
        let new_circle = Circle {
            radius: new_radius,
            ..*old_circle
        };

        if new_circle.radius > self.size_max {
            self.state = BubbleState::Finished;
        }

        self.circle = new_circle;
        Some(self.circle.clone())
    }
}
