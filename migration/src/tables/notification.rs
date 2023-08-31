use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]

pub enum Notification {
    Table,

    Id,
    Title,
    Content,
    Type,
    Priority,
    IsRead,
    RelatedItemId,
    UserId,
    IsDeleted,
    CreatedTime,
}

impl SchemaTable for Notification {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Title).string().not_null())
            .col(ColumnDef::new(Self::Content).text().not_null())
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::Priority).integer().not_null())
            .col(ColumnDef::new(Self::IsRead).boolean().not_null())
            .col(ColumnDef::new(Self::RelatedItemId).uuid().not_null())
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .col(ColumnDef::new(Self::IsDeleted).boolean().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
