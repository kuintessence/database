use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Dictionary};
use crate::schema::SchemaTable;

/// Enum dictionary value
#[derive(Iden)]
pub enum DictionaryValue {
    Table,

    Id,
    /// Enum name
    Key,
    /// Enum integer
    Value,
    /// Parent dictionary value id
    ParentId,
    /// Dictionary id
    DictionaryId,
    CreatedTime,
}

impl SchemaTable for DictionaryValue {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Key).string().not_null())
            .col(ColumnDef::new(Self::Value).integer().not_null())
            .col(ColumnDef::new(Self::ParentId).uuid())
            .col(ColumnDef::new(Self::DictionaryId).uuid().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::DictionaryId)
                    .to(Dictionary::Table, Dictionary::Id),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ParentId)
                    .to(Self::Table, Self::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
