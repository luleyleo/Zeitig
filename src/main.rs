use druid::{
    widget::{Button, Flex, Label, List},
    AppLauncher, Selector, Widget, WidgetExt, WindowDesc,
};
use std::path::Path;

mod state;
use state::AppState;

const SCHEDULE_AUTO_SAVE: Selector = Selector::new("zeitig.schedule-auto-save");
const SELECT_ACTION: Selector = Selector::new("zeitig.select_action");
const SELECT_SUBJECT: Selector = Selector::new("zeitig.select_subject");

fn read_state() -> AppState {
    let path = Path::new("zeitig.yaml");
    if path.exists() {
        let data = std::fs::read_to_string(path).expect("Failed to read data.");
        serde_yaml::from_str(&data).expect("Failed to deserialize data.")
    } else {
        AppState::default()
    }
}

fn write_state(state: AppState) {
    let path = Path::new("zeitig.yaml");
    let data = serde_yaml::to_string(&state).expect("Failed to serialize data.");
    std::fs::write(path, data.as_bytes()).expect("Failed to write data.");
}

fn main() {
    let window = WindowDesc::new(ui)
        .title("Zeitig")
        .window_size((300.0, 400.0));

    let state = read_state();

    AppLauncher::with_window(window)
        .launch(state)
        .expect("Failed to launch Zeitig.");
}

fn ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    Label::dynamic(|time, _| format!("{}", time))
                        .lens(AppState::spent_time)
                        .expand_width(),
                    1.0,
                )
                .with_spacer(5.0)
                .with_child(
                    Button::new(|active: &bool, _: &_| {
                        if !active { "Start" } else { "Stop" }.into()
                    })
                    .on_click(|_, active: &mut bool, _| {
                        *active = !*active;
                    })
                    .lens(AppState::active),
                )
                .padding(10.0),
        )
        .with_spacer(10.0)
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    List::new(|| Label::new("Some Action"))
                        .lens(AppState::actions)
                        .expand_width(),
                    1.0,
                )
                .with_flex_child(
                    List::new(|| Label::new("Some Subject"))
                        .lens(AppState::subjects)
                        .expand_width(),
                    1.0,
                )
                .expand_height(),
            1.0,
        )
        .with_child(
            Flex::row()
                .with_flex_child(Button::new("New Action").expand_width(), 1.0)
                .with_flex_child(Button::new("New Subject").expand_width(), 1.0),
        )
}
