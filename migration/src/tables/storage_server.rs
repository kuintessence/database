use sea_orm_migration::prelude::*;

use super::{uuid_pkey, AvailableZone};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum StorageServer {
    Table,

    Id,
    Name,
    Options,
    Capacity,
    StorageType,
    AvailableZoneId,
}

impl SchemaTable for StorageServer {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Options).json().not_null())
            .col(ColumnDef::new(Self::Capacity).string().not_null())
            .col(ColumnDef::new(Self::StorageType).integer().not_null())
            .col(ColumnDef::new(Self::AvailableZoneId).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::AvailableZoneId)
                    .to(AvailableZone::Table, AvailableZone::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
