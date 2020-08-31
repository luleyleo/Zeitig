use druid::{Data, Lens};
use im::Vector;

use crate::state::{Action, AppState, Date, SpentTime, Subject};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Data, Lens)]
pub struct Insights {
    weeks: Vector<Week>,
}

impl Insights {
    // TODO: Bring back insights
    #[allow(dead_code)]
    pub fn generate(data: &AppState) -> Self {
        let mut this = Insights::default();
        let mut entries: HashMap<Topic, SpentTime> = HashMap::new();
        let mut week: Option<Week> = None;
        for session in &data.history {
            if let Some(mut w) = week.take() {
                if w.end.inner() < &session.started.date() {
                    for (topic, time) in entries.drain() {
                        w.entries.insert_ord(Summary {
                            topic,
                            spent_time: time,
                        });
                    }
                    this.weeks.push_back(w);
                    week = None;
                } else {
                    week = Some(w);
                }
            }
            if week.is_none() {
                let date = session.started.date();
                let weekday = date.weekday();
                let monday = date - time::Duration::days(weekday.number_days_from_monday() as i64);
                let friday = monday + time::Duration::days(6);
                week = Some(Week {
                    begin: Date::from(monday),
                    end: Date::from(friday),
                    entries: Vector::new(),
                });
            }
            let topic = Topic {
                action: session.action.clone(),
                subject: session.subject.clone(),
            };
            *entries.entry(topic).or_default() += session.duration;
        }
        if let Some(mut w) = week {
            for (topic, time) in entries.drain() {
                w.entries.insert_ord(Summary {
                    topic,
                    spent_time: time,
                });
            }
            this.weeks.push_back(w);
        }
        this
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Week {
    pub begin: Date,
    pub end: Date,
    pub entries: Vector<Summary>,
}

#[derive(Debug, Clone, Data, Lens, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Topic {
    pub action: Action,
    pub subject: Subject,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct Summary {
    pub topic: Topic,
    pub spent_time: SpentTime,
}

impl PartialEq for Summary {
    fn eq(&self, other: &Self) -> bool {
        self.topic.eq(&other.topic)
    }
}

impl Eq for Summary {}

impl PartialOrd for Summary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.topic.partial_cmp(&other.topic)
    }
}

impl Ord for Summary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.topic.cmp(&other.topic)
    }
}
