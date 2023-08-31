use sea_orm_migration::prelude::*;

use super::{uuid_pkey, AvailableZone};
use crate::schema::SchemaTable;

/// Cluster
#[derive(Iden)]
pub enum Cluster {
    Table,

    Id,
    Name,
    /// Belong to available zone id
    AvailableZoneId,
    ProviderId,
}

impl SchemaTable for Cluster {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::AvailableZoneId).uuid().not_null())
            .col(ColumnDef::new(Self::ProviderId).uuid())
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
