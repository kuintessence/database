use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum WorkOrder {
    Table,

    Id,
    Title,
    Type,
    Description,
    AssignedUserId,
    Status,
    StartTime,
    EndTime,
    CreatedUserId,
}

impl SchemaTable for WorkOrder {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Title).string().not_null())
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::Description).text().not_null())
            .col(ColumnDef::new(Self::AssignedUserId).uuid().not_null())
            .col(ColumnDef::new(Self::Status).integer().not_null())
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
            .col(ColumnDef::new(Self::CreatedUserId).uuid().not_null())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
