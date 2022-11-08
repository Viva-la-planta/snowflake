use crate::snowflake::Snowflake;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// This is a generation pool that will generate snowflakes.
pub struct Pool {
    /// The assigned worker id.
    worker_id: u64,
    /// The current increment of this pool.
    increment: u64,
    /// The startup time of the pool. This is used with the process id
    /// to make it unique
    startup: u128,
}

impl Pool {
    pub fn new(worker_id: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self {
            worker_id: worker_id,
            increment: 0,
            startup: now,
        }
    }

    pub fn generate(&mut self) -> Snowflake {
        self.increment += 1;
        Snowflake::create(
            self.worker_id.into(),
            self.startup as u64 - process::id() as u64,
            self.increment.into(),
        )
    }
}
