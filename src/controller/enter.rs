use druid::{keyboard_types::Key, widget::Controller, Env, Event, EventCtx, Widget};
use std::any::Any;

pub struct EnterController<D> {
    callback: Box<dyn Fn(&mut EventCtx, &mut D)>,
}

impl<D> EnterController<D> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut D) + Any) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl<D, W: Widget<D>> Controller<D, W> for EnterController<D> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        child.event(ctx, event, data, env);
        if let Event::KeyUp(event) = event {
            if event.key == Key::Enter {
                (self.callback)(ctx, data);
            }
        }
    }
}
