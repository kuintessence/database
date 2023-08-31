use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

/// Flow template
#[derive(Iden)]
pub enum FlowTemplate {
    Table,

    Id,
    Name,
    Description,
    Logo,
    Spec,
    UserId,
    CreatedTime,
    FavoriteCount,
    Status,
}

impl SchemaTable for FlowTemplate {
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
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::FavoriteCount)
                    .integer()
                    .not_null()
                    .default(0),
            )
            .col(ColumnDef::new(Self::Status).string())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
