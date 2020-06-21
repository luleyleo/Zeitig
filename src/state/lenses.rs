use druid::Lens;

use crate::state::{AppState, SpentTime};

pub struct SpendTimeLens;
impl Lens<AppState, SpentTime> for SpendTimeLens {
    fn with<V, F: FnOnce(&SpentTime) -> V>(&self, data: &AppState, f: F) -> V {
        if let (Some(action), Some(subject)) = (&data.selected_action, &data.selected_subject) {
            f(&data.time_table.get(action, subject))
        } else {
            f(&SpentTime::default())
        }
    }

    fn with_mut<V, F: FnOnce(&mut SpentTime) -> V>(&self, data: &mut AppState, f: F) -> V {
        if let (Some(action), Some(subject)) = (&data.selected_action, &data.selected_subject) {
            f(data.time_table.get_mut(action, subject))
        } else {
            f(&mut SpentTime::default())
        }
    }
}
