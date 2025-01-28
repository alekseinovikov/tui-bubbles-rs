use app::App;

mod model;
mod app;
mod bubble;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    let app = App::new(60, &terminal);
    app.run(terminal).await;

    ratatui::restore();
    Ok(())
}
