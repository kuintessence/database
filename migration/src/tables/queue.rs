use sea_orm_migration::prelude::*;

use crate::schema::SchemaTable;

use super::{uuid_pkey, Cluster};

#[derive(Iden)]
pub enum Queue {
    Table,

    Id,
    Memory,
    MemoryAlert,
    CoreNumber,
    CoreNumberAlert,
    StorageCapacity,
    StorageCapacityAlert,
    ClusterId,
    ProviderId,
    TopicName,
    Name,
    Description,
    MaxQueuingTaskCount,
    MaxRunningTaskCount,
    MaxNodeCount,
    NodeCount,
    Enabled,
    SchedulerTech,
}

impl SchemaTable for Queue {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::TopicName).string().not_null())
            .col(ColumnDef::new(Self::Memory).big_unsigned().not_null())
            .col(ColumnDef::new(Self::MemoryAlert).big_unsigned())
            .col(ColumnDef::new(Self::CoreNumber).big_unsigned().not_null())
            .col(ColumnDef::new(Self::CoreNumberAlert).big_unsigned())
            .col(ColumnDef::new(Self::StorageCapacity).big_unsigned().not_null())
            .col(ColumnDef::new(Self::StorageCapacityAlert).big_unsigned())
            .col(ColumnDef::new(Self::NodeCount).big_unsigned().not_null())
            .col(ColumnDef::new(Self::MaxNodeCount).big_unsigned())
            .col(ColumnDef::new(Self::MaxQueuingTaskCount).big_unsigned())
            .col(ColumnDef::new(Self::MaxRunningTaskCount).big_unsigned())
            .col(ColumnDef::new(Self::ClusterId).uuid())
            .col(ColumnDef::new(Self::ProviderId).uuid())
            .col(ColumnDef::new(Self::Description).string())
            .col(ColumnDef::new(Self::SchedulerTech).big_integer().not_null())
            .col(ColumnDef::new(Self::Enabled).boolean().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ClusterId)
                    .to(Cluster::Table, Cluster::Id),
            )
            .check(Expr::col(Queue::Memory).gte(0))
            .check(Expr::col(Queue::MemoryAlert).gte(0))
            .check(Expr::col(Queue::CoreNumber).gte(0))
            .check(Expr::col(Queue::CoreNumberAlert).gte(0))
            .check(Expr::col(Queue::StorageCapacity).gte(0))
            .check(Expr::col(Queue::StorageCapacityAlert).gte(0))
            .check(Expr::col(Queue::NodeCount).gte(0))
            .check(Expr::col(Queue::MaxNodeCount).gte(0))
            .check(Expr::col(Queue::MaxQueuingTaskCount).gte(0))
            .check(Expr::col(Queue::MaxRunningTaskCount).gte(0))
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
