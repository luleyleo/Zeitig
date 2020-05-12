
use druid::{TimerToken, Selector, Widget, widget::Controller, EventCtx, Event, Env};
use crate::{write_state, state::AppState};
use std::time::Duration;

const SCHEDULE_AUTO_SAVE: Selector = Selector::new("zeitig.schedule-auto-save");
pub const SAVE_NOW: Selector = Selector::new("zeitig.save");

pub struct AutoSaver {
    timer: Option<TimerToken>,
}

impl AutoSaver {
    pub fn new() -> Self {
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
            Event::Command(cmd) if cmd.selector == SAVE_NOW => {
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