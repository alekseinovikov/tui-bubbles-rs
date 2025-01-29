use app::App;

mod model;
mod app;
mod bubble;
mod sound;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let app = App::new(60, &terminal, 100.0, 1.0);
    app.run(terminal).await;

    ratatui::restore();
    Ok(())
}
