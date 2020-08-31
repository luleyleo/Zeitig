use super::{Action, Backend, Content, History, Session, Subject};
use crate::state::{DateTime, SpentTime, Topic};
use rusqlite::{params, Connection, NO_PARAMS};
use std::{error::Error, path::Path, time::Duration};

static SCHEMA: &str = include_str!("sqlite/schema.sql");

pub struct Sqlite {
    connection: Connection,
}

impl Sqlite {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(path)?;

        Ok(Sqlite { connection })
    }

    pub fn setup(&mut self) -> Result<(), rusqlite::Error> {
        let version = self.query_version();

        if version.is_none() {
            return self.connection.execute_batch(SCHEMA);
        }

        match version.unwrap() {
            1 => {}
            v @ _ => log::error!("Database is using unknown version {}.", v),
        }

        Ok(())
    }

    fn query_version(&mut self) -> Option<u32> {
        if let Ok(_) = self.connection.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='Meta'",
            NO_PARAMS,
            |_row| Ok(()),
        ) {
            self.connection
                .query_row(
                    "select value from Meta where key='version'",
                    NO_PARAMS,
                    |row| row.get(0).map(|s: String| s.parse().ok()),
                )
                .ok()
                .flatten()
        } else {
            None
        }
    }

    pub fn close(self) -> Result<(), Box<dyn Error>> {
        self.connection.close().map_err(|(_, err)| err)?;
        Ok(())
    }
}

fn create_action(connection: &Connection, name: &str) -> Result<Action, Box<dyn Error>> {
    connection.execute("insert into Actions (name) values (?)", &[&name])?;
    let id = connection.last_insert_rowid() as usize;
    let name = name.into();
    Ok(Action { id, name })
}

fn create_subject(connection: &Connection, name: &str) -> Result<Subject, Box<dyn Error>> {
    connection.execute("insert into Subjects (name) values (?)", &[&name])?;
    let id = connection.last_insert_rowid() as usize;
    let name = name.into();
    Ok(Subject { id, name })
}

fn update_time(
    connection: &Connection,
    topic: &Topic,
    time: &SpentTime,
) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "\
        insert into TimeTable (action, subject, duration) \
        values (?1, ?2, ?3) \
        on conflict (action, subject) \
        do update set duration = ?3",
        params![
            topic.action.id as u32,
            topic.subject.id as u32,
            time.as_secs() as u32,
        ],
    )?;
    Ok(())
}

fn add_session(connection: &Connection, session: &Session) -> Result<(), Box<dyn Error>> {
    connection.execute(
        "insert into History (started, ended, action, subject) values (?, ?, ?, ?)",
        params![
            *session.started,
            *session.ended,
            session.topic.action.id as u32,
            session.topic.subject.id as u32,
        ],
    )?;
    Ok(())
}

impl Backend for Sqlite {
    fn transfer_content(&mut self, content: &Content) -> Result<(), Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        for action in &content.actions {
            create_action(&transaction, &action.name)?;
        }

        for subject in &content.subjects {
            create_subject(&transaction, &subject.name)?;
        }

        for (topic, time) in &content.time_table {
            update_time(&transaction, topic, time)?;
        }

        transaction.commit()?;
        Ok(())
    }
    fn transfer_history(&mut self, history: &History) -> Result<(), Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        for session in history {
            add_session(&transaction, session)?;
        }

        transaction.commit()?;
        Ok(())
    }
    fn load_content(&mut self) -> Result<Content, Box<dyn Error>> {
        let mut content = Content::default();

        {
            let mut action_query = self
                .connection
                .prepare_cached("select id, name from Actions")?;
            let mut rows = action_query.query(NO_PARAMS)?;
            while let Some(row) = rows.next()? {
                let id = row.get::<_, u32>("id")? as usize;
                let name = row.get("name")?;
                content.actions.push_back(Action { id, name });
            }
        }

        {
            let mut subject_query = self
                .connection
                .prepare_cached("select id, name from Subjects")?;
            let mut rows = subject_query.query(NO_PARAMS)?;
            while let Some(row) = rows.next()? {
                let id = row.get::<_, u32>("id")? as usize;
                let name = row.get("name")?;
                content.subjects.push_back(Subject { id, name });
            }
        }

        {
            let mut time_query = self
                .connection
                .prepare_cached("select action, subject, duration from TimeTable")?;
            let mut rows = time_query.query(NO_PARAMS)?;
            while let Some(row) = rows.next()? {
                let action_id: usize = row.get::<_, u32>("action")? as usize;
                let subject_id: usize = row.get::<_, u32>("subject")? as usize;
                let duration_secs: u32 = row.get("duration")?;
                let action = content.find_action(action_id).ok_or_else(|| {
                    NoneError::new(format!(
                        "An action with id {} has a session entry but does not exist.",
                        action_id
                    ))
                })?;
                let subject = content.find_subject(subject_id).ok_or_else(|| {
                    NoneError::new(format!(
                        "A subject with id {} has a session entry but does not exist.",
                        action_id
                    ))
                })?;
                let duration = SpentTime::from(Duration::from_secs(duration_secs as u64));
                *content.time_table.get_mut(Topic { action, subject }) = duration;
            }
        }

        Ok(content)
    }
    fn load_history(&mut self, content: &Content) -> Result<History, Box<dyn Error>> {
        let mut history = History::default();
        let mut query = self
            .connection
            .prepare_cached("select started, ended, action, subject from History")?;
        let mut rows = query.query(NO_PARAMS)?;
        while let Some(row) = rows.next()? {
            let started_dt: time::OffsetDateTime = row.get("started")?;
            let ended_dt: time::OffsetDateTime = row.get("ended")?;
            let started = DateTime::from(started_dt);
            let ended = DateTime::from(ended_dt);

            let action_id: usize = row.get::<_, u32>("action")? as usize;
            let subject_id: usize = row.get::<_, u32>("subject")? as usize;
            let action = content.find_action(action_id).ok_or_else(|| {
                NoneError::new(format!(
                    "An action with id {} has a session entry but does not exist.",
                    action_id
                ))
            })?;
            let subject = content.find_subject(subject_id).ok_or_else(|| {
                NoneError::new(format!(
                    "A subject with id {} has a session entry but does not exist.",
                    action_id
                ))
            })?;

            history.add(Session {
                started,
                ended,
                topic: Topic { action, subject },
            })
        }
        Ok(history)
    }
    fn create_action(&mut self, name: &str) -> Result<Action, Box<dyn Error>> {
        create_action(&self.connection, name)
    }
    fn create_subject(&mut self, name: &str) -> Result<Subject, Box<dyn Error>> {
        create_subject(&self.connection, name)
    }
    fn update_time(&mut self, topic: &Topic, time: &SpentTime) -> Result<(), Box<dyn Error>> {
        update_time(&self.connection, topic, time)
    }
    fn add_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>> {
        add_session(&self.connection, session)
    }
}

pub struct NoneError {
    msg: String,
}

impl NoneError {
    pub fn new(msg: impl Into<String>) -> Self {
        NoneError { msg: msg.into() }
    }
}

impl std::fmt::Debug for NoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneError({})", self.msg)
    }
}

impl std::fmt::Display for NoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected to be Some(_): {}", self.msg)
    }
}

impl std::error::Error for NoneError {}
