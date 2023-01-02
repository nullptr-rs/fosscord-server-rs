use crate::utils::database::sql_parser::SqlParser;
use sea_orm::DeriveMigrationName;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, Statement};
use sea_orm_migration::{MigrationTrait, MigratorTrait, SchemaManager};
use tokio::fs;

const EXCLUDED_MIGRATIONS: &str = "0_foreign_keys";

pub struct Migrator;

#[derive(DeriveMigrationName)]
pub struct FileMigration;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(FileMigration)]
    }
}

#[async_trait::async_trait]
impl MigrationTrait for FileMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend_folder = match manager.get_database_backend() {
            DatabaseBackend::Postgres => "postgres",
            DatabaseBackend::MySql => "mariadb",
            DatabaseBackend::Sqlite => "sqlite",
        };

        migrate_directory(manager, format!("migrations/{}", backend_folder)).await?;
        migrate_file(
            manager,
            format!("migrations/{}/0_foreign_keys.sql", backend_folder),
        )
        .await?;

        Ok(())
    }
}

pub async fn migrate_directory(
    manager: &SchemaManager<'_>,
    directory: String,
) -> Result<(), DbErr> {
    let mut files = fs::read_dir(&directory)
        .await
        .map_err(|e| DbErr::Migration(e.to_string()))?;

    while let Some(file) = files
        .next_entry()
        .await
        .map_err(|e| DbErr::Migration(e.to_string()))?
    {
        let file_type = file
            .file_type()
            .await
            .map_err(|e| DbErr::Migration(e.to_string()))?;
        if file_type.is_dir() {
            continue;
        }

        let file_name = file
            .file_name()
            .into_string()
            .map_err(|e| DbErr::Migration(e.into_string().unwrap()))?;
        if EXCLUDED_MIGRATIONS.contains(&file_name.replace(".sql", "").as_str()) {
            continue;
        }

        let file_path = format!("{}/{}", directory, file_name);

        let file_content = fs::read_to_string(file_path)
            .await
            .map_err(|e| DbErr::Migration(e.to_string()))?;
        let statement = Statement::from_string(manager.get_database_backend(), file_content);

        log::info!("Migrating {}...", file_name);
        manager.get_connection().execute(statement).await?;
    }

    Ok(())
}

pub async fn migrate_file(manager: &SchemaManager<'_>, file_path: String) -> Result<(), DbErr> {
    let mut parser = SqlParser::new(file_path);
    let statements = parser.parse(manager).await?;

    log::info!("Migrating {}...", parser.script_name);
    for statement in statements {
        manager.get_connection().execute(statement).await?;
    }

    Ok(())
}
