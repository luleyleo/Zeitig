use druid::{
    im::{HashMap, Vector},
    Data, Lens,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod files;
pub mod insights;
mod lenses;
mod time;

pub use self::time::{Date, DateTime, SpentTime};
use insights::Insights;

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

    #[serde(skip)]
    pub insights: Option<Insights>,
}

impl AppState {
    #[allow(non_upper_case_globals)]
    pub const spent_time: lenses::SpendTime = lenses::SpendTime;
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
