use chrono::Utc;

pub const DISCORD_EPOCH: u64 = 1420070400000;

pub type Snowflake = String;

pub struct SnowflakeGenerator {
    pub worker_id: u64,
    pub process_id: u64,
    pub increment: u64,
}

pub struct GeneratedSnowflake {
    pub timestamp: u64,
    pub worker_id: u64,
    pub process_id: u64,
    pub increment: u64,
    pub snowflake: Snowflake,
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u64, process_id: u64) -> SnowflakeGenerator {
        SnowflakeGenerator {
            worker_id,
            process_id,
            increment: 0,
        }
    }

    pub fn generate(&mut self) -> GeneratedSnowflake {
        self.increment = (self.increment + 1) % 4096;

        let timestamp = (Utc::now().timestamp_millis() as u64 - DISCORD_EPOCH) << 22;
        let worker_id = self.worker_id << 17;
        let process_id = self.process_id << 12;

        let snowflake = timestamp | worker_id | process_id | self.increment;

        GeneratedSnowflake {
            timestamp,
            worker_id,
            process_id,
            increment: self.increment,
            snowflake: snowflake.to_string(),
        }
    }
}

impl GeneratedSnowflake {
    pub fn from(snowflake: u64) -> GeneratedSnowflake {
        let timestamp = (snowflake >> 22) + DISCORD_EPOCH;
        let worker_id = (snowflake & 0x3E0000) >> 17;
        let process_id = (snowflake & 0x1F000) >> 12;
        let increment = snowflake & 0xFFF;

        GeneratedSnowflake {
            timestamp,
            worker_id,
            process_id,
            increment,
            snowflake: snowflake.to_string(),
        }
    }
}
