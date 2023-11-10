use sea_orm_migration::prelude::*;

use super::{uuid_pkey, FileMetadata};
use crate::schema::SchemaTable;

/// NetDisk File System
#[derive(Iden)]
pub enum FileSystem {
    Table,

    Id,
    ParentId,
    Name,
    IsDict,
    Kind,
    OwnerId,
    CreatedTime,
    FileMetadataId,
    Meta,
}

impl SchemaTable for FileSystem {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::ParentId).uuid())
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::IsDict).boolean().not_null().default(false))
            .col(ColumnDef::new(Self::Kind).integer().not_null())
            .col(ColumnDef::new(Self::OwnerId).uuid().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::FileMetadataId).uuid())
            .col(ColumnDef::new(Self::Meta).json_binary())
            .index(Index::create().unique().col(Self::ParentId).col(Self::Name).col(Self::OwnerId))
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FileMetadataId)
                    .to(FileMetadata::Table, FileMetadata::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ParentId)
                    .to(Self::Table, Self::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
