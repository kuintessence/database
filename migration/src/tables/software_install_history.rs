use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum SoftwareInstallHistory {
    Table,

    Id,
    Name,
    Status,
    Log,
    StartTime,
    EndTime,
    RequestUserId,
    ClusterId,
}

impl SchemaTable for SoftwareInstallHistory {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Status).integer().not_null())
            .col(ColumnDef::new(Self::Log).text().not_null())
            .col(
                ColumnDef::new(Self::StartTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::EndTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::RequestUserId).uuid().not_null())
            .col(ColumnDef::new(Self::ClusterId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
