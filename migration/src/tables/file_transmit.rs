use sea_orm_migration::prelude::*;

use super::{uuid_pkey, FileMetadata, NodeInstance, StorageServer};
use crate::schema::SchemaTable;

/// File transmit record
#[derive(Iden)]
pub enum FileTransmit {
    Table,

    Id,
    FileMetadataId,
    FromStorageServerId,
    ToStorageServerId,
    FromNodeInstanceId,
    ToNodeInstanceId,
    FromPlot,
    ToPlot,
    /// Transmit type: network, disk, etc.
    Type,
    Status,
    /// Tracking number is used to track the file transmit process
    TrackingNumber,
    StartTime,
    EndTime,
    Comment,
}

impl SchemaTable for FileTransmit {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::FileMetadataId).uuid().not_null())
            .col(ColumnDef::new(Self::FromStorageServerId).uuid().not_null())
            .col(ColumnDef::new(Self::ToStorageServerId).uuid().not_null())
            .col(ColumnDef::new(Self::FromNodeInstanceId).uuid().not_null())
            .col(ColumnDef::new(Self::ToNodeInstanceId).uuid().not_null())
            .col(ColumnDef::new(Self::FromPlot).string().not_null())
            .col(ColumnDef::new(Self::ToPlot).string().not_null())
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::Status).integer().not_null())
            .col(ColumnDef::new(Self::TrackingNumber).string())
            .col(
                ColumnDef::new(Self::StartTime)
                    .timestamp_with_time_zone()
                    .not_null()
                    .default(Expr::current_timestamp()),
            )
            .col(ColumnDef::new(Self::EndTime).timestamp_with_time_zone())
            .col(ColumnDef::new(Self::Comment).string())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FileMetadataId)
                    .to(FileMetadata::Table, FileMetadata::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FromNodeInstanceId)
                    .to(NodeInstance::Table, NodeInstance::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FromStorageServerId)
                    .to(StorageServer::Table, StorageServer::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ToNodeInstanceId)
                    .to(NodeInstance::Table, NodeInstance::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::ToStorageServerId)
                    .to(StorageServer::Table, StorageServer::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
