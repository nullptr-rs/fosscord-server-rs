use crate::utils::config::Configuration;
use crate::utils::other::snowflake::SnowflakeGenerator;
use once_cell::sync::OnceCell;

pub const SNOWFLAKE_GENERATOR: OnceCell<SnowflakeGenerator> = OnceCell::new();
pub const CONFIGURATION: OnceCell<Configuration> = OnceCell::new();
