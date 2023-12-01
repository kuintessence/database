use sea_orm_migration::{
    async_trait::async_trait, sea_orm::DeriveMigrationName, DbErr, MigrationTrait, SchemaManager, sea_query::{AddColumnOption, TableAlterStatement},
};

use crate::schema::Statements;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
manager.alter_table(TableAlterStatement::new().table(table))
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
