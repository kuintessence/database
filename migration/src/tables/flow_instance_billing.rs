use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

/// Billing of flow instance
#[derive(Iden)]
pub enum FlowInstanceBilling {
    Table,

    Id,
    FlowInstanceId,
    Cpu,
    Memory,
    Storage,
    WallTime,
    TotalPrice,
    UserId,
    CreatedTime,
    ModifiedTime,
}

impl SchemaTable for FlowInstanceBilling {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(
                ColumnDef::new(Self::FlowInstanceId)
                    .uuid()
                    .not_null()
                    .unique_key(),
            )
            .col(ColumnDef::new(Self::Cpu).integer().not_null())
            .col(ColumnDef::new(Self::Memory).big_integer().not_null())
            .col(ColumnDef::new(Self::Storage).big_integer().not_null())
            .col(ColumnDef::new(Self::WallTime).big_integer().not_null())
            .col(
                ColumnDef::new(Self::TotalPrice)
                    .decimal_len(12, 2)
                    .not_null(),
            )
            .col(ColumnDef::new(Self::UserId).uuid())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp())
                    .not_null(),
            )
            .col(
                ColumnDef::new(Self::ModifiedTime)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
