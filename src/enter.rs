
use druid::{widget::Controller, Widget, EventCtx, Event, Env, KeyCode};
use std::any::Any;

pub struct EnterController<D> {
    callback: Box<dyn Fn(&mut EventCtx, &mut D)>
}

impl<D> EnterController<D> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut D) + Any) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

impl<D, W: Widget<D>> Controller<D, W> for EnterController<D> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        child.event(ctx, event, data, env);
        if let Event::KeyUp(key) = event {
            if key.key_code == KeyCode::Return {
                (self.callback)(ctx, data);
            }
        }
    }
} 
