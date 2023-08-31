use crate::schema::SchemaTable;
use sea_orm_migration::prelude::*;

use super::{FileMetadata, FlowDraft};

#[derive(Iden)]
/// File uploaded to a node draft
pub enum NodeDraftFile {
    Table,

    FlowDraftId,
    NodeExternalId,
    PlotDescriptor,
    FileMetadataId,
}

impl SchemaTable for NodeDraftFile {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .primary_key(
                Index::create()
                    .col(Self::FlowDraftId)
                    .col(Self::NodeExternalId)
                    .col(Self::FileMetadataId),
            )
            .col(ColumnDef::new(Self::FlowDraftId).uuid())
            .col(ColumnDef::new(Self::NodeExternalId).uuid())
            .col(ColumnDef::new(Self::PlotDescriptor).string())
            .col(ColumnDef::new(Self::FileMetadataId).uuid().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FileMetadataId)
                    .to(FileMetadata::Table, FileMetadata::Id),
            )
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::FlowDraftId)
                    .to(FlowDraft::Table, FlowDraft::Id),
            )
            .to_owned()
    }

    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
