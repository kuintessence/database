//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "file_storage")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub storage_server_id: Uuid,
    pub file_metadata_id: Uuid,
    pub server_url: String,
    pub created_time: DateTimeWithTimeZone,
    pub created_user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file_metadata::Entity",
        from = "Column::FileMetadataId",
        to = "super::file_metadata::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    FileMetadata,
    #[sea_orm(
        belongs_to = "super::storage_server::Entity",
        from = "Column::StorageServerId",
        to = "super::storage_server::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    StorageServer,
}

impl Related<super::file_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FileMetadata.def()
    }
}

impl Related<super::storage_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StorageServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
