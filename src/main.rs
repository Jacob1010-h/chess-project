pub mod app;
pub mod event;
pub mod game;
pub mod pieces;
pub mod ui;

use crate::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
