use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]

pub enum NodeInstanceBilling {
    Table,

    Id,
    NodeInstanceId,
    FlowInstanceId,
    Cpu,
    Memory,
    Storage,
    CpuTime,
    WallTime,
    Price,
    Formula,
    CreatedTime,
    ModifiedTime,
}

impl SchemaTable for NodeInstanceBilling {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(
                ColumnDef::new(Self::NodeInstanceId)
                    .uuid()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Self::FlowInstanceId).uuid().not_null())
            .col(ColumnDef::new(Self::Cpu).big_integer().not_null())
            .col(ColumnDef::new(Self::Memory).big_integer().not_null())
            .col(ColumnDef::new(Self::Storage).big_integer().not_null())
            .col(ColumnDef::new(Self::CpuTime).big_integer().not_null())
            .col(ColumnDef::new(Self::WallTime).big_integer().not_null())
            .col(ColumnDef::new(Self::Price).decimal_len(12, 2).not_null())
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
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
