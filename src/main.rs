use druid::{AppLauncher, WindowDesc};

mod controller;
mod state;
mod ui;
mod widgets;

mod state2;

fn main() {
    let window = WindowDesc::new(ui::tracker)
        .title("Zeitig")
        .window_size((300.0, 400.0));

    let state = state::files::read_state();

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(state)
        .expect("Failed to launch Zeitig.");
}
