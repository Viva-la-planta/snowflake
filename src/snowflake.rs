use std::time::{SystemTime, UNIX_EPOCH};

// A snowflake               1420070400000
pub const VIVA_EPOCH: u128 = 1660769388000;

pub const WORKER_BITS: u8 = 5;
pub const PROCESS_BITS: u8 = 5;
pub const INCREMENT_BITS: u8 = 12;
pub const MAX_WORKER_ID: u32 = 1 << WORKER_BITS;
pub const MAX_PROCESS_ID: u32 = 1 << PROCESS_BITS;
pub const MAX_INCREMENT_ID: u32 = 1 << INCREMENT_BITS;
pub const MAX_TIMESTAMP: u64 = 1 << (64 - WORKER_BITS - PROCESS_BITS - INCREMENT_BITS);


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

    pub fn init_from(id: u64, timestamp: u64, worker_id: u64, process_id: u64, increment: u64) -> Self {
        Self {
            id,
            timestamp,
            worker_id,
            process_id,
            increment,
        }
    }

    pub fn get_timestamp_at(&self) -> u64 {
        ((self.timestamp as u128) + VIVA_EPOCH) as u64
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

    pub fn get_id(&self) -> u64 {
        self.id
    }


    /// This function will create a Snowflake from the current time, the worker ID, the process ID, and an increment.
    /// The increment is used to ensure that IDs are unique across a worker process.
    pub fn create(worker_id: u64, process_id: u64, increment: u64) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let timestamp: u64 = ((now - VIVA_EPOCH) % (MAX_TIMESTAMP as u128)).try_into().unwrap();
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
        let mut id = timestamp << WORKER_BITS << PROCESS_BITS << INCREMENT_BITS;
        id |= (worker_id % (MAX_WORKER_ID as u64)) << PROCESS_BITS << INCREMENT_BITS;
        id |= (process_id % (MAX_PROCESS_ID as u64)) << INCREMENT_BITS;
        id |=  increment % MAX_INCREMENT_ID as u64;
        id
    }
}

impl From<&str> for Snowflake {
    fn from(s: &str) -> Self {
        Snowflake::new(s.parse::<u64>().unwrap())
    }
}

impl From<String> for Snowflake {
    fn from(s: String) -> Self {
        Snowflake::new(s.parse::<u64>().unwrap())
    }
}

impl Into<String> for Snowflake {
    fn into(self) -> String {
        self.id.to_string()
    }
}