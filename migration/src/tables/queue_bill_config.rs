use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Queue};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum QueueBillConfig {
    Table,

    Id,
    QueueId,
    Cpu,
    Memory,
    Storage,
    CpuTime,
    WallTime,
    Formula,
    CreatedTime,
    ModifiedTime,
}

impl SchemaTable for QueueBillConfig {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::QueueId).uuid().not_null().unique_key())
            .col(ColumnDef::new(Self::Cpu).decimal_len(20, 10).not_null())
            .col(ColumnDef::new(Self::Memory).decimal_len(20, 10).not_null())
            .col(ColumnDef::new(Self::Storage).decimal_len(20, 10).not_null())
            .col(ColumnDef::new(Self::CpuTime).decimal_len(20, 10).not_null())
            .col(
                ColumnDef::new(Self::WallTime)
                    .decimal_len(20, 10)
                    .not_null(),
            )
            .col(ColumnDef::new(Self::Formula).string().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::ModifiedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::QueueId)
                    .to(Queue::Table, Queue::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
