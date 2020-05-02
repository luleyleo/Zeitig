use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, TextBox},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};
use serde::{Serialize, Deserialize};
use std::fmt::Display;
use std::{sync::Arc, time::Duration, path::Path};

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
struct AppState {
    new_subject_name: String,
    subjects: Arc<Vec<Subject>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SpentTime(Duration);

impl SpentTime {
    pub fn new() -> Self {
        SpentTime(Duration::from_secs(0))
    }
}

impl Display for SpentTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}h {}m", self.0.as_secs() / 60, self.0.as_secs() % 60)
    }
}

impl Data for SpentTime {
    fn same(&self, other: &Self) -> bool {
        self.0.as_secs() == other.0.as_secs()
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
struct Subject {
    name: String,
    time: SpentTime,
}

fn read_state() -> AppState {
    let path = Path::new("zeitig.yaml");
    if path.exists() {
        let data = std::fs::read_to_string(path).expect("Failed to read data.");
        serde_yaml::from_str(&data).expect("Failed to deserialize data.")
    } else {
        AppState {
            new_subject_name: String::new(),
            subjects: Default::default(),
        }
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
                    TextBox::new()
                        .expand_width()
                        .lens(AppState::new_subject_name),
                    1.0,
                )
                .with_spacer(5.0)
                .with_child(Button::new("Add").on_click(|_, data: &mut AppState, _| {
                    if !data.new_subject_name.is_empty() {
                        Arc::make_mut(&mut data.subjects).push(Subject {
                            name: data.new_subject_name.clone(),
                            time: SpentTime::new(),
                        });
                        data.new_subject_name.clear();
                        write_state(data.clone());
                    }
                })),
        )
        .with_spacer(10.0)
        .with_child(
            List::new(|| {
                Flex::column()
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .with_child(Label::dynamic(|data: &Subject, _| data.name.to_owned()))
                    .with_child(Label::dynamic(|data: &Subject, _| format!("{}", data.time)))
                    .padding(3.0)
            })
            .expand_width()
            .lens(AppState::subjects),
        )
        .padding(10.0)
}

