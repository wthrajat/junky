use std::time::Duration;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub struct MinimumAge {
    inner: Option<Duration>,
}

impl MinimumAge {
    pub fn none() -> Self {
        Self { inner: None }
    }

    pub fn from_hours(hours: u64) -> Self {
        Self {
            inner: Some(Duration::from_secs(hours.saturating_mul(3600))),
        }
    }

    pub fn as_duration(self) -> Option<Duration> {
        self.inner
    }

    pub fn cutoff(self) -> Option<SystemTime> {
        self.inner.map(|required| {
            SystemTime::now()
                .checked_sub(required)
                .unwrap_or(std::time::UNIX_EPOCH)
        })
    }
}
