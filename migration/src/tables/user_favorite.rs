use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum UserFavorite {
    Table,

    Id,
    CreatedAt,
    Type,
    FavoriteId,
    UserId,
    ContentRepoId,
}

impl SchemaTable for UserFavorite {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(
                ColumnDef::new(Self::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::Type).integer().default(0))
            .col(ColumnDef::new(Self::FavoriteId).uuid())
            .col(ColumnDef::new(Self::UserId).uuid())
            .col(ColumnDef::new(Self::ContentRepoId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
