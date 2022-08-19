use std::time::{SystemTime, Duration, UNIX_EPOCH};

// A snowflake              1420070400000
pub const VIVA_EPOCH: u64 = 1660769388000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Snowflake {
    pub id: u64,
    timestamp: u64,
    worker_id: u64,
    process_id: u64,
    increment: u64,
}

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Self::from_id(id)
    }

    /// A method to get the current timestamp.
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    /// A method to get the worker id.
    pub fn get_worker_id(&self) -> u64 {
        self.worker_id
    }

    /// A method to get the process id.
    pub fn get_process_id(&self) -> u64 {
        self.process_id
    }

    /// A method to get the increment.
    pub fn get_increment(&self) -> u64 {
        self.increment
    }


    /// This function will create a Snowflake from the current time, the worker ID, the process ID, and an increment.
    /// The increment is used to ensure that IDs are unique across a worker process.
    pub fn create(worker_id: u64, process_id: u64, increment: u64) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let timestamp = now - VIVA_EPOCH;
        Snowflake {
            id: Self::generate_id(timestamp, worker_id, process_id, increment),
            timestamp: timestamp,
            worker_id: worker_id,
            process_id: process_id,
            increment: increment,
        }
    }

    fn from_id(id: u64) -> Self {
        let timestamp = id >> 22;
        let worker_id = (id & 0x3e0000) >> 17;
        let process_id = (id & 0x1f000) >> 12;
        let increment = id & 0xfff;
        Snowflake {
            id: id,
            timestamp: timestamp,
            worker_id: worker_id,
            process_id: process_id,
            increment: increment,
        }
    }

    fn generate_id(timestamp: u64, worker_id: u64, process_id: u64, increment: u64) -> u64 {
        let mut id = 0;
        id |= timestamp << 22;
        id |= worker_id << 17;
        id |= process_id << 12;
        id |= increment << 0;
        id
    }
}