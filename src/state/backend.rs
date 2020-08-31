use super::{Action, Content, History, Session, Subject, Topic};
use crate::state::SpentTime;
use std::error::Error;

mod sqlite;
pub use sqlite::Sqlite;

pub trait Backend {
    fn transfer_content(&mut self, content: &Content) -> Result<(), Box<dyn Error>>;
    fn transfer_history(&mut self, history: &History) -> Result<(), Box<dyn Error>>;

    fn load_content(&mut self) -> Result<Content, Box<dyn Error>>;
    fn load_history(&mut self, content: &Content) -> Result<History, Box<dyn Error>>;

    fn create_action(&mut self, name: &str) -> Result<Action, Box<dyn Error>>;
    fn create_subject(&mut self, name: &str) -> Result<Subject, Box<dyn Error>>;

    fn update_time(&mut self, topic: &Topic, time: &SpentTime) -> Result<(), Box<dyn Error>>;
    fn add_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>>;
}
