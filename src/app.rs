use crate::{model, sound};
use crate::model::RunningState;
use crossterm::event;
use model::{Message, Model};
use ratatui::DefaultTerminal;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub(crate) struct App {
    model: Arc<Mutex<Model>>,
    fps_duration: Duration,
}

impl App {
    pub(crate) fn new(fps: u8, terminal: &DefaultTerminal, max_size: f64, speed: f64) -> Self {
        let fps_duration = Duration::from_secs(1) / fps as u32;
        App {
            model: Arc::new(Mutex::new(Model::new(terminal, max_size, speed))),
            fps_duration,
        }
    }

    pub(crate) async fn run(&self, terminal: DefaultTerminal) {
        let model = self.model.clone();
        let fps_duration = self.fps_duration;

        tokio::spawn(Self::draw_state_loop(model, terminal, fps_duration));

        self.handle_key_events().await;
    }

    async fn handle_key_events(&self) {
        loop {
            if self.is_quitting().await {
                break;
            }

            let event = event::read().expect("Failed to read event");
            if let event::Event::Key(key_event) = event {
                if key_event.kind == event::KeyEventKind::Press {
                    self.handle_key_event(key_event).await;
                }
            }
        }
    }

    async fn handle_key_event(&self, key_event: event::KeyEvent) {
        let mut model = self.model.lock().await;

        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Char(ch),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } if ch == 'q' || ch == 'c' => {
                model.update(Message::Quit).await;
            }
            _ => {
                if let Some(frequency) = sound::get_frequency_for_key(&key_event.code) {
                    tokio::task::spawn_blocking(move || {
                        sound::generate_sound(frequency, 200);
                    });
                }
                model.update(Message::KeyPressed(key_event.code)).await;
            }
        }
    }

    async fn draw_state_loop(
        model: Arc<Mutex<Model>>,
        mut terminal: DefaultTerminal,
        fps_duration: Duration,
    ) {
        loop {
            tokio::time::sleep(fps_duration).await;

            let mut model = model.lock().await;
            terminal
                .draw(|frame| model.draw(frame))
                .expect("Failed to draw");
        }
    }

    async fn is_quitting(&self) -> bool {
        let model = self.model.lock().await;
        matches!(model.state, RunningState::Quiting)
    }
}