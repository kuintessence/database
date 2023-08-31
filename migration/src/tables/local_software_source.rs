use sea_orm_migration::prelude::*;

use super::uuid_pkey;
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum LocalSoftwareSource {
    Table,

    Id,
    Name,
    Version,
    SoftwareInstallArgument,
    ExePath,
    QueueId,
}

impl SchemaTable for LocalSoftwareSource {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::Version).string().not_null())
            .col(
                ColumnDef::new(Self::SoftwareInstallArgument)
                    .json_binary()
                    .not_null(),
            )
            .col(ColumnDef::new(Self::ExePath).string().not_null())
            .col(ColumnDef::new(Self::QueueId).uuid())
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
