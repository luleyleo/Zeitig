use druid::{AppLauncher, WindowDesc};
use std::error::Error;

mod ui;
mod state;
mod widgets;
mod controller;

fn main() -> Result<(), Box<dyn Error>> {
    let window = WindowDesc::new(ui::tracker)
        .title("Zeitig")
        .window_size((300.0, 400.0));

    use state::backend::Backend;
    let mut backend = state::backend::Sqlite::new(state::paths::data_file())?;
    backend.setup()?;
    let content = backend.load_content()?;
    let history = backend.load_history(&content)?;
    backend.close()?;
    let state = state::AppState {
        content,
        history,
        setup: state::Setup::default(),
        active: None,
    };

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)?;

    Ok(())
}
