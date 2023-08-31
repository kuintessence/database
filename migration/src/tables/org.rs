use sea_orm_migration::prelude::*;

use super::{uuid_pkey, Provider};
use crate::schema::SchemaTable;

#[derive(Iden)]

pub enum Org {
    Table,

    Id,
    Name,
    ProviderId,
    ParentId,
}

impl SchemaTable for Org {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::ProviderId).uuid().not_null())
            .col(ColumnDef::new(Self::ParentId).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ProviderId)
                    .to(Provider::Table, Provider::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
