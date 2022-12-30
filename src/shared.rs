use once_cell::sync::OnceCell;
use crate::utils::config::Configuration;
use crate::utils::types::snowflake::SnowflakeGenerator;

pub const SNOWFLAKE_GENERATOR: OnceCell<SnowflakeGenerator> = OnceCell::new();
pub const CONFIGURATION: OnceCell<Configuration> = OnceCell::new();