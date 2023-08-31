use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use super::Region;
use crate::schema::SchemaTable;

/// Available Zone
#[derive(Iden)]
pub enum AvailableZone {
    Table,

    Id,
    Name,
    /// Belong to region id
    RegionId,
    ProviderId,
}

impl SchemaTable for AvailableZone {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::RegionId).uuid().not_null())
            .col(ColumnDef::new(Self::ProviderId).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::RegionId)
                    .to(Region::Table, Region::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
