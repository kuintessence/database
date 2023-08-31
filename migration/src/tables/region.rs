use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum Region {
    Table,

    Id,
    Name,
    Address,
    Location,
    OrganizationId,
    PostalCode,
    MailingAddress,
    ProviderId,
}

impl SchemaTable for Region {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Address).string().not_null())
            .col(ColumnDef::new(Self::Location).json().not_null())
            .col(ColumnDef::new(Self::OrganizationId).uuid())
            .col(ColumnDef::new(Self::PostalCode).string().not_null())
            .col(ColumnDef::new(Self::MailingAddress).string().not_null())
            .col(ColumnDef::new(Self::ProviderId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
