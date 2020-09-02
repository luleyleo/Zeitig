use crate::{controller::backend_msg, state::AppState, ui::tracker};
use druid::{widget::Controller, Env, Event, EventCtx, Widget};

pub struct CloseController {
    should_close: bool,
}

impl CloseController {
    pub fn new() -> Self {
        CloseController {
            should_close: false,
        }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for CloseController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(druid::commands::CLOSE_WINDOW) => {
                if !self.should_close {
                    ctx.set_handled();
                    self.should_close = true;
                    if data.active.is_some() {
                        tracker::end_session(ctx, data);
                    }
                    ctx.submit_command(backend_msg::STOP, None);
                }
            }
            Event::Command(cmd) if cmd.is(backend_msg::STOPPED) => {
                ctx.window().close();
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
