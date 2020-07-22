use druid::{
    widget::{CrossAxisAlignment, Flex, Label, List, Scroll},
    Widget, WidgetExt,
};

use crate::{
    state::{
        insights::{Insights, Summary, Week},
        AppState,
    },
    widgets::Maybe,
};

#[allow(dead_code)]
pub fn ui() -> impl Widget<AppState> {
    Maybe::or_empty(inner_ui()).lens(AppState::insights)
}

fn inner_ui() -> impl Widget<Insights> {
    Scroll::new(List::new(|| {
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(Label::dynamic(|week: &Week, _| {
                format!("Week {}", *week.begin)
            }))
            .with_spacer(3.0)
            .with_child(
                List::new(|| {
                    Label::dynamic(|sum: &Summary, _| {
                        format!(
                            "    {} {}: {}",
                            sum.topic.action.as_ref(),
                            sum.topic.subject.as_ref(),
                            sum.spent_time
                        )
                    })
                })
                .lens(Week::entries),
            )
            .with_spacer(10.0)
    }))
    .lens(Insights::weeks)
}
