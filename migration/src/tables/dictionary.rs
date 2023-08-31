use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

/// Enum dictionary
#[derive(Iden)]
pub enum Dictionary {
    Table,

    Id,
    /// Dictionary name
    Name,
    Description,
    /// True for enable, false for disable
    Status,
    CreatedTime,
}

impl SchemaTable for Dictionary {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Description).string().not_null())
            .col(ColumnDef::new(Self::Status).boolean().not_null())
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
