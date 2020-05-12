use druid::{widget::Controller, Command, Env, Event, EventCtx, Widget};
use std::any::Any;

pub struct CommandReceiver<D> {
    callback: Box<dyn Fn(&mut EventCtx, &mut D, &Command)>,
}

impl<D> CommandReceiver<D> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut D, &Command) + Any) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl<D, W: Widget<D>> Controller<D, W> for CommandReceiver<D> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        if let Event::Command(cmd) = event {
            (self.callback)(ctx, data, cmd);
        }
        child.event(ctx, event, data, env);
    }
}
