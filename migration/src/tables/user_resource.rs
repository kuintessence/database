use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Cluster};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum UserResource {
    Table,

    Id,
    UserId,
    Memory,
    MemoryMax,
    MemoryAlert,
    CoreNumber,
    CoreNumberMax,
    CoreNumberAlert,
    StorageCapacity,
    StorageCapacityMax,
    StorageCapacityAlert,
    QueueId,
    MaxPendingTaskCount,
    MaxNodeCount,
    MaxRunningTaskCount,
    NodeCount,
}

impl SchemaTable for UserResource {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .col(ColumnDef::new(Self::Memory).big_integer())
            .col(ColumnDef::new(Self::MemoryMax).big_integer())
            .col(ColumnDef::new(Self::MemoryAlert).big_integer())
            .col(ColumnDef::new(Self::CoreNumber).big_integer())
            .col(ColumnDef::new(Self::CoreNumberMax).big_integer())
            .col(ColumnDef::new(Self::CoreNumberAlert).big_integer())
            .col(ColumnDef::new(Self::StorageCapacity).big_integer())
            .col(ColumnDef::new(Self::StorageCapacityMax).big_integer())
            .col(ColumnDef::new(Self::StorageCapacityAlert).big_integer())
            .col(ColumnDef::new(Self::QueueId).uuid().not_null())
            .col(ColumnDef::new(Self::MaxPendingTaskCount).big_integer())
            .col(ColumnDef::new(Self::MaxNodeCount).big_integer())
            .col(ColumnDef::new(Self::MaxRunningTaskCount).big_integer())
            .col(ColumnDef::new(Self::NodeCount).big_integer())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::QueueId)
                    .to(Cluster::Table, Cluster::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
