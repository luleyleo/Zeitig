use druid::{widget::Controller, Env, Event, EventCtx, TimerToken, Widget};
use std::time::Duration;

use crate::{controller::SAVE_NOW, state::AppState};

const INTERVAL: Duration = Duration::from_secs(1);

pub struct Ticker {
    timer: Option<TimerToken>,
}

impl Ticker {
    pub fn new() -> Self {
        Self { timer: None }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for Ticker {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Timer(token) = event {
            if Some(*token) == self.timer {
                if let Some(session) = data.active.as_mut() {
                    *session.duration += INTERVAL;
                    self.timer = Some(ctx.request_timer(INTERVAL));
                }
            }
        }
        child.event(ctx, event, data, env);
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        match (old_data.active.is_some(), data.active.is_some()) {
            (false, true) => self.timer = Some(ctx.request_timer(INTERVAL)),
            (true, false) => {
                self.timer = None;
                ctx.submit_command(SAVE_NOW, None)
            }
            _ => (),
        }
        child.update(ctx, old_data, data, env);
    }
}
