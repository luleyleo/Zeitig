use druid::{
    widget::{Button, CrossAxisAlignment, Either, Flex, Label, List, SizedBox, TextBox},
    AppLauncher, Selector, Widget, WidgetExt, WindowDesc,
};
use std::{mem, path::Path, sync::Arc};

mod state;
use state::{Action, AppState, Creating, Subject};

mod enter;
use enter::EnterController;

mod auto_saver;
use auto_saver::{AutoSaver, SAVE_NOW};

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
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_flex_child(
                    List::new(|| Label::dynamic(|action: &Action, _| action.as_ref().to_string()))
                        .lens(AppState::actions)
                        .expand_width(),
                    1.0,
                )
                .with_flex_child(
                    List::new(|| {
                        Label::dynamic(|subject: &Subject, _| subject.as_ref().to_string())
                    })
                    .lens(AppState::subjects)
                    .expand_width(),
                    1.0,
                )
                .expand_height(),
            1.0,
        )
        .with_child(Either::new(
            |data, _| data.creating != Creating::None,
            Flex::column()
                .with_child(
                    Label::dynamic(|data: &AppState, _| {
                        match data.creating {
                            Creating::None => "No Title",
                            Creating::Action => "Add new action",
                            Creating::Subject => "Add new subject",
                        }
                        .to_string()
                    })
                    .expand_width(),
                )
                .with_child(
                    TextBox::new()
                        .lens(AppState::creating_name)
                        .controller(EnterController::new(|ctx, data: &mut AppState| match data
                            .creating
                        {
                            Creating::None => (),
                            Creating::Action => {
                                Arc::make_mut(&mut data.actions)
                                    .push(Action::new(mem::take(&mut data.creating_name)));
                                ctx.submit_command(SAVE_NOW, None);
                            }
                            Creating::Subject => {
                                Arc::make_mut(&mut data.subjects)
                                    .push(Subject::new(mem::take(&mut data.creating_name)));
                                ctx.submit_command(SAVE_NOW, None);
                            }
                        }))
                        .expand_width(),
                )
                .padding(5.0),
            SizedBox::empty(),
        ))
        .with_child(
            Flex::row()
                .with_flex_child(
                    Button::new(|data: &AppState, _: &_| {
                        if data.creating == Creating::Action {
                            "Cancel"
                        } else {
                            "New Action"
                        }
                        .into()
                    })
                    .on_click(|_, data: &mut AppState, _| {
                        data.creating = if data.creating == Creating::Action {
                            Creating::None
                        } else {
                            Creating::Action
                        }
                    })
                    .expand_width(),
                    1.0,
                )
                .with_flex_child(
                    Button::new(|data: &AppState, _: &_| {
                        if data.creating == Creating::Subject {
                            "Cancel"
                        } else {
                            "New Subject"
                        }
                        .into()
                    })
                    .on_click(|_, data: &mut AppState, _| {
                        data.creating = if data.creating == Creating::Subject {
                            Creating::None
                        } else {
                            Creating::Subject
                        }
                    })
                    .expand_width(),
                    1.0,
                ),
        )
        .controller(AutoSaver::new())
}
