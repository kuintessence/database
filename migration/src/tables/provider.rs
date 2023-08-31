use crate::schema::SchemaTable;
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Provider {
    Table,

    Id,
    Name,
    UserId,
}

impl SchemaTable for Provider {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(ColumnDef::new(Self::Id).uuid().primary_key())
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::UserId).uuid().not_null())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
