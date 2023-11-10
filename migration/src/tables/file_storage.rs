use sea_orm_migration::prelude::*;

use super::{uuid_pkey, FileMetadata, StorageServer};
use crate::schema::SchemaTable;

/// Record where the file metadata is stored
#[derive(Iden)]
pub enum FileStorage {
    Table,

    Id,
    /// Which storage server the file is stored
    StorageServerId,
    FileMetadataId,
    ServerUrl,
    CreatedTime,
    CreatedUserId,
}

impl SchemaTable for FileStorage {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::StorageServerId).uuid().not_null())
            .col(ColumnDef::new(Self::FileMetadataId).uuid().not_null())
            .col(ColumnDef::new(Self::ServerUrl).string().not_null())
            .col(
                ColumnDef::new(Self::CreatedTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::CreatedUserId).uuid().not_null())
            .index(Index::create().unique().col(Self::StorageServerId).col(Self::FileMetadataId))
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FileMetadataId)
                    .to(FileMetadata::Table, FileMetadata::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::StorageServerId)
                    .to(StorageServer::Table, StorageServer::Id)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
