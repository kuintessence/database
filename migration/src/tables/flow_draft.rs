use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Project};
use crate::schema::SchemaTable;

/// Flow Draft
#[derive(Iden)]
pub enum FlowDraft {
    Table,

    Id,
    Name,
    Description,
    Logo,
    Spec,
    UserId,
    CreatedTime,
    LastModifiedTime,
    Type,
    ProjectId,
}

impl SchemaTable for FlowDraft {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Description).string())
            .col(ColumnDef::new(Self::Logo).text())
            .col(ColumnDef::new(Self::Spec).json_binary().not_null())
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .col(
                ColumnDef::new(Self::LastModifiedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::ProjectId).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ProjectId)
                    .to(Project::Table, Project::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
