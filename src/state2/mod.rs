use crate::state::{DateTime, SpentTime};
use druid::{Data, Lens};
use druid_enums::Matcher;
use im::{HashMap, Vector};
use std::sync::Arc;

pub mod backend;

#[derive(Clone, Default, Data, Lens)]
pub struct AppState {
    pub content: Content,
    pub history: History,
    pub setup: Setup,
    pub active: Option<ActiveSession>,
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Hash)]
pub struct Topic {
    pub action: Action,
    pub subject: Subject,
}

#[derive(Clone, Data, Lens, Eq, Hash)]
pub struct Action {
    pub id: usize,
    pub name: Arc<str>,
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Data, Lens, Eq, Hash)]
pub struct Subject {
    pub id: usize,
    pub name: Arc<str>,
}

impl AsRef<str> for Subject {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Subject {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Default, Data, Lens)]
pub struct Content {
    pub actions: Vector<Action>,
    pub subjects: Vector<Subject>,
    pub time_table: TimeTable,
}

impl Content {
    pub fn find_action(&self, id: usize) -> Option<Action> {
        self.actions.iter().find(|a| a.id == id).cloned()
    }

    pub fn find_subject(&self, id: usize) -> Option<Subject> {
        self.subjects.iter().find(|s| s.id == id).cloned()
    }
}

#[derive(Clone, Default, Data)]
pub struct TimeTable(HashMap<Topic, SpentTime>);

impl TimeTable {
    pub fn get(&self, topic: &Topic) -> SpentTime {
        self.0.get(topic).cloned().unwrap_or_default()
    }

    pub fn get_mut(&mut self, topic: Topic) -> &mut SpentTime {
        self.0.entry(topic).or_insert(SpentTime::default())
    }

    pub fn iter(&self) -> im::hashmap::Iter<Topic, SpentTime> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> im::hashmap::IterMut<Topic, SpentTime> {
        self.0.iter_mut()
    }
}

impl<'a> IntoIterator for &'a TimeTable {
    type Item = (&'a Topic, &'a SpentTime);
    type IntoIter = im::hashmap::Iter<'a, Topic, SpentTime>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut TimeTable {
    type Item = (&'a Topic, &'a mut SpentTime);
    type IntoIter = im::hashmap::IterMut<'a, Topic, SpentTime>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Clone, Default, Data)]
pub struct History {
    entries: Vector<Session>,
}

impl History {
    pub fn iter(&self) -> im::vector::Iter<Session> {
        self.entries.iter()
    }

    pub fn add(&mut self, session: Session) {
        self.entries.push_back(session);
    }
}

impl<'a> IntoIterator for &'a History {
    type Item = &'a Session;
    type IntoIter = im::vector::Iter<'a, Session>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Data, Lens)]
pub struct Session {
    pub topic: Topic,
    pub started: DateTime,
    pub ended: DateTime,
}

#[derive(Clone, Default, Data, Lens)]
pub struct Setup {
    selected_action: Option<Action>,
    selected_subject: Option<Subject>,
    creating: Creating,
}

#[derive(Clone, Data, Lens)]
pub struct ActiveSession {
    pub started: DateTime,
    pub duration: SpentTime,
}

#[derive(Clone, Data, Matcher, PartialEq, Eq)]
#[matcher(matcher_name = Creator)]
pub enum Creating {
    Nothing,
    Choosing,
    Action(Arc<String>),
    Subject(Arc<String>),
}

impl Default for Creating {
    fn default() -> Self {
        Creating::Nothing
    }
}
