use druid::{widget::Controller, Env, Event, EventCtx, Selector, TimerToken, Widget};
use std::time::Duration;

use crate::state::AppState;

pub const SAVE_NOW: Selector = Selector::new("zeitig.save");

pub struct AutoSaver {
    timer: Option<TimerToken>,
}

impl AutoSaver {
    pub fn new() -> Self {
        Self { timer: None }
    }

    fn save(&mut self, _data: &mut AppState) {
        self.timer = None;
        // TODO: actually save the data
        //state::files::write_state(data.clone());
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
            Event::Command(cmd) if cmd.is(SAVE_NOW) => self.save(data),
            Event::Timer(token) if Some(*token) == self.timer => self.save(data),
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
            self.timer = Some(ctx.request_timer(Duration::from_secs(5)));
        }
        child.update(ctx, old_data, data, env)
    }
}
