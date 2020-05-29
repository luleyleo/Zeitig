use druid::im::{HashMap, Vector};
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};
use time::OffsetDateTime;

#[derive(Debug, Clone, Default, Data, Lens, Serialize, Deserialize)]
pub struct AppState {
    pub new_name: String,
    pub actions: Vector<Action>,
    pub selected_action: Option<Action>,
    pub subjects: Vector<Subject>,
    pub selected_subject: Option<Subject>,
    pub time_table: TimeTable,
    pub history: Vector<Session>,

    pub creating: Creating,
    pub creating_name: String,

    pub active: Option<ActiveSession>,
}

impl AppState {
    #[allow(non_upper_case_globals)]
    pub const spent_time: lenses::SpendTimeLens = lenses::SpendTimeLens;
}

#[derive(Debug, Clone, Data, Serialize, Deserialize)]
pub struct DateTime(#[data(same_fn = "PartialEq::eq")] OffsetDateTime);

impl DateTime {
    pub fn now() -> Self {
        DateTime(OffsetDateTime::now_local())
    }
}

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize)]
pub struct ActiveSession {
    pub started: DateTime,
    pub duration: SpentTime,
}

#[derive(Debug, Clone, Data, Lens, Serialize, Deserialize)]
pub struct Session {
    pub action: Action,
    pub subject: Subject,
    pub started: DateTime,
    pub duration: SpentTime,
    pub ended: DateTime,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize, PartialEq, Eq)]
pub enum Creating {
    None,
    Action,
    Subject,
}

impl Default for Creating {
    fn default() -> Self {
        Creating::None
    }
}

#[derive(Debug, Clone, Data, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Subject(Arc<String>);

impl Subject {
    pub fn new(name: String) -> Self {
        Subject(Arc::new(name))
    }
}

impl AsRef<str> for Subject {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Data, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Action(Arc<String>);

impl Action {
    pub fn new(name: String) -> Self {
        Action(Arc::new(name))
    }
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Default, Data, Serialize, Deserialize)]
pub struct TimeTable(HashMap<(Action, Subject), SpentTime>);

impl TimeTable {
    pub fn get(&self, action: &Action, subject: &Subject) -> SpentTime {
        self.0
            .get(&(action.clone(), subject.clone()))
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_mut(&mut self, action: &Action, subject: &Subject) -> &mut SpentTime {
        self.0
            .entry((action.clone(), subject.clone()))
            .or_insert(SpentTime::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpentTime(Duration);

impl Default for SpentTime {
    fn default() -> Self {
        SpentTime(Duration::from_secs(0))
    }
}

impl Deref for SpentTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SpentTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for SpentTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total = self.0.as_secs();
        let hours = total / 60 / 60;
        let minutes = (total / 60) % 60;
        let seconds = total % 60;
        write!(f, "{}h {}m {}s", hours, minutes, seconds)
    }
}

impl Data for SpentTime {
    fn same(&self, other: &Self) -> bool {
        self.0.as_secs() == other.0.as_secs()
    }
}

mod lenses {
    use super::{AppState, SpentTime};
    use druid::Lens;

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
}
