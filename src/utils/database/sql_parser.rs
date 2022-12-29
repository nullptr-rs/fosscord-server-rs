use sea_orm::{DbErr, Statement};
use sea_orm_migration::SchemaManager;
use std::ffi::OsStr;
use std::path::Path;

pub struct SqlParser {
    pub script_path: String,
    pub script_name: String,
}

impl SqlParser {
    pub fn new(script_path: String) -> SqlParser {
        SqlParser {
            script_path,
            script_name: String::new(),
        }
    }

    pub async fn parse(&mut self, manager: &SchemaManager<'_>) -> Result<Vec<Statement>, DbErr> {
        let script_name = Path::new(&self.script_path)
            .file_name()
            .unwrap_or(OsStr::new("Unknown"));
        let script_name = script_name.to_str().unwrap_or("Unknown");
        self.script_name = script_name.to_string();

        let content = tokio::fs::read_to_string(&self.script_path)
            .await
            .map_err(|e| DbErr::Migration(e.to_string()))?;

        let statements = content.split(";").collect::<Vec<&str>>();
        let statements = statements.iter().map(|s| s.trim()).collect::<Vec<&str>>();
        let statements: Vec<Statement> = statements
            .iter()
            .filter(|statement| !statement.is_empty())
            .map(|statement| {
                Statement::from_string(manager.get_database_backend(), statement.to_string())
            })
            .collect();

        Ok(statements)
    }
}
