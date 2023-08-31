use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum SoftwareSource {
    Table,

    Id,
    Name,
    Type,
    Url,
    Username,
    Password,
    ClusterId,
}

impl SchemaTable for SoftwareSource {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::Url).string())
            .col(ColumnDef::new(Self::Username).string())
            .col(ColumnDef::new(Self::Password).string())
            .col(ColumnDef::new(Self::ClusterId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
