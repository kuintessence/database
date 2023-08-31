use sea_orm_migration::prelude::*;

use super::{uuid_pkey, FlowInstance, Queue};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum NodeInstance {
    Table,

    Id,
    Name,
    Kind,
    IsParent,
    BatchParentId,
    Status,
    ResourceMeter,
    Log,
    QueueId,
    FlowInstanceId,
    CreatedTime,
    LastModifiedTime,
}

impl SchemaTable for NodeInstance {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Kind).integer().not_null())
            .col(ColumnDef::new(Self::IsParent).boolean().not_null())
            .col(ColumnDef::new(Self::BatchParentId).uuid())
            .col(ColumnDef::new(Self::Status).integer().not_null())
            .col(ColumnDef::new(Self::ResourceMeter).json_binary())
            .col(ColumnDef::new(Self::Log).string())
            .col(ColumnDef::new(Self::QueueId).uuid())
            .col(ColumnDef::new(Self::FlowInstanceId).uuid().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::LastModifiedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::BatchParentId)
                    .to(Self::Table, Self::Id),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::QueueId)
                    .to(Queue::Table, Queue::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FlowInstanceId)
                    .to(FlowInstance::Table, FlowInstance::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
