use druid::{AppLauncher, WindowDesc};
use std::error::Error;

mod controller;
// TODO: Remove old state module
mod state;
mod ui;
mod widgets;

mod state2;

fn main() -> Result<(), Box<dyn Error>> {
    let window = WindowDesc::new(ui::tracker)
        .title("Zeitig")
        .window_size((300.0, 400.0));

    use state2::backend::Backend;
    let mut backend = state2::backend::Sqlite::new(state::files::data_file_path())?;
    backend.setup()?;
    let content = backend.load_content()?;
    let history = backend.load_history(&content)?;
    backend.close()?;
    let state = state2::AppState {
        content,
        history,
        setup: state2::Setup::default(),
        active: None,
    };

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)?;

    Ok(())
}
