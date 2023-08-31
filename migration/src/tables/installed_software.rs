use sea_orm_migration::prelude::*;

use super::{uuid_pkey, SoftwareSource};
use crate::schema::SchemaTable;

#[derive(Iden)]
pub enum InstalledSoftware {
    Table,

    Id,
    SourceId,
    SoftwareId,
    SoftwareName,
    InstallArgument,
    InstalledTime,
    InstalledUserId,
    QueueId,
}

impl SchemaTable for InstalledSoftware {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::SourceId).uuid().not_null())
            .col(ColumnDef::new(Self::SoftwareId).uuid().not_null())
            .col(ColumnDef::new(Self::SoftwareName).string().not_null())
            .col(
                ColumnDef::new(Self::InstallArgument)
                    .json_binary()
                    .not_null(),
            )
            .col(
                ColumnDef::new(Self::InstalledTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::InstalledUserId).uuid().not_null())
            .col(ColumnDef::new(Self::QueueId).uuid())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::SourceId)
                    .to(SoftwareSource::Table, SoftwareSource::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
