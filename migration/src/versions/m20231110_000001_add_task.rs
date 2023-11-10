use sea_orm_migration::prelude::*;

use crate::schema::Statements;
use crate::tables::Task;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Statements::create_table().push::<Task>().exec(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Statements::drop_table().push::<Task>().exec(manager).await
    }
}
