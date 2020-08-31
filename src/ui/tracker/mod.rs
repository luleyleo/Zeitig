use druid::{
    widget::{
        Button, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, Painter, Scroll,
        SizedBox, TextBox,
    },
    Command, Data, EventCtx, LensExt, Selector, UnitPoint, Widget, WidgetExt,
};
use std::time::Duration;

use crate::{
    controller::{
        backend_msg, AutoSaver, BackendController, CommandReceiver, EnterController, Ticker,
    },
    state::{
        Action, ActiveSession, AppState, Content, Creating, Creator, DateTime, Session, Setup,
        SpentTime, Subject, Topic,
    },
    widgets::Maybe,
};

const SELECT_ACTION: Selector<Action> = Selector::new("zeitig.select_action");
const SELECT_SUBJECT: Selector<Subject> = Selector::new("zeitig.select_subject");

fn start_new_session(data: &mut AppState) {
    data.active = Some(ActiveSession {
        started: DateTime::now(),
        duration: SpentTime::default(),
    })
}

fn end_session(data: &mut AppState) {
    if data.active.is_some() {
        let active = data.active.take().unwrap();
        if *active.duration > Duration::from_secs(30) {
            let session = Session {
                topic: Topic {
                    action: data.setup.selected_action.clone().unwrap(),
                    subject: data.setup.selected_subject.clone().unwrap(),
                },
                started: active.started,
                ended: DateTime::now(),
            };
            data.history.add(session);
        }
    }
}

fn handle_command(_ctx: &mut EventCtx, data: &mut AppState, cmd: &Command) {
    if let Some(action) = cmd.get(SELECT_ACTION) {
        end_session(data);
        data.setup.selected_action = Some(action.clone());
    }
    if let Some(subject) = cmd.get(SELECT_SUBJECT) {
        end_session(data);
        data.setup.selected_subject = Some(subject.clone());
    }
}

pub fn ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(header())
        .with_spacer(5.0)
        .with_child(separator())
        .with_spacer(10.0)
        .with_flex_child(lists(), 1.0)
        .with_child(dialogs())
        .with_child(buttons())
        .controller(CommandReceiver::new(handle_command))
        .controller(BackendController::new())
        .controller(AutoSaver::new())
}

fn selected_action_label() -> impl Widget<Option<Action>> {
    Maybe::new(
        Label::dynamic(|action: &Action, _| format!("{}", action.as_ref())),
        Label::new("No Action"),
    )
}

fn selected_subject_label() -> impl Widget<Option<Subject>> {
    Maybe::new(
        Label::dynamic(|subject: &Subject, _| format!("{}", subject.as_ref())),
        Label::new("No Subject"),
    )
}

fn session_duration_label() -> impl Widget<Option<ActiveSession>> {
    Maybe::new(
        Label::dynamic(|session: &ActiveSession, _| format!("Session: {}", session.duration)),
        Label::new("Session: not running"),
    )
}

fn separator<T: Data>() -> impl Widget<T> {
    use druid::RenderContext;
    Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &env.get(druid::theme::BORDER_DARK));
    })
    .expand_width()
    .fix_height(2.0)
}

fn header() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(session_duration_label().lens(AppState::active))
                        .with_child(Label::dynamic(|data: &AppState, _| {
                            format!("Total: {}", data.session_duration())
                        }))
                        .expand_width(),
                    1.0,
                )
                .with_spacer(5.0)
                .with_child(
                    Button::new(|data: &AppState, _: &_| match data.active {
                        None => "Start".to_string(),
                        Some(_) => "Stop".to_string(),
                    })
                    .on_click(|_, data: &mut AppState, _| match data.active {
                        Some(_) => end_session(data),
                        None => start_new_session(data),
                    }),
                )
                .padding((10.0, 10.0, 10.0, 5.0))
                .controller(Ticker::new()),
        )
        .with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::Center)
                .with_child(selected_action_label().lens(Setup::selected_action))
                .with_child(Label::new(""))
                .with_child(selected_subject_label().lens(Setup::selected_subject))
                .lens(AppState::setup),
        )
}

fn lists() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(
            Scroll::new(List::new(|| {
                Label::dynamic(|action: &Action, _| action.as_ref().to_string())
                    .padding(3.0)
                    .on_click(|ctx, action: &mut Action, _| {
                        ctx.submit_command(SELECT_ACTION.with(action.clone()), None);
                    })
                    .align_horizontal(UnitPoint::CENTER)
            }))
            .vertical()
            .lens(AppState::content.then(Content::actions))
            .expand_width(),
            1.0,
        )
        .with_flex_child(
            Scroll::new(List::new(|| {
                Label::dynamic(|subject: &Subject, _| subject.as_ref().to_string())
                    .padding(3.0)
                    .on_click(|ctx, subject: &mut Subject, _| {
                        ctx.submit_command(SELECT_SUBJECT.with(subject.clone()), None);
                    })
                    .align_horizontal(UnitPoint::CENTER)
            }))
            .vertical()
            .lens(AppState::content.then(Content::subjects))
            .expand_width(),
            1.0,
        )
        .expand_height()
}

fn dialogs() -> impl Widget<AppState> {
    const ADVANCE: Selector<Creating> = Selector::new("zeitig.dialogs.advance");
    fn handle_advance(ctx: &mut EventCtx, data: &mut AppState, cmd: &Command) {
        if let Some(creating) = cmd.get(ADVANCE) {
            if creating == &Creating::Nothing {
                match &data.setup.creating {
                    Creating::Action(a) => {
                        ctx.submit_command(backend_msg::ADD_ACTION.with(a.to_owned()), None);
                    }
                    Creating::Subject(s) => {
                        ctx.submit_command(backend_msg::ADD_SUBJECT.with(s.to_owned()), None);
                    }
                    _ => {}
                }
            }
            data.setup.creating = creating.clone();
        }
    }
    fn handle_creation(_ctx: &mut EventCtx, data: &mut AppState, cmd: &Command) {
        match cmd {
            _ if cmd.is(backend_msg::ACTION_ADDED) => {
                let action = cmd.get_unchecked(backend_msg::ACTION_ADDED).clone();
                data.content.actions.insert_ord(action);
            }
            _ if cmd.is(backend_msg::SUBJECT_ADDED) => {
                let subject = cmd.get_unchecked(backend_msg::SUBJECT_ADDED).clone();
                data.content.subjects.insert_ord(subject);
            }
            _ => {}
        }
    }
    fn finish(ctx: &mut EventCtx) {
        ctx.submit_command(ADVANCE.with(Creating::Nothing), None);
    }
    fn base<T: Data>(title: &str, content: impl Widget<T> + 'static) -> impl Widget<T> {
        Flex::column()
            .with_child(Label::new(title))
            .with_spacer(5.0)
            .with_child(content)
            .padding(5.0)
            .border(druid::theme::BORDER_LIGHT, 2.0)
            .rounded(5.0)
            .padding(10.0)
    }
    Creator::new()
        .nothing(SizedBox::empty())
        .choosing(base(
            "What to add?",
            Flex::row()
                .with_child(Button::new("Action").on_click(|ctx, _, _| {
                    ctx.submit_command(ADVANCE.with(Creating::Action(String::new())), None)
                }))
                .with_spacer(5.0)
                .with_child(Button::new("Subject").on_click(|ctx, _, _| {
                    ctx.submit_command(ADVANCE.with(Creating::Subject(String::new())), None)
                })),
        ))
        .action(base(
            "Add new action:",
            Flex::row()
                .with_flex_child(
                    TextBox::new()
                        .controller(EnterController::new(|ctx, _| finish(ctx)))
                        .expand_width(),
                    1.0,
                )
                .with_spacer(3.0)
                .with_child(Button::new("Add").on_click(|ctx, _, _| finish(ctx))),
        ))
        .subject(base(
            "Add new subject:",
            Flex::row()
                .with_flex_child(
                    TextBox::new()
                        .controller(EnterController::new(|ctx, _| finish(ctx)))
                        .expand_width(),
                    1.0,
                )
                .with_spacer(3.0)
                .with_child(Button::new("Add").on_click(|ctx, _, _| finish(ctx))),
        ))
        .lens(AppState::setup.then(Setup::creating))
        .controller(CommandReceiver::new(handle_advance))
        .controller(CommandReceiver::new(handle_creation))
}

fn buttons() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(
            Button::dynamic(Setup::new_item_label)
                .on_click(|_, data: &mut Setup, _| {
                    data.creating = match data.creating {
                        Creating::Nothing => Creating::Choosing,
                        _ => Creating::Nothing,
                    }
                })
                .lens(AppState::setup)
                .expand_width(),
            1.0,
        )
        .with_flex_child(
            Button::new("Insights")
                .on_click(|_ctx, _data: &mut AppState, _| {
                    //if data.insights.is_none() {
                    //    data.insights = Some(Insights::generate(data));
                    //}
                    //ctx.new_window(WindowDesc::new(ui::insights).title("Insights"));
                })
                .expand_width(),
            1.0,
        )
}
