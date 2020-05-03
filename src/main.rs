use druid::{
    widget::{Button, Controller, CrossAxisAlignment, Flex, Label, List, TextBox},
    AppLauncher, Command, Data, Env, Event, EventCtx, Lens, Selector, TimerToken, Widget,
    WidgetExt, WidgetId, WindowDesc,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{path::Path, sync::Arc, time::Duration};

// payload: WidgetId
const START: Selector = Selector::new("zeitig.start");
const SAVE: Selector = Selector::new("zeitig.save");
const SCHEDULE_AUTO_SAVE: Selector = Selector::new("zeitig.schedule-auto-save");

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
        let total = self.0.as_secs();
        let hours = total / 60 / 60;
        let minutes = (total / 60) % 60;
        let seconds = total % 60;
        write!(f, "{}h {}m {}s", hours, minutes, seconds)
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
    #[serde(skip)]
    active: bool,
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
                .must_fill_main_axis(true)
                .with_flex_child(
                    TextBox::new()
                        .lens(AppState::new_subject_name)
                        .expand_width(),
                    1.0,
                )
                .with_spacer(5.0)
                .with_child(Button::new("Add").on_click(|_, data: &mut AppState, _| {
                    if !data.new_subject_name.is_empty() {
                        Arc::make_mut(&mut data.subjects).push(Subject {
                            name: data.new_subject_name.clone(),
                            time: SpentTime::new(),
                            active: false,
                        });
                        data.new_subject_name.clear();
                        write_state(data.clone());
                    }
                })),
        )
        .with_spacer(10.0)
        .with_child(
            List::new(|| {
                Flex::row()
                    .must_fill_main_axis(true)
                    .with_flex_child(
                        Flex::column()
                            .cross_axis_alignment(CrossAxisAlignment::Start)
                            .with_child(Label::dynamic(|data: &Subject, _| data.name.to_owned()))
                            .with_child(Label::dynamic(|data: &Subject, _| {
                                format!("{}", data.time)
                            }))
                            .expand_width(),
                        1.0,
                    )
                    .with_child(
                        Button::new(|data: &Subject, _: &Env| {
                            String::from(if !data.active { "Start" } else { "Stop" })
                        })
                        .on_click(|_, data: &mut Subject, _| data.active = !data.active),
                    )
                    .padding(3.0)
                    .controller(Ticker::new())
            })
            .expand_width()
            .lens(AppState::subjects),
        )
        .padding(10.0)
        .controller(AutoSaver::new())
}

struct AutoSaver {
    timer: Option<TimerToken>,
}

impl AutoSaver {
    fn new() -> Self {
        Self { timer: None }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for AutoSaver {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.selector == SCHEDULE_AUTO_SAVE => {
                self.timer = Some(ctx.request_timer(Duration::from_secs(5)));
            }
            Event::Command(cmd) if cmd.selector == SAVE => {
                self.timer = None;
                write_state(data.clone());
            }
            Event::Timer(token) if Some(*token) == self.timer => {
                self.timer = None;
                write_state(data.clone());
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        if self.timer.is_none() {
            ctx.submit_command(SCHEDULE_AUTO_SAVE, None);
        }
        child.update(ctx, old_data, data, env)
    }
}

struct Ticker {
    timer: Option<TimerToken>,
}

impl Ticker {
    fn new() -> Self {
        Self { timer: None }
    }
}

impl<W: Widget<Subject>> Controller<Subject, W> for Ticker {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Subject,
        env: &Env,
    ) {
        match event {
            Event::Timer(token) if Some(*token) == self.timer => {
                data.time.0 += Duration::from_secs(1);
                self.timer = Some(ctx.request_timer(Duration::from_secs(1)));
            }
            Event::Command(cmd) if cmd.selector == START => {
                let id: WidgetId = *cmd.get_object().expect("Wrong payload for START command");
                if id == ctx.widget_id() {
                    self.timer = Some(ctx.request_timer(Duration::from_secs(1)));
                } else {
                    self.timer = None;
                    data.active = false;
                }
            }
            _ => (),
        }
        child.event(ctx, event, data, env);
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &Subject,
        data: &Subject,
        env: &Env,
    ) {
        match (old_data.active, data.active) {
            (false, true) => ctx.submit_command(Command::new(START, ctx.widget_id()), None),
            (true, false) => {
                self.timer = None;
                ctx.submit_command(SAVE, None)
            }
            _ => (),
        }
        child.update(ctx, old_data, data, env);
    }
}
