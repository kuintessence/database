use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum UserInformation {
    Table,

    Id,
    Avatar,
    CreatedAt,
    UpdatedAt,
    OidcId,
    Phone,
    Email,
}

impl SchemaTable for UserInformation {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Avatar).text().not_null())
            .col(
                ColumnDef::new(Self::CreatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(
                ColumnDef::new(Self::UpdatedAt)
                    .timestamp_with_time_zone()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::OidcId).uuid().not_null().unique_key())
            .col(ColumnDef::new(Self::Phone).string())
            .col(ColumnDef::new(Self::Email).string())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
