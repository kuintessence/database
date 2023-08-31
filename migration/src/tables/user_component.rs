use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum UserComponent {
    Table,

    Id,
    Name,
    UsecaseversionId,
    SoftwareVersionId,
    CreatedAt,
    UserId,
}

impl SchemaTable for UserComponent {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::UsecaseversionId).uuid().not_null())
            .col(ColumnDef::new(Self::SoftwareVersionId).uuid().not_null())
            .col(
                ColumnDef::new(Self::CreatedAt)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::UserId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
