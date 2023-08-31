use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

/// File with same hash will be stored only once
#[derive(Iden)]
pub enum FileMetadata {
    Table,

    Id,
    Name,
    Hash,
    HashAlgorithm,
    Size,
    CreatedTime,
}

impl SchemaTable for FileMetadata {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Hash).string().not_null())
            .col(ColumnDef::new(Self::HashAlgorithm).string().not_null())
            .col(ColumnDef::new(Self::Size).integer().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
