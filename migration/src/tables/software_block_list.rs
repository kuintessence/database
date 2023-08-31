use sea_orm_migration::prelude::*;

use super::{uuid_pkey, TRANSACTION_TIMESTAMP};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum SoftwareBlockList {
    Table,

    Id,
    Name,
    Version,
    CreatedUserId,
    CreatedTime,
    LastModifiedTime,
    ClusterId,
}

impl SchemaTable for SoftwareBlockList {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Version).string().not_null())
            .col(ColumnDef::new(Self::CreatedUserId).uuid().not_null())
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
                    .default(Expr::cust(TRANSACTION_TIMESTAMP)),
            )
            .col(ColumnDef::new(Self::ClusterId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
