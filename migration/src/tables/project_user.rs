use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Project};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum ProjectUser {
    Table,

    Id,
    ProjectId,
    UserId,
    GrantedBy,
    Role,
    CreatedAt,
    UpdatedAt,
}

impl SchemaTable for ProjectUser {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::ProjectId).uuid().not_null())
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .col(ColumnDef::new(Self::GrantedBy).uuid().not_null())
            .col(ColumnDef::new(Self::Role).integer().not_null().default(0))
            .col(
                ColumnDef::new(Self::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::UpdatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ProjectId)
                    .to(Project::Table, Project::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
