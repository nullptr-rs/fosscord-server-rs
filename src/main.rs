extern crate core;

use actix_web::middleware::{Compress, Logger};
use actix_web::rt::System;
use actix_web::{App, HttpServer, Scope};
use core::panicking;
use fosscord_server_rs::shared::{CONFIGURATION, SNOWFLAKE_GENERATOR};
use fosscord_server_rs::stats;
use fosscord_server_rs::utils::config::Configuration;
use fosscord_server_rs::utils::database::migrator::Migrator;
use fosscord_server_rs::utils::entities::user;
use fosscord_server_rs::utils::entities::user::ActiveModel;
use fosscord_server_rs::utils::other::snowflake::SnowflakeGenerator;
use log::LevelFilter;
use once_cell::sync::OnceCell;
use sea_orm::{ActiveModelTrait, ConnectOptions, Database};
use sea_orm_migration::MigratorTrait;
use std::env;
use std::io::{Error, ErrorKind};
use std::sync::RwLock;
use tokio::io::AsyncBufReadExt;
use tokio::runtime::Builder;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    #[cfg(target_os = "windows")]
    {
        ansi_term::enable_ansi_support().expect("Failed to enable ansi support");
    }

    stats::initialize().expect("Failed to initialize stats");

    //Setting up dev environment
    env::set_var(
        "DATABASE_URL",
        "mysql://nullptr-rs:nullptr-rs@127.0.0.1:3306/fosscord",
    );

    let worker_threads = env::var("WORKER_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap();

    log::info!("Starting server with {} worker threads...", worker_threads);

    System::with_tokio_rt(|| {
        Builder::new_multi_thread()
            .thread_name("fosscord-server-worker")
            .worker_threads(worker_threads)
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
    .block_on(async_bootstrap(worker_threads))
}

async fn async_bootstrap(worker_threads: usize) -> std::io::Result<()> {
    let worker_id = env::var("WORKER_ID")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<u64>()
        .unwrap();
    let process_id = env::var("PROCESS_ID")
        .unwrap_or_else(|_| "0".to_string())
        .parse::<u64>()
        .unwrap();
    let generator = SnowflakeGenerator::new(worker_id, process_id);

    SNOWFLAKE_GENERATOR.set(generator).unwrap();
    CONFIGURATION.set(Configuration::new()).unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut options = ConnectOptions::new(database_url);
    options.sqlx_logging(false);
    options.sqlx_logging_level(LevelFilter::Debug);

    let database = Database::connect(options)
        .await
        .expect("Failed to connect to database. Is the database running ?");
    Migrator::up(&database, None)
        .await
        .expect("Failed to run migrations");

    let host = env::var("FOSSCORD_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("FOSSCORD_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{}:{}", host, port);

    log::info!("Starting server on {}...", &address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(Scope::new("/api"))
    })
    .workers(worker_threads)
    .bind(address)?
    .run()
    .await
}
