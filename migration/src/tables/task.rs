use sea_orm_migration::prelude::*;

use super::{uuid_pkey, NodeInstance};
use crate::schema::SchemaTable;

/// Task
#[derive(Iden)]
pub enum Task {
    Table,

    Id,
    NodeInstanceId,
    Body,
    Type,
    Status,
}

impl SchemaTable for Task {
    fn create() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(&mut uuid_pkey(Self::Id))
            .col(ColumnDef::new(Self::NodeInstanceId).uuid().not_null())
            .col(ColumnDef::new(Self::Body).json_binary().not_null())
            .col(ColumnDef::new(Self::Type).integer().not_null())
            .col(ColumnDef::new(Self::Status).integer().not_null())
            .foreign_key(
                ForeignKey::create()
                    .from(Self::Table, Self::NodeInstanceId)
                    .to(NodeInstance::Table, NodeInstance::Id),
            )
            .to_owned()
    }
    fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).if_exists().to_owned()
    }
}
