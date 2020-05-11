
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