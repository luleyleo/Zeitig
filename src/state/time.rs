use druid::Data;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref, DerefMut},
    time::Duration,
};

#[derive(Debug, Clone, Copy, Data, Serialize, Deserialize)]
pub struct DateTime(#[data(same_fn = "PartialEq::eq")] time::OffsetDateTime);

#[derive(Debug, Clone, Copy, Data, Serialize, Deserialize)]
pub struct Date(#[data(same_fn = "PartialEq::eq")] time::Date);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpentTime(Duration);

impl DateTime {
    pub fn now() -> Self {
        DateTime(time::OffsetDateTime::now_local())
    }
}

impl Deref for DateTime {
    type Target = time::OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<time::OffsetDateTime> for DateTime {
    fn from(dt: time::OffsetDateTime) -> Self {
        DateTime(dt)
    }
}

impl Date {
    pub fn inner(&self) -> &time::Date {
        &self.0
    }
}

impl From<time::Date> for Date {
    fn from(date: time::Date) -> Self {
        Date(date)
    }
}

impl From<time::OffsetDateTime> for Date {
    fn from(dt: time::OffsetDateTime) -> Self {
        Date(dt.date())
    }
}

impl Deref for Date {
    type Target = time::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl Add<SpentTime> for SpentTime {
    type Output = SpentTime;

    fn add(self, rhs: SpentTime) -> Self::Output {
        SpentTime(self.0 + rhs.0)
    }
}

impl AddAssign<SpentTime> for SpentTime {
    fn add_assign(&mut self, rhs: SpentTime) {
        self.0 += rhs.0;
    }
}
