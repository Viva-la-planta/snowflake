use std::process;

use crate::snowflake::Snowflake;

/// This is a generation pool that will generate snowflakes.
pub struct Pool {
    /// The assigned worker id.
    worker_id: u64,
    /// The current increment of this pool.
    increment: u64,
}

impl Pool {
    pub fn new(worker_id: u64) -> Self {
        Self {
            worker_id: worker_id,
            increment: 0,
        }
    }

    pub fn generate(&mut self) -> Snowflake {
        self.increment += 1;
        Snowflake::create(self.worker_id, process::id().into(), self.increment)
    }
}