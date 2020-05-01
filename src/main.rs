use druid::{WindowDesc, Lens, Data, AppLauncher, widget::Button, Widget};

#[derive(Clone, Data, Lens)]
struct AppState {}

fn main() {
    let window = WindowDesc::new(ui).title("Zeitig");

    let state = AppState {};

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch Zeitig.");
}

fn ui() -> impl Widget<AppState> {
    Button::new("Test")
}
