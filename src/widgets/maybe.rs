use druid::{
    widget::{prelude::*, SizedBox},
    Data, WidgetPod,
};

/// A widget that switches between two possible child views, for `Data` that
/// is `Option<T>`.
pub struct Maybe<T> {
    some: WidgetPod<T, Box<dyn Widget<T>>>,
    none: WidgetPod<(), Box<dyn Widget<()>>>,
}

impl<T: Data> Maybe<T> {
    /// Create a new `Maybe` widget with a `Some` and a `None` branch.
    pub fn new(some: impl Widget<T> + 'static, none: impl Widget<()> + 'static) -> Maybe<T> {
        Maybe {
            some: WidgetPod::new(Box::new(some)),
            none: WidgetPod::new(Box::new(none)),
        }
    }

    /// Create a new `Maybe` widget where the `None` branch is an empty widget.
    #[allow(dead_code)]
    pub fn or_empty(some: impl Widget<T> + 'static) -> Maybe<T> {
        Maybe {
            some: WidgetPod::new(Box::new(some)),
            none: WidgetPod::new(Box::new(SizedBox::empty())),
        }
    }
}

impl<T: Data> Widget<Option<T>> for Maybe<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Option<T>, env: &Env) {
        match data {
            Some(data) if self.some.is_initialized() => self.some.event(ctx, event, data, env),
            Some(_) => (),
            None => self.none.event(ctx, event, &mut (), env),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Option<T>,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            if let Some(data) = data {
                self.some.lifecycle(ctx, event, data, env);
            }
            self.none.lifecycle(ctx, event, &(), env);
        } else {
            match data {
                Some(data) => self.some.lifecycle(ctx, event, data, env),
                None => self.none.lifecycle(ctx, event, &(), env),
            };
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Option<T>, data: &Option<T>, env: &Env) {
        if old_data.is_some() != data.is_some() {
            ctx.children_changed();
        } else {
            match data {
                Some(new) => self.some.update(ctx, new, env),
                None => self.none.update(ctx, &(), env),
            };
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Option<T>,
        env: &Env,
    ) -> Size {
        match data {
            Some(data) => {
                let size = self.some.layout(ctx, bc, data, env);
                self.some.set_layout_rect(ctx, data, env, size.to_rect());
                size
            }
            None => {
                let size = self.none.layout(ctx, bc, &(), env);
                self.none.set_layout_rect(ctx, &(), env, size.to_rect());
                size
            }
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Option<T>, env: &Env) {
        match data {
            Some(data) => self.some.paint(ctx, data, env),
            None => self.none.paint(ctx, &(), env),
        };
    }
}
